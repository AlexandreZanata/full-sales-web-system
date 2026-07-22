#!/usr/bin/env bash
# Seller Play preflight — G1–G5 (+ secrets). Must be green before Play upload.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPO_ROOT="$(cd "${ROOT}/../.." && pwd)"
cd "${ROOT}"

echo "==> verify-no-android-secrets"
bash "${REPO_ROOT}/scripts/verify-no-android-secrets.sh"

ensure_signing() {
  if [[ -f "${ROOT}/keystore.properties" ]]; then
    echo "==> using existing keystore.properties"
    return
  fi
  echo "==> generating ephemeral upload keystore for preflight (not for Play)"
  local jks="${ROOT}/.preflight-upload.jks"
  rm -f "${jks}" "${ROOT}/keystore.properties"
  keytool -genkeypair -v \
    -keystore "${jks}" \
    -alias upload \
    -keyalg RSA -keysize 2048 -validity 10000 \
    -storepass preflight -keypass preflight \
    -dname "CN=Seller Preflight, OU=Dev, O=FullSales, L=Local, ST=NA, C=BR" \
    >/dev/null
  cat > "${ROOT}/keystore.properties" <<EOF
storeFile=.preflight-upload.jks
storePassword=preflight
keyAlias=upload
keyPassword=preflight
EOF
  PREFLIGHT_EPHEMERAL=1
}

cleanup_ephemeral() {
  if [[ "${PREFLIGHT_EPHEMERAL:-0}" == "1" ]]; then
    rm -f "${ROOT}/.preflight-upload.jks" "${ROOT}/keystore.properties"
  fi
}
trap cleanup_ephemeral EXIT

PREFLIGHT_EPHEMERAL=0
ensure_signing

echo "==> :shared:check"
./gradlew :shared:check --quiet

echo "==> :androidApp:lint"
./gradlew :androidApp:lint --quiet

echo "==> release compile"
./gradlew :composeApp:compileReleaseKotlinAndroid :shared:assembleRelease --quiet

echo "==> assert release BuildConfig HTTPS"
BUILD_CFG="$(find shared/build -path '*release*' -name 'BuildConfig.java' 2>/dev/null | head -n 1 || true)"
if [[ -z "${BUILD_CFG}" ]]; then
  BUILD_CFG="$(find shared/build -path '*release*' -name 'BuildConfig.kt' 2>/dev/null | head -n 1 || true)"
fi
if [[ -n "${BUILD_CFG}" ]]; then
  if ! grep -q 'https://' "${BUILD_CFG}"; then
    echo "REFUSE: release BuildConfig API URL is not HTTPS (${BUILD_CFG})" >&2
    exit 1
  fi
  if grep -qE '10\.0\.2\.2|127\.0\.0\.1|localhost' "${BUILD_CFG}"; then
    echo "REFUSE: release BuildConfig still points at emulator/LAN (${BUILD_CFG})" >&2
    exit 1
  fi
  echo "    OK ${BUILD_CFG}"
else
  echo "WARN: could not locate release BuildConfig; relying on unit contract + bundleRelease"
fi

echo "==> :androidApp:bundleRelease"
./gradlew :androidApp:bundleRelease --quiet

AAB="$(find androidApp/build/outputs/bundle -name '*.aab' | head -n 1 || true)"
[[ -n "${AAB}" ]] || { echo "REFUSE: no AAB produced" >&2; exit 1; }
echo "play-preflight: OK (${AAB})"

#!/usr/bin/env bash
# Create production/env/*.env from examples with random secrets (idempotent keep).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ENV_DIR="${ROOT}/production/env"

random_hex() { openssl rand -hex "$1"; }
random_b64() { openssl rand -base64 "$1" | tr -d '\n/+='; }

copy_if_missing() {
  local src="$1" dst="$2"
  if [[ -f "${dst}" ]]; then
    echo "generate-secrets.sh: keep ${dst}"
    return
  fi
  cp "${src}" "${dst}"
  echo "generate-secrets.sh: created ${dst}"
}

replace_once() {
  local file="$1" needle="$2" value="$3"
  if grep -qF "${needle}" "${file}"; then
    sed -i "s|${needle}|${value}|g" "${file}"
  fi
}

cd "${ROOT}"
mkdir -p "${ENV_DIR}"
copy_if_missing "${ENV_DIR}/docker.env.example" "${ENV_DIR}/docker.env"
copy_if_missing "${ENV_DIR}/api.env.example" "${ENV_DIR}/api.env"
copy_if_missing "${ENV_DIR}/portal.env.example" "${ENV_DIR}/portal.env"
copy_if_missing "${ENV_DIR}/admin.env.example" "${ENV_DIR}/admin.env"
copy_if_missing "${ENV_DIR}/platform-admin.env.example" "${ENV_DIR}/platform-admin.env"
copy_if_missing "${ENV_DIR}/field.env.example" "${ENV_DIR}/field.env"
copy_if_missing "${ROOT}/production/vps.env.example" "${ROOT}/production/vps.env"

DB_PASS="$(random_hex 16)"
APP_PASS="$(random_hex 16)"
MINIO_PASS="$(random_hex 16)"
JWT="$(random_b64 48)"
REPORT_KEY="$(random_hex 32)"

replace_once "${ENV_DIR}/docker.env" "POSTGRES_PASSWORD=CHANGE_ME" "POSTGRES_PASSWORD=${DB_PASS}"
replace_once "${ENV_DIR}/docker.env" "APP_USER_PASSWORD=CHANGE_ME" "APP_USER_PASSWORD=${APP_PASS}"
replace_once "${ENV_DIR}/docker.env" "MINIO_ROOT_PASSWORD=CHANGE_ME" "MINIO_ROOT_PASSWORD=${MINIO_PASS}"

replace_once "${ENV_DIR}/api.env" "postgres://fullsales:CHANGE_ME@" "postgres://fullsales:${DB_PASS}@"
replace_once "${ENV_DIR}/api.env" "postgres://app_user:CHANGE_ME@" "postgres://app_user:${APP_PASS}@"
replace_once "${ENV_DIR}/api.env" "JWT_SECRET=CHANGE_ME_MIN_32_RANDOM_BYTES" "JWT_SECRET=${JWT}"
replace_once "${ENV_DIR}/api.env" \
  "REPORT_SIGNING_KEY_HEX=CHANGE_ME_64_HEX_CHARS______________________________" \
  "REPORT_SIGNING_KEY_HEX=${REPORT_KEY}"
replace_once "${ENV_DIR}/api.env" "STORAGE_SECRET_KEY=CHANGE_ME" "STORAGE_SECRET_KEY=${MINIO_PASS}"

chmod 600 "${ENV_DIR}/"*.env "${ROOT}/production/vps.env" 2>/dev/null || true
echo "generate-secrets.sh: done — run prepare-env.sh / deploy-to-vps.sh --env-only for URLs"

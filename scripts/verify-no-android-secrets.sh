#!/usr/bin/env bash
# Fail if Android signing / Play secrets would be tracked (open-repo guard).
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT}"
bad=0

refuse() {
  echo "REFUSE: $1" >&2
  bad=1
}

patterns=(
  '*.jks'
  '*.keystore'
  '**/keystore.properties'
  '**/play-service-account.json'
  '**/upload-keystore.jks'
)

while IFS= read -r f; do
  [[ -z "$f" ]] && continue
  refuse "Android secret path tracked/staged: $f"
done < <(
  {
    git ls-files -- "${patterns[@]}"
    git diff --cached --name-only -- "${patterns[@]}"
  } | sort -u
)

while IFS= read -r f; do
  [[ -z "$f" ]] && continue
  refuse "unignored Android secret candidate: $f"
done < <(git ls-files --others --exclude-standard -- "${patterns[@]}")

[[ "$bad" -eq 0 ]] || exit 1
echo "verify-no-android-secrets.sh: OK"

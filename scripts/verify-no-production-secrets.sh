#!/usr/bin/env bash
# Fail if production secrets would be tracked (open-repo guard).
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT}"
bad=0

refuse() {
  echo "REFUSE: $1" >&2
  bad=1
}

# Tracked or staged secret paths
while IFS= read -r f; do
  [[ -z "$f" ]] && continue
  case "$f" in
    production/env/*.env|production/vps.env|production/ssh/id_*)
      refuse "secret path tracked/staged: $f"
      ;;
  esac
done < <(
  git ls-files -- 'production/**'
  git diff --cached --name-only -- 'production/**'
)

# Unignored untracked secrets (gitignore broken)
while IFS= read -r f; do
  [[ -z "$f" ]] && continue
  refuse "unignored secret candidate: $f"
done < <(git ls-files --others --exclude-standard -- \
  'production/env/*.env' 'production/vps.env' 'production/ssh/id_*')

# Private key material in deploy-related trees
while IFS= read -r f; do
  [[ -z "$f" || ! -f "$f" ]] && continue
  if grep -qE 'BEGIN (OPENSSH|RSA|EC) PRIVATE KEY' "$f" 2>/dev/null; then
    refuse "private key material in $f"
  fi
done < <(
  git ls-files -- production infra docs .github scripts
  git ls-files --others --exclude-standard -- production infra docs .github scripts
)

[[ "$bad" -eq 0 ]] || exit 1
echo "verify-no-production-secrets.sh: OK"

#!/usr/bin/env bash
# Full remote deploy after rsync — run on VPS.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "${ROOT}"

if [[ -f "${ROOT}/production/vps.env" ]]; then
  # shellcheck disable=SC1090
  source "${ROOT}/production/vps.env"
fi

echo "==> Bootstrap checks"
"${ROOT}/infra/scripts/bootstrap-vps.sh"

echo "==> Prepare env on server (ports/URLs)"
"${ROOT}/infra/scripts/prepare-env.sh"

echo "==> Deploy app"
export SKIP_GIT_PULL=1
"${ROOT}/infra/scripts/deploy.sh"

echo "==> Nginx"
if [[ "${VPS_USE_HTTPS:-0}" == "1" ]]; then
  "${ROOT}/infra/scripts/install-nginx-domain.sh"
else
  "${ROOT}/infra/scripts/install-nginx-ip.sh"
fi

PUBLIC_HOST="${DOMAIN:-127.0.0.1}"
curl -sS -o /dev/null -w "Local portal via nginx: HTTP %{http_code}\n" \
  "http://127.0.0.1:${NGINX_HTTP_PORT:-80}/" || true

echo "vps-full-deploy.sh: done — check ${PUBLIC_HOST}"

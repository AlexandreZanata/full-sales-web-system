#!/usr/bin/env bash
# Build SPAs + rebuild/restart API compose stack.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "${ROOT}"

if [[ -f "${ROOT}/production/env/api.env" ]]; then
  "${ROOT}/infra/scripts/install-env.sh"
fi

if [[ "${SKIP_GIT_PULL:-}" == "1" ]] || [[ ! -d "${ROOT}/.git" ]]; then
  echo "deploy.sh: skipping git pull"
else
  git pull --ff-only origin main
fi

"${ROOT}/infra/scripts/ensure-node.sh"
export HUSKY=0 CI=true
NODE_ENV=development pnpm install --frozen-lockfile

echo "==> Building SPAs"
NODE_ENV=production pnpm --filter @full-sales/portal --filter @full-sales/admin --filter @full-sales/platform-admin --filter @full-sales/field build

echo "==> Data layer + API"
"${ROOT}/infra/scripts/up-data-layer.sh"

API_PORT=8108
if [[ -f "${ROOT}/production/vps.env" ]]; then
  # shellcheck disable=SC1090
  source "${ROOT}/production/vps.env"
  API_PORT="${API_HOST_PORT:-8108}"
fi

echo "==> Health"
for i in 1 2 3 4 5 6 7 8 9 10; do
  if curl -fsS "http://127.0.0.1:${API_PORT}/health" >/dev/null; then
    echo "deploy.sh: API healthy on :${API_PORT}"
    exit 0
  fi
  sleep 3
done
echo "deploy.sh: API health check failed" >&2
exit 1

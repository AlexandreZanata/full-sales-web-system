#!/usr/bin/env bash
# One-command manual deploy: env + rsync + remote build (password once, like sorrimobi).
# Usage:
#   ./production/deploy-to-vps.sh
#   ./production/deploy-to-vps.sh --env-only
#   VPS_USE_PASSWORD=1 ./production/deploy-to-vps.sh   # default: ask password once
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VPS_ENV="${ROOT}/production/vps.env"
MODE=all
for arg in "$@"; do
  case "${arg}" in
    --env-only) MODE=env-only ;;
    *)
      echo "deploy-to-vps.sh: unknown option ${arg}" >&2
      exit 1
      ;;
  esac
done

if [[ ! -f "${VPS_ENV}" ]]; then
  cp "${ROOT}/production/vps.env.domain.example" "${VPS_ENV}"
  echo "deploy-to-vps.sh: created production/vps.env — set VPS_HOST then re-run" >&2
  exit 1
fi

# shellcheck disable=SC1090
source "${VPS_ENV}"
: "${VPS_HOST:?set VPS_HOST}"
: "${VPS_USER:?set VPS_USER}"
VPS_APP_DIR="${VPS_APP_DIR:-/var/www/fullsales}"
VPS_PORT="${VPS_PORT:-22}"
VPS_USE_PASSWORD="${VPS_USE_PASSWORD:-1}"

if [[ "${VPS_HOST}" == "YOUR_VPS_IP" ]]; then
  echo "deploy-to-vps.sh: set real VPS_HOST in production/vps.env" >&2
  exit 1
fi

"${ROOT}/infra/scripts/generate-secrets.sh"
"${ROOT}/infra/scripts/prepare-env.sh"
[[ "${MODE}" == "env-only" ]] && exit 0

echo "==> SSH probe ${VPS_USER}@${VPS_HOST}:${VPS_PORT} (15s timeout)"
if ! nc -zv -w 5 "${VPS_HOST}" "${VPS_PORT}" >/dev/null 2>&1; then
  echo "deploy-to-vps.sh: cannot reach ${VPS_HOST}:${VPS_PORT}" >&2
  echo "  Open Hostinger firewall TCP ${VPS_PORT} for your IP, then retry." >&2
  exit 1
fi

# shellcheck disable=SC1091
source "${ROOT}/infra/scripts/vps-ssh-common.sh"
export VPS_USE_PASSWORD
vps_ssh_begin "${ROOT}"

echo "==> Ensure remote dir"
vps_run_ssh "${VPS_REMOTE}" "mkdir -p '${VPS_APP_DIR}/production/env'"

echo "==> Rsync to ${VPS_REMOTE}:${VPS_APP_DIR}"
rsync -avz --delete \
  -e "${VPS_RSYNC_SSH}" \
  --exclude node_modules \
  --exclude .pnpm-store \
  --exclude dist \
  --exclude target \
  --exclude .git \
  --exclude .local \
  --exclude test-results \
  --exclude coverage \
  --exclude 'production/ssh/id_*' \
  --exclude apps-mobile \
  "${ROOT}/" "${VPS_REMOTE}:${VPS_APP_DIR}/"

echo "==> Remote full deploy"
vps_run_ssh "${VPS_REMOTE}" bash -s <<REMOTE
set -euo pipefail
cd "${VPS_APP_DIR}"
chmod +x infra/scripts/*.sh production/*.sh infra/postgres/*.sh 2>/dev/null || true
./infra/scripts/vps-full-deploy.sh
REMOTE

echo "deploy-to-vps.sh: done"

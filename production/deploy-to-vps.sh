#!/usr/bin/env bash
# Manual deploy from laptop: env + rsync + remote vps-full-deploy.
# Usage:
#   ./production/deploy-to-vps.sh
#   ./production/deploy-to-vps.sh --env-only
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
  echo "deploy-to-vps.sh: created production/vps.env — review then re-run" >&2
  exit 1
fi

# shellcheck disable=SC1090
source "${VPS_ENV}"
: "${VPS_HOST:?set VPS_HOST}"
: "${VPS_USER:?set VPS_USER}"
VPS_APP_DIR="${VPS_APP_DIR:-/var/www/fullsales}"

if [[ "${VPS_HOST}" == "YOUR_VPS_IP" ]]; then
  echo "deploy-to-vps.sh: set real VPS_HOST in production/vps.env" >&2
  exit 1
fi

"${ROOT}/infra/scripts/generate-secrets.sh"
"${ROOT}/infra/scripts/prepare-env.sh"
[[ "${MODE}" == "env-only" ]] && exit 0

# shellcheck disable=SC1091
source "${ROOT}/infra/scripts/vps-ssh-common.sh"
vps_ssh_begin "${ROOT}"

echo "==> Ensure remote dir"
ssh "${VPS_SSH_OPTS[@]}" "${VPS_REMOTE}" "mkdir -p '${VPS_APP_DIR}/production/env'"

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
ssh "${VPS_SSH_OPTS[@]}" "${VPS_REMOTE}" bash -s <<REMOTE
set -euo pipefail
cd "${VPS_APP_DIR}"
chmod +x infra/scripts/*.sh production/*.sh infra/postgres/*.sh 2>/dev/null || true
./infra/scripts/vps-full-deploy.sh
REMOTE

echo "deploy-to-vps.sh: done"

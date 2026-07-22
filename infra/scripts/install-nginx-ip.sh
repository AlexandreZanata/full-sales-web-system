#!/usr/bin/env bash
# Enable IP-mode nginx vhost (portal + /v1). Does not touch other sites.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VPS_ENV="${ROOT}/production/vps.env"
if [[ -f "${VPS_ENV}" ]]; then
  # shellcheck disable=SC1090
  source "${VPS_ENV}"
fi

VPS_IP="${DOMAIN:-${VPS_HOST:?}}"
NGINX_HTTP_PORT="${NGINX_HTTP_PORT:-80}"
APP_DIR="${VPS_APP_DIR:-/var/www/fullsales}"
API_PORT="${API_HOST_PORT:-8108}"
src="${APP_DIR}/infra/nginx/fullsales-ip.conf"
dst="/etc/nginx/sites-available/fullsales-ip.conf"

sed \
  -e "s/YOUR_VPS_IP/${VPS_IP}/g" \
  -e "s/NGINX_HTTP_PORT/${NGINX_HTTP_PORT}/g" \
  -e "s/API_PORT/${API_PORT}/g" \
  -e "s|APP_DIR|${APP_DIR}|g" \
  "${src}" > "${dst}"

ln -sf "${dst}" /etc/nginx/sites-enabled/fullsales-ip.conf
rm -f /etc/nginx/sites-enabled/fullsales.conf
nginx -t
systemctl reload nginx
echo "install-nginx-ip.sh: http://${VPS_IP}:${NGINX_HTTP_PORT}/"

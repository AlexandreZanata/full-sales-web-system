#!/usr/bin/env bash
# Enable domain nginx (bootstrap HTTP or HTTPS if certs exist).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VPS_ENV="${ROOT}/production/vps.env"
if [[ -f "${VPS_ENV}" ]]; then
  # shellcheck disable=SC1090
  source "${VPS_ENV}"
fi

DOMAIN="${DOMAIN:?set DOMAIN}"
APP_DIR="${VPS_APP_DIR:-/var/www/fullsales}"
API_PORT="${API_HOST_PORT:-8108}"
ADMIN_HOST="${ADMIN_HOST:-admin.${DOMAIN}}"
API_PUBLIC="${API_HOST:-api.${DOMAIN}}"
PLATFORM_HOST="${PLATFORM_HOST:-platform.${DOMAIN}}"
dst="/etc/nginx/sites-available/fullsales.conf"
cert_path="/etc/letsencrypt/live/${DOMAIN}/fullchain.pem"

if [[ "${DOMAIN}" =~ ^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "install-nginx-domain.sh: DOMAIN must be a hostname" >&2
  exit 1
fi

template="fullsales.domain-bootstrap.conf"
[[ -f "${cert_path}" ]] && template="fullsales.conf"
src="${APP_DIR}/infra/nginx/${template}"

sed \
  -e "s/DOMAIN_APEX/${DOMAIN}/g" \
  -e "s/DOMAIN_ADMIN/${ADMIN_HOST}/g" \
  -e "s/DOMAIN_API/${API_PUBLIC}/g" \
  -e "s/DOMAIN_PLATFORM/${PLATFORM_HOST}/g" \
  -e "s/API_PORT/${API_PORT}/g" \
  -e "s|APP_DIR|${APP_DIR}|g" \
  "${src}" > "${dst}"

mkdir -p "${APP_DIR}/infra/nginx/acme-webroot"
ln -sf "${dst}" /etc/nginx/sites-enabled/fullsales.conf
rm -f /etc/nginx/sites-enabled/fullsales-ip.conf
nginx -t
systemctl reload nginx

if [[ "${template}" == "fullsales.domain-bootstrap.conf" ]]; then
  echo "install-nginx-domain.sh: HTTP bootstrap — run certbot then re-run this script"
  echo "  certbot --nginx -d ${DOMAIN} -d ${ADMIN_HOST} -d ${API_PUBLIC} -d ${PLATFORM_HOST}"
else
  echo "install-nginx-domain.sh: HTTPS enabled for ${DOMAIN}"
fi

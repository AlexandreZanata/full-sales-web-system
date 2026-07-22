#!/usr/bin/env bash
# Fill production/env URLs and ports from production/vps.env
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VPS_ENV="${ROOT}/production/vps.env"
ENV_DIR="${ROOT}/production/env"

if [[ ! -f "${VPS_ENV}" ]]; then
  echo "prepare-env.sh: missing ${VPS_ENV}" >&2
  exit 1
fi
# shellcheck disable=SC1090
source "${VPS_ENV}"

: "${VPS_HOST:?}"
DOMAIN="${DOMAIN:-${VPS_HOST}}"
VPS_USE_HTTPS="${VPS_USE_HTTPS:-0}"
ADMIN_HOST="${ADMIN_HOST:-admin.${DOMAIN}}"
API_PUBLIC_HOST="${API_HOST:-api.${DOMAIN}}"
PLATFORM_HOST="${PLATFORM_HOST:-platform.${DOMAIN}}"

if [[ "${VPS_USE_HTTPS}" == "1" ]]; then
  PORTAL_ORIGIN="https://${DOMAIN}"
  SCHEME=https
else
  PORTAL_ORIGIN="http://${DOMAIN}"
  SCHEME=http
  ADMIN_HOST="${DOMAIN}"
  API_PUBLIC_HOST="${DOMAIN}"
  PLATFORM_HOST="${DOMAIN}"
fi

if [[ ! -f "${ENV_DIR}/docker.env" ]]; then
  "${ROOT}/infra/scripts/generate-secrets.sh"
fi

# shellcheck disable=SC1090
source "${ENV_DIR}/docker.env"

POSTGRES_HOST_PORT="${POSTGRES_HOST_PORT:-5435}"
REDIS_HOST_PORT="${REDIS_HOST_PORT:-6381}"
MINIO_API_HOST_PORT="${MINIO_API_HOST_PORT:-9010}"
MINIO_CONSOLE_HOST_PORT="${MINIO_CONSOLE_HOST_PORT:-9011}"
API_HOST_PORT="${API_HOST_PORT:-8108}"

# Keep secrets; rewrite ports in docker.env
grep -vE '^(POSTGRES_HOST_PORT|REDIS_HOST_PORT|MINIO_API_HOST_PORT|MINIO_CONSOLE_HOST_PORT|API_HOST_PORT)=' \
  "${ENV_DIR}/docker.env" > "${ENV_DIR}/docker.env.tmp" || true
mv "${ENV_DIR}/docker.env.tmp" "${ENV_DIR}/docker.env"
{
  echo "POSTGRES_HOST_PORT=${POSTGRES_HOST_PORT}"
  echo "REDIS_HOST_PORT=${REDIS_HOST_PORT}"
  echo "MINIO_API_HOST_PORT=${MINIO_API_HOST_PORT}"
  echo "MINIO_CONSOLE_HOST_PORT=${MINIO_CONSOLE_HOST_PORT}"
  echo "API_HOST_PORT=${API_HOST_PORT}"
} >> "${ENV_DIR}/docker.env"

# Preserve secrets in api.env; refresh public origins + ensure docker DNS URLs
# shellcheck disable=SC1090
source "${ENV_DIR}/api.env"
DB_PASS="${POSTGRES_PASSWORD}"
APP_PASS="${APP_USER_PASSWORD}"
MINIO_PASS="${MINIO_ROOT_PASSWORD}"
JWT_SECRET="${JWT_SECRET:?}"
REPORT_SIGNING_KEY_HEX="${REPORT_SIGNING_KEY_HEX:?}"

cat > "${ENV_DIR}/api.env" <<EOF
RUST_LOG=api_http=info,tower_http=info
API_HOST=0.0.0.0
API_PORT=8080
DATABASE_ADMIN_URL=postgres://fullsales:${DB_PASS}@postgres:5432/${POSTGRES_DB:-fullsales_prod}
DATABASE_URL=postgres://app_user:${APP_PASS}@postgres:5432/${POSTGRES_DB:-fullsales_prod}
REDIS_URL=redis://redis:6379
JWT_SECRET=${JWT_SECRET}
REPORT_SIGNING_KEY_HEX=${REPORT_SIGNING_KEY_HEX}
STORAGE_ENDPOINT=http://minio:9000
STORAGE_ACCESS_KEY=${MINIO_ROOT_USER:-fullsales}
STORAGE_SECRET_KEY=${MINIO_PASS}
STORAGE_BUCKET=${MINIO_BUCKET:-media}
STORAGE_REGION=us-east-1
MEDIA_BUCKET=${MINIO_BUCKET:-media}
PORTAL_PUBLIC_ORIGIN=${PORTAL_ORIGIN}
PLATFORM_APEX_HOST=${DOMAIN}
EOF

# SPA same-origin /v1 via nginx
printf 'VITE_API_BASE_URL=/v1\n' > "${ENV_DIR}/portal.env"
printf 'VITE_API_BASE_URL=/v1\n' > "${ENV_DIR}/admin.env"
printf 'VITE_API_BASE_URL=/v1\n' > "${ENV_DIR}/platform-admin.env"

chmod 600 "${ENV_DIR}/"*.env
echo "prepare-env.sh: ${SCHEME} portal=${PORTAL_ORIGIN} api_host=${API_PUBLIC_HOST} admin=${ADMIN_HOST}"

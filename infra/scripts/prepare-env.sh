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
CATALOG_HOST="${CATALOG_HOST:-catalogo.comerc.app.br}"
VPS_USE_HTTPS="${VPS_USE_HTTPS:-0}"
API_PUBLIC_HOST="${API_HOST:-${DOMAIN}}"

if [[ "${VPS_USE_HTTPS}" == "1" ]]; then
  PORTAL_ORIGIN="https://${CATALOG_HOST}"
  APP_ORIGIN="https://${DOMAIN}"
  SCHEME=https
else
  PORTAL_ORIGIN="http://${CATALOG_HOST}"
  APP_ORIGIN="http://${DOMAIN}"
  SCHEME=http
  API_PUBLIC_HOST="${DOMAIN}"
fi

if [[ ! -f "${ENV_DIR}/docker.env" ]]; then
  "${ROOT}/infra/scripts/generate-secrets.sh"
fi

FS_POSTGRES_HOST_PORT="${POSTGRES_HOST_PORT:-5436}"
FS_REDIS_HOST_PORT="${REDIS_HOST_PORT:-6382}"
FS_MINIO_API_HOST_PORT="${MINIO_API_HOST_PORT:-9012}"
FS_MINIO_CONSOLE_HOST_PORT="${MINIO_CONSOLE_HOST_PORT:-9013}"
FS_API_HOST_PORT="${API_HOST_PORT:-8108}"

# shellcheck disable=SC1090
source "${ENV_DIR}/docker.env"

POSTGRES_HOST_PORT="${FS_POSTGRES_HOST_PORT}"
REDIS_HOST_PORT="${FS_REDIS_HOST_PORT}"
MINIO_API_HOST_PORT="${FS_MINIO_API_HOST_PORT}"
MINIO_CONSOLE_HOST_PORT="${FS_MINIO_CONSOLE_HOST_PORT}"
API_HOST_PORT="${FS_API_HOST_PORT}"

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
PLATFORM_APEX_HOST=${DOMAIN},${CATALOG_HOST}
EOF

printf 'VITE_API_BASE_URL=/v1\n' > "${ENV_DIR}/portal.env"
printf 'VITE_API_BASE_URL=/v1\nVITE_BASE=/admin/\n' > "${ENV_DIR}/admin.env"
printf 'VITE_API_BASE_URL=/v1\nVITE_BASE=/platform/\n' > "${ENV_DIR}/platform-admin.env"

chmod 600 "${ENV_DIR}/"*.env
echo "prepare-env.sh: ${SCHEME} portal=${PORTAL_ORIGIN} app=${APP_ORIGIN} api=${API_PUBLIC_HOST}"

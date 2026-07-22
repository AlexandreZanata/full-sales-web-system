#!/usr/bin/env bash
# Start Postgres + Redis + MinIO + API on VPS.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ENV_FILE="${ROOT}/production/env/docker.env"
COMPOSE_FILE="${ROOT}/infra/docker-compose.prod.yml"
API_ENV="${ROOT}/production/env/api.env"

cd "${ROOT}"
[[ -f "${ENV_FILE}" ]] || { echo "up-data-layer.sh: missing ${ENV_FILE}" >&2; exit 1; }
[[ -f "${API_ENV}" ]] || { echo "up-data-layer.sh: missing ${API_ENV}" >&2; exit 1; }

# shellcheck disable=SC1090
source "${ENV_FILE}"
: "${POSTGRES_PASSWORD:?}"
: "${APP_USER_PASSWORD:?}"
: "${MINIO_ROOT_PASSWORD:?}"

chmod +x "${ROOT}/infra/postgres/init-app-user.sh"

PROJECT="${COMPOSE_PROJECT_NAME:-fullsales}"
docker compose -p "${PROJECT}" -f "${COMPOSE_FILE}" --env-file "${ENV_FILE}" up -d --remove-orphans postgres redis minio
docker compose -p "${PROJECT}" -f "${COMPOSE_FILE}" --env-file "${ENV_FILE}" up -d --remove-orphans minio-init
docker compose -p "${PROJECT}" -f "${COMPOSE_FILE}" --env-file "${ENV_FILE}" up -d --build --remove-orphans api
docker compose -p "${PROJECT}" -f "${COMPOSE_FILE}" --env-file "${ENV_FILE}" ps

echo "up-data-layer.sh: stack running (project ${PROJECT})"

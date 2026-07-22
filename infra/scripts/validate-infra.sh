#!/usr/bin/env bash
# Local contract checks for Phase 20 infra (no live VPS required).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "${ROOT}"
fail=0

need() {
  if [[ ! -f "$1" ]]; then
    echo "MISSING $1" >&2
    fail=1
  fi
}

need infra/docker-compose.prod.yml
need infra/nginx/fullsales.conf
need infra/nginx/fullsales-ip.conf
need infra/nginx/fullsales.domain-bootstrap.conf
need infra/postgres/init-app-user.sh
need production/vps.env.example
need production/vps.env.domain.example
need production/env/api.env.example
need production/deploy-to-vps.sh
need docs/deployment/vps-shared-host.md
need docs/runbooks/deploy-vps.md
need docs/runbooks/rollback-vps.md
need .github/workflows/vps-deploy.yml

for s in generate-secrets prepare-env up-data-layer install-env \
  install-nginx-ip install-nginx-domain bootstrap-vps ensure-node \
  deploy vps-full-deploy diagnose-nginx-vhosts validate-infra; do
  need "infra/scripts/${s}.sh"
  chmod +x "infra/scripts/${s}.sh"
done
chmod +x production/deploy-to-vps.sh infra/postgres/init-app-user.sh
[[ "${fail}" -eq 0 ]] || exit 1

# Env generation (writes gitignored production/env — safe)
if [[ ! -f production/vps.env ]]; then
  cp production/vps.env.domain.example production/vps.env
fi
bash infra/scripts/generate-secrets.sh
bash infra/scripts/prepare-env.sh
grep -q 'postgres://fullsales:' production/env/api.env
grep -q 'PORTAL_PUBLIC_ORIGIN=https://vendas.comerc.app.br' production/env/api.env
if grep -q 'CHANGE_ME' production/env/api.env production/env/docker.env; then
  echo "CHANGE_ME still present in env files" >&2
  exit 1
fi
grep -q 'VITE_API_BASE_URL=/v1' production/env/portal.env

if command -v docker >/dev/null 2>&1; then
  docker compose -f infra/docker-compose.prod.yml \
    --env-file production/env/docker.env config >/dev/null
  echo "validate-infra.sh: compose config OK"
else
  echo "validate-infra.sh: skip compose (no docker)"
fi

sed \
  -e 's/DOMAIN_APEX/vendas.comerc.app.br/g' \
  -e 's/DOMAIN_ADMIN/admin.vendas.comerc.app.br/g' \
  -e 's/DOMAIN_API/api.vendas.comerc.app.br/g' \
  -e 's/DOMAIN_PLATFORM/platform.vendas.comerc.app.br/g' \
  -e 's/API_PORT/8108/g' \
  -e 's|APP_DIR|/var/www/fullsales|g' \
  infra/nginx/fullsales.domain-bootstrap.conf > /tmp/fullsales-nginx-test.conf
grep -q 'server_name api.vendas.comerc.app.br' /tmp/fullsales-nginx-test.conf
rm -f /tmp/fullsales-nginx-test.conf

# Line-count guard for scripts/nginx (soft warning)
while IFS= read -r f; do
  lines="$(wc -l < "${f}")"
  if [[ "${lines}" -gt 200 ]]; then
    echo "OVER 200 lines: ${f} (${lines})" >&2
    fail=1
  fi
done < <(find infra/scripts infra/nginx production/deploy-to-vps.sh -type f 2>/dev/null)

[[ "${fail}" -eq 0 ]] || exit 1
bash "${ROOT}/scripts/verify-no-production-secrets.sh"
echo "validate-infra.sh: all checks passed"

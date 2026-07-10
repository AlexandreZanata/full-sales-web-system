#!/usr/bin/env bash
# Reset dev catalog to seeded demo state (removes user-created products, re-syncs assets).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../.." && pwd)"
cd "$ROOT"

if [[ -f backend/.env ]]; then
  set -a
  # shellcheck disable=SC1091
  source backend/.env
  set +a
fi

ADMIN_URL="${DATABASE_ADMIN_URL:-postgres://print3d:print3d@localhost:5435/full_sales_dev}"
TENANT_ID="01900001-0000-7000-8000-000000000001"
PG_CONTAINER="${FULL_SALES_PG_CONTAINER:-full-sales-postgres}"

run_psql() {
  if psql "$ADMIN_URL" -c "SELECT 1" >/dev/null 2>&1; then
    psql "$ADMIN_URL" "$@"
    return
  fi
  if docker ps --format '{{.Names}}' | grep -qx "$PG_CONTAINER"; then
    docker exec -i "$PG_CONTAINER" psql -U print3d -d full_sales_dev "$@"
    return
  fi
  echo "Cannot connect to Postgres (psql or $PG_CONTAINER)." >&2
  exit 1
}

echo "Removing non-seed catalog rows for dev tenant..."
run_psql -v ON_ERROR_STOP=1 <<SQL
DELETE FROM inventory.product_images pi
USING inventory.products p
WHERE pi.product_id = p.id
  AND p.tenant_id = '$TENANT_ID'::uuid
  AND p.sku NOT LIKE 'SEED-%';

DELETE FROM inventory.products
WHERE tenant_id = '$TENANT_ID'::uuid
  AND sku NOT LIKE 'SEED-%';
SQL

echo "Syncing FoodKing demo assets..."
"$(dirname "$0")/sync-foodking-assets.sh"

echo "Re-applying dev seed (catalog + portal + site logo backfill)..."
pnpm seed:dev

echo "Dev catalog reset complete."

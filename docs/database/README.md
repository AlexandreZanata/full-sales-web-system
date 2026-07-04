# Database modules

> PostgreSQL bounded contexts — one schema per module. Authoritative specs live in `.local/phases/01b-database-modularization/modules/`.

---

## Schema map

| Module | Schema | Tables |
|--------|--------|--------|
| 00-shared | `shared` | `tenants` |
| 01-identity | `identity` | `users` |
| 02-commerces | `commerces` | `commerces` |
| 03-inventory | `inventory` | `products`, `stock_movements`, `stock_balances` |
| 04-sales | `sales` | `sales`, `sale_items` |
| 05-reports | `reports` | `signing_keys`, `reports` |

---

## Migrations

Ordered sqlx files in `backend/migrations/` — prefix `YYYYMMDDHHMMSS_<module>_<name>.sql`.

Run: `cd backend && sqlx migrate run`

---

## RLS

All tenant-scoped tables use `app.tenant_id` session variable. Application role: `app_user` (see migration `20260704121000_shared_app_role.sql`).

Standard policy pattern: `.local/phases/01b-database-modularization/documentation/RLS-POLICY-STANDARD.md`

Integration tests: `cargo test -p infra-postgres --test integration rls_`

---

## Spec locations

| Document | Path |
|----------|------|
| Module map | `.local/phases/01b-database-modularization/documentation/MODULE-MAP.md` |
| Cross-module refs | `.local/phases/01b-database-modularization/documentation/CROSS-MODULE-INTERACTIONS.md` |
| Entity specs | `.local/phases/01b-database-modularization/modules/*/ENTITY-SPEC-*.md` |
| Migration specs | `.local/phases/01b-database-modularization/modules/*/migrations/MIGRATION-SPEC-*.md` |

Promote finalized specs here when stable (`docs/database/modules/NN-name/`).

---

## Key decisions

| ADR | Topic |
|-----|-------|
| [ADR-001](../adr/ADR-001-stock-balance-materialized.md) | Materialized `stock_balances` |
| [ADR-002](../adr/ADR-002-tenant-platform-org.md) | Tenant = platform org |
| [ADR-005](../adr/ADR-005-inventory-driver-scope.md) | Driver-scoped inventory |

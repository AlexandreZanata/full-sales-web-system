# Database modules

> PostgreSQL bounded contexts — one schema per module. Authoritative specs live in `.local/phases/01b-database-modularization/modules/` and `.local/phases/01c-database-hardening/modules/`.

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
| 06-audit | `audit` | `events` |

---

## Phase 1c additions (2026-07-04)

| Change | Migration |
|--------|-----------|
| Drop `shared.tenant_scoped_placeholder` | `20260704121100` |
| Append-only triggers (`stock_movements`, `reports`, `audit.events`) | `20260704121200`, `20260704121300`, `20260704122000` |
| `shared.set_updated_at()` on mutable entities | `20260704121400`, `20260704121600` |
| `stock_movements.reason` | `20260704121500` |
| Sales lifecycle (`cancelled_at`, `updated_at`) | `20260704121600` |
| List indexes (sales, movements reference, signing keys active) | `20260704121700`–`20260704121900` |
| Audit module | `20260704122000` |

---

## Migrations

Ordered sqlx files in `backend/migrations/` — prefix `YYYYMMDDHHMMSS_<module>_<name>.sql`.

Run: `cd backend && sqlx migrate run`

---

## RLS

All tenant-scoped tables use `app.tenant_id` session variable. Application role: `app_user` (see migration `20260704121000_shared_app_role.sql`).

Standard policy pattern: `.local/phases/01b-database-modularization/documentation/RLS-POLICY-STANDARD.md`

Integration tests: `cargo test -p infra-postgres --test integration rls_` and `cargo test -p infra-postgres --test repo_phase1c`

---

## Spec locations

| Document | Path |
|----------|------|
| Module map | `.local/phases/01b-database-modularization/documentation/MODULE-MAP.md` |
| Cross-module refs | `.local/phases/01b-database-modularization/documentation/CROSS-MODULE-INTERACTIONS.md` |
| Entity specs | `.local/phases/01b-database-modularization/modules/*/ENTITY-SPEC-*.md` |
| Audit module (1c) | `.local/phases/01c-database-hardening/modules/06-audit/` |
| Migration specs | `modules/*/migrations/MIGRATION-SPEC-*.md` |

Promote finalized specs here when stable (`docs/database/modules/NN-name/`).

---

## Key decisions

| ADR | Topic |
|-----|-------|
| [ADR-001](../adr/ADR-001-stock-balance-materialized.md) | Materialized `stock_balances` |
| [ADR-002](../adr/ADR-002-tenant-platform-org.md) | Tenant = platform org |
| [ADR-005](../adr/ADR-005-inventory-driver-scope.md) | Driver-scoped inventory |
| [ADR-009](../adr/ADR-009-ephemeral-state-redis.md) | Redis for idempotency / refresh sessions; outbox deferred |

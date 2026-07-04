# Database modules

> PostgreSQL bounded contexts — one schema per module. **Implemented schema:** `backend/migrations/`. **Expansion specs:** `.local/phases/0d-domain-expansion/` and phases 07–15.

---

## Schema map

| Module | Schema | Tables |
|--------|--------|--------|
| 00-shared | `shared` | `tenants` |
| 01-identity | `identity` | `users`, `driver_profiles`, `seller_profiles` |
| 02-commerces | `commerces` | `commerces`, `commerce_addresses` |
| 03-inventory | `inventory` | `products`, `stock_movements`, `stock_balances`, `product_images`, `stock_reservations` |
| 04-sales | `sales` | `sales`, `sale_items` |
| 05-reports | `reports` | `signing_keys`, `reports` |
| 06-audit | `audit` | `events` |
| 07-media | `media` | `files` |
| 01-identity (delta) | `identity` | `users` (+ `commerce_id`, `profile_file_id`), `driver_profiles`, `seller_profiles` |

---

## Phase 07 additions (2026-07-04)

| Change | Migration |
|--------|-----------|
| `media` schema + `media.files` (metadata only; bytes in MinIO) | `20260704122200` |
| RLS tenant isolation on `media.files` | `20260704122200` |
| `app_user` GRANTs on `media` schema | `20260704122200` |

---

## Phase 10 additions (2026-07-04)

| Change | Migration |
|--------|-----------|
| `products.category`, `products.unit_of_measure` | `20260704122800` |
| `inventory.product_images` (one primary per product) | `20260704122900` |
| `inventory.stock_reservations` (RN2, ADR-010) | `20260704123000` |

### Available stock formula (DE-001 / ADR-010)

Tenant-level pool until driver is assigned at picking:

```text
available(tenant, product) =
  SUM(stock_balances.quantity WHERE tenant_id AND product_id)
  − SUM(stock_reservations.quantity_reserved WHERE status = 'Active' AND same tenant/product)
```

Field sales (`sales.order_id IS NULL`) continue to deduct directly on confirm — no reservation.

---

## Phase 09 additions (2026-07-04)

| Change | Migration |
|--------|-----------|
| `commerces.commerce_addresses` (Billing/Delivery, primary per type) | `20260704122600` |
| `commerces.commerces.logo_file_id` → `media.files` (app-validated) | `20260704122700` |

### JSON `address` deprecation path

Legacy `commerces.commerces.address` (JSONB) remains for backward compatibility until Phase 14 (commerce portal API). New address writes use `commerce_addresses`.

**One-time backfill (run before Phase 14 portal go-live):**

1. For each commerce row with non-empty JSON `address`, insert a `Delivery` row in `commerce_addresses` with `is_primary = true` (map JSON keys: `street`, `number`, `district`, `city`, `state`, `postalCode`).
2. Optionally insert a `Billing` row when JSON contains separate billing fields.
3. Portal and order APIs (Phases 11–14) read/write `commerce_addresses` only; do not extend the JSON column.
4. After all tenants are backfilled and portal is live, drop `address` JSONB in a future migration (not Phase 09).

---

## Phase 08 additions (2026-07-04)

| Change | Migration |
|--------|-----------|
| `CommerceContact` role + `commerce_id` / `profile_file_id` on `identity.users` | `20260704122300` |
| `identity.driver_profiles` (CNH photo → `media.files`) | `20260704122400` |
| `identity.seller_profiles` | `20260704122500` |

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

Standard policy pattern: tenant isolation via `app.tenant_id` (see migrations + integration tests).

Integration tests: `cargo test -p infra-postgres --test integration rls_` and `cargo test -p infra-postgres --test repo_phase1c`

---

## Spec locations

| Document | Path |
|----------|------|
| Live DDL | `backend/migrations/` |
| Expansion module map | `.local/phases/0d-domain-expansion/documentation/MODULE-MAP-EXPANSION.md` |
| Expansion planning | `.local/phases/0d-domain-expansion/documentation/` |
| Pending entity specs | `.local/phases/07-media/` … `15-reports-settlement/` |

Phases 01b/01c local spec folders were removed after completion; historical specs remain in git history.

Promote finalized expansion specs to `docs/database/modules/NN-name/` when stable.

---

## Key decisions

| ADR | Topic |
|-----|-------|
| [ADR-001](../adr/ADR-001-stock-balance-materialized.md) | Materialized `stock_balances` |
| [ADR-002](../adr/ADR-002-tenant-platform-org.md) | Tenant = platform org |
| [ADR-005](../adr/ADR-005-inventory-driver-scope.md) | Driver-scoped inventory |
| [ADR-009](../adr/ADR-009-ephemeral-state-redis.md) | Redis for idempotency / refresh sessions; outbox deferred |
| [ADR-010](../adr/ADR-010-stock-reservation-tenant-pool.md) | Stock reservations — tenant pool until driver assigned |
| [ADR-011](../adr/ADR-011-object-storage-minio.md) | Self-hosted MinIO for media bytes |

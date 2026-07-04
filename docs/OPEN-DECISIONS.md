# Open Decisions

> Resolve each item as an ADR in `docs/adr/` when decided. Until then, agents **must ask** the product owner.

---

## Pending

| # | Question | Options | Impact |
|---|----------|---------|--------|
| — | *(none — all Phase 0 decisions resolved)* | — | — |

---

## ADR template

When deciding, create `docs/adr/NNN-short-title.md` using `agent-rules/11-documentation-and-glossary/adr-template.md`.

---

## Resolved

| ADR | Decision | Date |
|-----|----------|------|
| [ADR-001](adr/ADR-001-stock-balance-materialized.md) | Materialized Postgres projection + Redis invalidate-on-write | 2026-07-04 |
| [ADR-002](adr/ADR-002-tenant-platform-org.md) | `tenant_id` = platform owner org (many commerces per tenant) | 2026-07-04 |
| [ADR-003](adr/ADR-003-report-on-demand.md) | On-demand `POST /v1/reports`; empty period → signed zero report | 2026-07-04 |
| [ADR-004](adr/ADR-004-ed25519-key-rotation.md) | Ed25519 key rotation every 180 days; retain old public keys | 2026-07-04 |
| [ADR-005](adr/ADR-005-inventory-driver-scope.md) | Inventory scoped per driver (`stock:{driver_id}:{product_id}`) | 2026-07-04 |
| [ADR-006](adr/ADR-006-payment-method-enum.md) | `Cash`, `Pix`, `Credit`, `Debit` | 2026-07-04 |
| [ADR-007](adr/ADR-007-public-report-verify.md) | `GET /v1/reports/{id}/verify` is public (rate limited) | 2026-07-04 |

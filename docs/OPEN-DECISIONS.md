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
| [ADR-010](adr/ADR-010-stock-reservation-tenant-pool.md) | Stock reservations — tenant pool until driver assigned (DE-001) | 2026-07-04 |
| [ADR-011](adr/ADR-011-object-storage-minio.md) | Self-hosted MinIO for object storage (DE-009) | 2026-07-04 |
| [ADR-013](adr/ADR-013-platform-admin-identity.md) | PlatformAdmin separate table; MFA required; impersonation audited | 2026-07-10 |
| [ADR-014](adr/ADR-014-asaas-platform-billing.md) | Single platform Asaas account; webhook token + idempotency | 2026-07-10 |
| [ADR-015](adr/ADR-015-tenant-lifecycle.md) | PlatformAdmin-only provision; 14-day trial; 90-day retention | 2026-07-10 |
| [ADR-016](adr/ADR-016-platform-admin-rls-bypass.md) | `app.bypass_rls` session flag for `/v1/platform/*` only | 2026-07-10 |
| [ADR-017](adr/ADR-017-custom-domain-verification.md) | DNS TXT verify; Caddy TLS; portal+admin only | 2026-07-10 |
| [ADR-018](adr/ADR-018-tenant-asaas-payments.md) | Pro+ tenant own Asaas key; no platform fee v1 | 2026-07-10 |

### Platform SaaS (0-OD-001 … 0-OD-020)

Full decision log: [.local/phases/0-platform-vision-decisions/documentation/OPEN-DECISIONS.md](../.local/phases/0-platform-vision-decisions/documentation/OPEN-DECISIONS.md)

### Domain expansion (DE-002…DE-008, DE-010)

Non-schema ADRs recorded in [.local/phases/0d-domain-expansion/documentation/OPEN-DECISIONS-EXPANSION.md](../.local/phases/0d-domain-expansion/documentation/OPEN-DECISIONS-EXPANSION.md):

| ID | Decision |
|----|----------|
| DE-002 | Always admin approves seller/portal orders |
| DE-003 | Manual follow-up for partial delivery |
| DE-004 | Order : Delivery 1:1 in v1 |
| DE-005 | Defer per-commerce price lists |
| DE-006 | Payment declaration optional forever |
| DE-007 | Single declarer per sale |
| DE-008 | Undeclared payment — report listing only |
| DE-010 | Keep `payment_method` as expected; declared fields separate |

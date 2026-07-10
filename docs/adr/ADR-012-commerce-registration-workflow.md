# ADR-012: Seller commerce registration with admin review

**Status:** Accepted  
**Date:** 2026-07-06  
**Deciders:** Phase 69 product sign-off (recommended options 69-OD-001 … 69-OD-010)

## Context

`POST /v1/commerces` is Admin-only (BR-IA-001). Field sellers need to register commerces on visit; back office must approve before the commerce is available for sales.

## Decision

1. **Submit vs approve privileges** — Replace single `can_register_commerce()` with:
   - `can_submit_commerce()` → `Seller` only
   - `can_review_commerce()` → `Admin` OR user flag `can_review_commerce` on `identity.users`
2. **Data model (69-OD-002 A)** — Extend `commerces.commerces` with `registration_status` (`Active` | `PendingReview` | `Rejected`), audit columns, optional `lookup_snapshot` JSONB. Pending rows have `active = true` (provisional catalog visibility); admin reject or deactivate sets `active = false`.
3. **Routes** — Explicit `/v1/commerces/registrations/*`; keep `POST /v1/commerces` for Admin direct create (auto-`Active`).
4. **CNPJ lookup (69-OD-001 A)** — Backend proxy to BrasilAPI with cache + rate limit; `GET /v1/commerces/cnpj-lookup?cnpj=`.
5. **Duplicates (69-OD-006 A)** — Reject submit when CNPJ already exists in tenant (`CNPJ_ALREADY_REGISTERED`).
6. **Edit while pending (69-OD-007 B)** — Seller may `PATCH` own `PendingReview` registration.
7. **Address on submit (69-OD-009 A)** — Normalized `Delivery` address required on submit.
8. **Lookup failure (69-OD-008 B)** — Client falls back to manual entry on `CNPJ_LOOKUP_UNAVAILABLE`.
9. **Offline (69-OD-010 A)** — Online-only for v1.

## Consequences

### Positive

- Clear workflow without breaking Admin direct-create path
- External API hidden behind backend; rate limits enforced
- Sellers see active commerces in sales picker (`filter[active]=true`), including `PendingReview` until admin rejects or deactivates

### Negative

- `commerces` table carries registration workflow columns
- Review privilege flag requires admin to set `can_review_commerce` on users (no dedicated BackOffice role in v1)

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| Separate `CommerceRegistration` aggregate (69-OD-002 B) | Extra joins; status on `Commerce` suffices for MVP |
| Auto-approve (69-OD-005 B/C) | Product requires manual review in v1 |
| Driver submit (69-OD-003 B) | Scope is seller field registration |

## Related

- [docs/BUSINESS-RULES.md](../BUSINESS-RULES.md) — BR-IA-001 (amended), BR-CO-010 … BR-CO-012
- [docs/API-CONTRACT.md](../API-CONTRACT.md) — registrations + cnpj-lookup sections

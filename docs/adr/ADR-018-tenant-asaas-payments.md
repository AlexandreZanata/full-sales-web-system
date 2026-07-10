# ADR-018: Tenant payment collection via Asaas

**Status:** Accepted  
**Date:** 2026-07-10  
**Deciders:** Phase 0 sign-off (0-OD-015 … 0-OD-017)

## Context

Beyond platform subscription billing, **Pro** and **Enterprise** tenants may collect online payments from their portal customers (orders). This is separate from the platform's single Asaas account in ADR-014.

## Decision

1. **Pro and Enterprise only** — `Starter` cannot enable tenant payment collection.
2. Tenant connects **their own Asaas API key**; stored encrypted (**AES-256-GCM**) in `billing.tenant_asaas_credentials`.
3. **No platform transaction fee** v1 — revenue is subscription-only.
4. Tenant Admin controls: enable/disable methods (PIX, credit, boleto toggles), auto-capture policy, refund initiation (Admin only v1).
5. Portal checkout creates charges on the **tenant's** Asaas account, not the platform account.
6. Settlement visibility is read-only v1 — no withdraw through platform UI.

## Consequences

### Positive

- Funds flow directly to tenant Asaas — no platform money-transmitter burden.
- Plan gating drives upgrades to Pro.
- Encrypted credentials meet secrets policy.

### Negative

- Two Asaas integration modes (platform key vs per-tenant key).
- Tenant must operate their own Asaas account and webhooks for order payments.

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| Cash/Pix declaration only v1 | Spec requires optional online payments for Pro+ |
| Platform subaccount per tenant | Asaas subaccount complexity; tenant key simpler |
| Platform % fee v1 | Scope creep; subscription model sufficient v1 |

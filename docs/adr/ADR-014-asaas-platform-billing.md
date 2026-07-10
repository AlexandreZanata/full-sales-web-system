# ADR-014: Asaas integration — platform billing

**Status:** Accepted  
**Date:** 2026-07-10  
**Deciders:** Phase 0 sign-off (0-OD-010 … 0-OD-014)

## Context

Tenants pay the platform via **Asaas** subscriptions. We must choose account topology, payment methods, dunning, and webhook security. Asaas documents **no HMAC body signature** — webhooks authenticate via a configured `authToken` sent in the `asaas-access-token` header.

## Decision

1. **Single platform Asaas account** v1 — one API key in env; each tenant is an Asaas `customer` with `externalReference` = `TenantId`.
2. Subscription billing: **PIX + credit card** v1; Boleto deferred.
3. **Monthly** billing cycle only v1.
4. **Dunning:** `PAYMENT_OVERDUE` → tenant `PastDue`; **7-day grace**; suspend on day 8 if still unpaid.
5. **Webhook security:** validate `asaas-access-token` with constant-time compare; persist events in `billing.payment_events` with **UNIQUE `asaas_event_id`** for idempotency; return 200 on duplicate.
6. Outbound calls: API key in `access_token` header; sandbox base `https://api-sandbox.asaas.com/v3`.

## Consequences

### Positive

- Matches Asaas documented webhook model — no invented HMAC.
- Idempotency table prevents replay double-processing.
- Single account simplifies platform finance reconciliation.

### Negative

- All tenant subscription revenue flows through one Asaas account.
- No annual discount until a later phase.

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| Subaccount per tenant (platform billing) | Operational overhead v1 |
| HMAC webhook verification | Not provided by Asaas |
| Immediate suspend on failed payment | Too harsh; 7-day grace adopted |

**Reference:** [ASAAS-INTEGRATION-MAP.md](../ASAAS-INTEGRATION-MAP.md)

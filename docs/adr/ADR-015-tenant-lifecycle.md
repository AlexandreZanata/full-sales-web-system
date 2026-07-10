# ADR-015: Tenant lifecycle state machine

**Status:** Accepted  
**Date:** 2026-07-10  
**Deciders:** Phase 0 sign-off (0-OD-006 … 0-OD-009)

## Context

Multi-tenant SaaS requires explicit tenant states for provisioning, trial, billing health, suspension, and offboarding. ADR-002 defined `tenant_id` semantics; this ADR extends the **tenant aggregate** with lifecycle.

## Decision

1. **Provisioning:** only **PlatformAdmin** creates tenants v1 (no self-serve signup).
2. **Trial:** **14 days** on `Starter` plan; `trial_ends_at` set at provision.
3. **Suspended behavior:** **hard block** all mutating tenant APIs; exceptions: tenant billing self-service (`GET/POST /v1/billing/*` read/payment-method) and inbound Asaas webhooks.
4. **Offboarding retention:** **90 days** in `Offboarding`, then PII anonymized and tenant `Deleted`; RLS returns zero rows.
5. States: `Provisioning` → `Trial` | `Active` → `PastDue` → `Suspended` → `Offboarding` → `Deleted` (see `STATE-MACHINES.md`).

## Consequences

### Positive

- Predictable dunning aligned with ADR-014 grace period.
- LGPD-friendly retention window before purge.
- Suspended tenants cannot mutate operational data but can fix billing.

### Negative

- Hard block may surprise tenants — billing UI must explain suspension reason.
- Self-serve signup deferred to Phase 14+.

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| Self-serve signup v1 | Fraud and approval burden; PlatformAdmin-only v1 |
| Read-only when suspended | Still exposes data mutations via side channels |
| 30-day retention | Shorter than LGPD export expectations |

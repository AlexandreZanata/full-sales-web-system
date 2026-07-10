# ADR-016: RLS bypass for PlatformAdmin

**Status:** Accepted  
**Date:** 2026-07-10  
**Deciders:** Phase 0 architecture review

## Context

PostgreSQL RLS isolates tenants via `app.tenant_id`. PlatformAdmin must list and mutate across tenants for support, fraud review, and provisioning — without weakening default tenant isolation for normal JWT sessions.

## Decision

1. Default: every tenant API request sets `SET LOCAL app.tenant_id = '<uuid>'` from JWT; RLS enforced.
2. PlatformAdmin cross-tenant reads/writes use an explicit middleware path that sets:
   - `SET LOCAL app.bypass_rls = 'true'` **only** when JWT role is `PlatformAdmin` **and** route is under `/v1/platform/*`.
3. Impersonation sets `app.tenant_id` to target tenant **without** bypass — PlatformAdmin acts inside tenant scope.
4. `app.bypass_rls` is never accepted from client headers or query params.
5. All bypass mutations emit `audit.events` (BR-AU-001).

## Consequences

### Positive

- Tenant sessions cannot escalate to cross-tenant access (BOLA mitigation).
- Impersonation remains tenant-scoped — realistic admin experience.
- Audit trail for elevated access.

### Negative

- Policies must check `current_setting('app.bypass_rls', true)` on every sensitive table.
- Risk if bypass flag leaks to tenant routes — middleware must be route-scoped.

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| Separate database connection without RLS | Two code paths; easy to misuse |
| PlatformAdmin always bypass | Impersonation and support views need tenant context |
| Schema-per-tenant | Rejected in ADR-002 |

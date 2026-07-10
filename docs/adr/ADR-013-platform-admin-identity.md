# ADR-013: PlatformAdmin identity model

**Status:** Accepted  
**Date:** 2026-07-10  
**Deciders:** Phase 0 sign-off (0-OD-001 … 0-OD-005)

## Context

The platform needs a **Super Admin** actor with cross-tenant powers distinct from tenant `Admin`. Storing platform operators in the same `identity.users` table with `tenant_id = NULL` would complicate RLS policies and blur authorization boundaries.

## Decision

1. Code and API use the term **`PlatformAdmin`** (not `SuperAdmin`).
2. Platform operators live in **`identity.platform_users`** — separate from tenant `identity.users`.
3. **MFA is required** before the first privileged session is issued.
4. **Impersonation** is allowed: PlatformAdmin may obtain a short-lived scoped JWT acting as a tenant `Admin`, with mandatory audit events and a visible UI banner.
5. Tenant **Admin** may create and manage other tenant users (including other Admins); only PlatformAdmin crosses tenants.

## Consequences

### Positive

- Clear separation: tenant RLS never applies to `platform_users` queries by default.
- MFA and impersonation audit satisfy elevated-privilege controls in `SECURITY.md`.
- Tenant self-service user management unchanged for operators.

### Negative

- Two user tables — login flows and password reset differ for PlatformAdmin vs tenant users.
- Impersonation tokens require careful scope and TTL enforcement.

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| `SuperAdmin` name | Collides conceptually with tenant `Admin`; glossary prefers `PlatformAdmin` |
| Flag on `identity.users` | RLS and join ambiguity; harder to enforce MFA policy |
| Forbid impersonation v1 | Support burden without audited impersonation |

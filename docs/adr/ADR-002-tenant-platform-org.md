# ADR-002: Tenant represents platform owner organization

**Status:** Accepted  
**Date:** 2026-07-04  
**Deciders:** Product spec (driver/seller README), Phase 0 sign-off

## Context

Multi-tenancy uses a single Postgres database with RLS on `tenant_id`. OD-002 asked whether `tenant_id` is (A) the platform operator org with many commerces, or (B) each commerce as its own tenant.

## Decision

**Option A:** `tenant_id` identifies the **platform owner organization** (the company operating the driver/seller system). **Commerces** are business clients registered within that tenant — not separate tenants. JWT carries `tenant_id`; middleware sets `app.tenant_id` for RLS.

## Consequences

### Positive

- Matches admin-managed commerce model (one operator, many stores)
- Simpler onboarding: new commerce ≠ new tenant provisioning
- Users (Admin, Driver, Seller) belong to the platform tenant

### Negative

- No hard isolation between commerces at RLS level — commerce scoping is application-layer
- Future white-label multi-operator SaaS would require revisiting this ADR

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| Each commerce is a tenant | Overkill for current B2B operator model; complicates user roles across commerces |
| Schema-per-tenant | Explicitly rejected in product spec for operational simplicity |

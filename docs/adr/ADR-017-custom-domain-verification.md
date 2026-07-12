# ADR-017: Custom domain verification and routing

**Status:** Accepted  
**Date:** 2026-07-10  
**Deciders:** Phase 0 sign-off (0-OD-018 … 0-OD-020)

## Context

Tenants on **Pro** and **Enterprise** plans may serve portal and admin UIs on their own hostname. We need verification, TLS, and tenant resolution without moving the API off the platform host.

## Decision

1. **Verification:** DNS **TXT** record at `_fullsales-verify.<domain>` with a random challenge token.
2. **Scope:** verified domain serves **portal + admin** only; **`/v1` API** remains on `api.<platform-host>`.
3. **TLS:** **Caddy on-demand TLS** at the edge; background job polls DNS every 5 min until verified or 72 h expiry.
4. **Routing:** resolve tenant by `Host` header on portal/admin; canonical redirect from platform subdomain optional.
5. **Invariant:** at most **one active primary domain** per tenant (BR-DM-001).
6. PlatformAdmin may force-verify, detach, or set canonical domain.

## Consequences

### Positive

- TXT verification is standard and works with any DNS provider.
- API stays centralized — simpler CORS and JWT issuer config.
- Caddy on-demand TLS fits self-hosted deployment.

### Negative

- Split hostnames (UI vs API) require correct CORS and cookie domain config.
- DNS propagation delays user experience.

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| CNAME-only verification | Less explicit ownership proof |
| Custom domain for API v1 | Certificate and routing complexity |
| Cloudflare proxy | Preferred public edge in ADR-019 (Nginx Ingress + Full Strict); self-hosted Caddy remains alternate |

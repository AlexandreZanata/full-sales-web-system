# ADR-019: Nginx Ingress + Cloudflare edge

**Status:** Accepted  
**Date:** 2026-07-12  
**Deciders:** Phase 15 defaults (OD-15-1 … OD-15-8)

## Context

Production needs a deployable edge for platform hosts (`api.`, `admin.`, `portal.`, `platform.`) and tenant custom domains (ADR-017). ADR-017 assumed **Caddy on-demand TLS** for self-hosted edges. We now standardize on **Cloudflare** in front of the cluster and **Nginx Ingress** inside Kubernetes.

## Decision

1. **Public edge:** Cloudflare (DNS proxy, WAF, visitor TLS). SSL mode **Full (Strict)** only — never Flexible.
2. **Cluster edge:** Nginx Ingress Controller terminates origin TLS (Cloudflare Origin Certificate by default; cert-manager optional).
3. **Packaging:** Kustomize under `deploy/kubernetes/` (base + `overlays/staging` + `overlays/prod`).
4. **Staging data stores:** in-cluster Postgres, Redis, MinIO; production uses managed connection strings via Secrets.
5. **Custom domains:** unchanged verification (TXT `_fullsales-verify.<domain>`); routing preserves `Host` to portal/admin; API remains on `api.<platform>` (ADR-017).
6. **Authenticated Origin Pulls:** deferred (OD-15-6 = No for v1).

## Consequences

### Positive

- CDN/WAF without operating Let’s Encrypt for every tenant hostname at the origin.
- Manifests stay plain YAML + Kustomize — reviewable, no Helm lock-in for v1.
- Aligns SECURITY.md “TLS mandatory” with a clear CF→origin HTTPS path.

### Negative

- Couples public DNS/WAF operations to Cloudflare.
- Origin must present a cert Cloudflare trusts (Origin CA or public LE).
- ADR-017 Caddy checklist becomes secondary; keep as alternate self-hosted path.

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| Caddy-only public edge | Conflicts with Cloudflare-first requirement |
| Traefik Ingress | Extra surface; team standardizes on Nginx |
| Helm umbrella chart | Heavier than needed for v1; Kustomize overlays suffice |
| Cloudflare Flexible SSL | Breaks secure cookies and SECURITY.md transport |

## References

- [ADR-017](ADR-017-custom-domain-verification.md)
- [docs/deployment/kubernetes.md](../deployment/kubernetes.md)
- [docs/deployment/nginx-ingress.md](../deployment/nginx-ingress.md)
- [docs/deployment/cloudflare.md](../deployment/cloudflare.md)

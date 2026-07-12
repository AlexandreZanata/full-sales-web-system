# Nginx Ingress

> Phase 15D · ADR-019 — stub filled as Ingress lands.

## Status

Base workloads live under `deploy/kubernetes/`. **Ingress resources and origin TLS** are Phase **15D**.

## Planned

- Install Nginx Ingress Controller
- Platform hosts: `api.`, `admin.`, `portal.`, `platform.`
- Preserve `Host` for custom domains (ADR-017)
- Origin TLS: Cloudflare Origin Certificate (default) or cert-manager
- HTTP→HTTPS redirect; `X-Forwarded-Proto` / `X-Forwarded-For`

## Related

- [caddy-custom-domains.md](caddy-custom-domains.md) — legacy self-hosted checklist
- [cloudflare.md](cloudflare.md)

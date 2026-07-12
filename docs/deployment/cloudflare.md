# Cloudflare edge

> Phase 15E · ADR-019 — stub filled when DNS/WAF is configured.

## Status

Origin workloads and images are Phase **15A–15C**. Cloudflare zone, Full Strict, WAF, and cache rules are Phase **15E**.

## Hard rules (already decided)

- SSL/TLS mode: **Full (Strict)** only — never Flexible
- Bypass cache for `/v1/*`
- Do not challenge Asaas webhook paths
- Orange-cloud (proxied) for platform hosts by default (OD-15-8)

## Related

- [nginx-ingress.md](nginx-ingress.md)
- [ADR-019](../adr/ADR-019-nginx-cloudflare-edge.md)

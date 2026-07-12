# Cloudflare edge

> Phase 15E · ADR-019

## DNS

| Record | Type | Target | Proxy |
|--------|------|--------|-------|
| `api` | A/AAAA or CNAME | Ingress LoadBalancer | Orange (proxied) |
| `admin` | A/AAAA or CNAME | Ingress LB | Orange |
| `portal` | A/AAAA or CNAME | Ingress LB | Orange |
| `platform` | A/AAAA or CNAME | Ingress LB | Orange |
| Tenant custom | CNAME | platform edge / portal hostname | Orange when possible (OD-15-8) |

Gray-cloud only if debugging origin directly.

## SSL/TLS

1. Mode: **Full (Strict)** — never Flexible
2. Install Cloudflare Origin Certificate on cluster Secret `cloudflare-origin-tls` ([nginx-ingress.md](nginx-ingress.md))
3. Enable **Always Use HTTPS**
4. HSTS: enable after Full Strict proven (start with short `max-age`)

## WAF

- Enable Cloudflare managed ruleset (baseline)
- Challenge noisy bot paths on SPAs if needed
- **Do not** challenge or block Asaas webhooks

### Webhook exception

Exact path (API contract):

`POST /v1/billing/webhooks/asaas`

Create a WAF/firewall skip or allow rule for that path (and optionally Asaas egress IPs). Confirm sandbox and production webhook URLs point at `https://api.<platform>/v1/billing/webhooks/asaas`.

## Cache rules

| Path / asset | Rule |
|--------------|------|
| `/v1/*` | **Bypass cache** (API) |
| `/health`, `/health/ready` | Bypass |
| SPA hashed assets (`*.js`, `*.css`, images under `/assets/`) | Cache aggressively (compatible with SPA nginx `immutable`) |
| `index.html` / `/` HTML | Bypass or short TTL (no long cache) |

## Custom domains (tenants)

1. Tenant adds domain in Admin (TXT `_fullsales-verify.<domain>` — ADR-017)
2. Tenant CNAMEs hostname → platform edge (proxied)
3. Origin Ingress catch-all serves portal; `Host` preserved
4. API remains on `api.<platform>`

See also [caddy-custom-domains.md](caddy-custom-domains.md) for verification flow (TLS path differs).

## Authenticated Origin Pulls

OD-15-6: **deferred** for v1. Optional hardening later (mTLS CF→origin).

## Secrets

- Cloudflare API token (DNS automation / CI): store in GitHub Actions secrets / vault — **never git**
- Origin cert private key: Kubernetes TLS Secret only

## Checklist (staging)

- [ ] Zone DNS records orange-cloud → Ingress LB
- [ ] Full (Strict) + Origin Cert installed
- [ ] Always Use HTTPS on
- [ ] Cache bypass for `/v1/*`
- [ ] WAF skip for `/v1/billing/webhooks/asaas`
- [ ] `curl https://api.<staging>/health` OK through Cloudflare
- [ ] SPA loads; API XHR not cached
- [ ] Sandbox Asaas webhook delivered

## Screenshots (optional)

Add ops screenshots under `.local/` if useful — do not commit secrets or full account IDs.

## Related

- [nginx-ingress.md](nginx-ingress.md)
- [ADR-019](../adr/ADR-019-nginx-cloudflare-edge.md)
- [API-CONTRACT.md](../API-CONTRACT.md) — Asaas webhook

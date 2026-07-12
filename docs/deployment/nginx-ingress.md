# Nginx Ingress + origin TLS

> Phase 15D · ADR-019

## Install controller (once per cluster)

```bash
helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
helm repo update
helm upgrade --install ingress-nginx ingress-nginx/ingress-nginx \
  --namespace ingress-nginx --create-namespace \
  --set controller.service.type=LoadBalancer \
  --set controller.config.use-forwarded-headers=true \
  --set controller.config.compute-full-forwarded-for=true
```

Set `controller.config.proxy-real-ip-cidr` to [Cloudflare IP ranges](https://www.cloudflare.com/ips/) so `X-Forwarded-For` is trustworthy.

Manifest note ConfigMap: `deploy/kubernetes/base/ingress-controller-notes.yaml`.

## Ingress resources

| Resource | Purpose |
|----------|---------|
| `ingress-platform.yaml` | `api.` / `admin.` / `portal.` / `platform.` hosts |
| `ingress-custom-domains.yaml` | Catch-all → portal (tenant custom domains, ADR-017) |

API traffic is **only** on `api.<platform>` (no `/v1` on SPA hosts). Custom domains preserve the request `Host` for tenant resolution.

Replace `*.example.com` with real DNS names before apply (edit Ingress or overlay patch).

## Origin TLS (OD-15-5)

Default: **Cloudflare Origin Certificate** as Secret `cloudflare-origin-tls`:

```bash
kubectl -n fullsales-staging create secret tls cloudflare-origin-tls \
  --cert=./origin.pem --key=./origin-key.pem \
  --dry-run=client -o yaml | kubectl apply -f -
```

Then remove or overwrite the placeholder Secret from base (`tls-secret-placeholder.yaml`).

Alternative: cert-manager + Let’s Encrypt (HTTP-01 or DNS-01) — optional if not using Origin CA.

## Annotations

- `ssl-redirect` / `force-ssl-redirect` — HTTP→HTTPS
- `use-forwarded-headers` / `compute-full-forwarded-for` — proto/client IP behind Cloudflare
- `proxy-body-size: 32m` — media uploads

Nginx sets `X-Forwarded-Proto`, `X-Forwarded-For`, and keeps `Host` for upstreams.

## CORS / cookies (ADR-017)

| Concern | Guidance |
|---------|----------|
| API host | `https://api.<platform>` only |
| SPA hosts | `admin.` / `portal.` / `platform.` / custom domains |
| CORS | Allow SPA origins on API; never reflect arbitrary Origin |
| Cookies | Prefer Bearer tokens (current apps); if cookies, `Secure; SameSite=None` cross-site |
| Custom domain portal | Calls API on platform host — CORS must list verified tenant origins or use a controlled allowlist |

## Nginx vs Caddy

| Path | Doc |
|------|-----|
| Nginx + Cloudflare (preferred) | this file |
| Self-hosted Caddy (alternate) | [caddy-custom-domains.md](caddy-custom-domains.md) |

## Smoke

```bash
# After controller + apply
curl -fsS -H 'Host: api.example.com' https://<ingress-ip>/health --insecure
curl -fsS -H 'Host: portal.example.com' https://<ingress-ip>/ --insecure
curl -fsS -H 'Host: shop.tenant.test' https://<ingress-ip>/ --insecure   # custom → portal
```

## Related

- [cloudflare.md](cloudflare.md)
- [kubernetes.md](kubernetes.md)
- [ADR-019](../adr/ADR-019-nginx-cloudflare-edge.md)

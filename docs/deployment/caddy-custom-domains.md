# Caddy — custom domain deployment checklist

> Phase 13D · Tenant custom domains (Phase 7 API)

## Prerequisites

- Tenant on **Pro** or higher plan
- Domain verified via `POST /v1/platform/jobs/domain-verification` (or DNS TXT match)
- Portal app deployed (e.g. `apps/portal` static build or Vite dev proxy)
- TLS certificate (Caddy automatic HTTPS)

## DNS (tenant admin)

1. Add domain in Admin → Settings → Domains (`POST /v1/settings/domains`)
2. Create **TXT** record: `_fullsales-verify.<hostname>` → value from API `txtValue`
3. Optional **CNAME** or **A** record pointing `<hostname>` to your edge server

## Caddyfile example

```caddy
shop.example.com {
    reverse_proxy 127.0.0.1:5175

    header {
        X-Forwarded-Host {host}
    }
}
```

Portal/API must receive the original `Host` header so `GET /v1/public/settings` resolves the tenant.

## Verification steps

| Step | Command / action | Expected |
|------|------------------|----------|
| Domain active | `GET /v1/settings/domains` (Admin JWT) | `status: Active` |
| Public resolve | `curl -H "Host: shop.example.com" http://127.0.0.1:8080/v1/public/settings` | Tenant `displayName` |
| Portal load | Browser `https://shop.example.com` | Branded portal home |
| Primary domain | Only one domain `isPrimary: true` | Old primary → `Detached` |

## Production checklist

- [ ] Caddy (or Nginx) terminates TLS for all tenant hostnames
- [ ] `Host` header forwarded to API unchanged
- [ ] Domain verification job scheduled (cron or platform job runner)
- [ ] Staging uses `force-verify` only in non-production environments
- [ ] Monitor failed verifications in platform admin Domains console

## References

- [API-CONTRACT.md](../API-CONTRACT.md) — `/v1/settings/domains/*`
- [tenant_domains.rs](../../backend/crates/api-http/tests/tenant_domains.rs) — contract tests
- [platform_saas/webhook_fraud_domain.rs](../../backend/crates/api-http/tests/platform_saas/webhook_fraud_domain.rs) — mock DNS path

**Implemented:** Phase 13.

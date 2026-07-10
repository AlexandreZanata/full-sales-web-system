# Platform Admin UI

> Super Admin SPA at `apps/platform-admin` (`@full-sales/platform-admin`).

## Run

```bash
pnpm dev:platform-admin              # http://127.0.0.1:5177
pnpm --filter @full-sales/platform-admin lint test build
```

Proxies `/v1` and `/health` to the Rust API (`127.0.0.1:8080`).

## Auth

- `POST /v1/platform/auth/login` → MFA step or tokens
- `POST /v1/platform/auth/mfa/verify`
- `POST /v1/platform/auth/refresh` / `logout`
- Route guard requires `PlatformAdmin` JWT role
- Dev stub session on login page when `import.meta.env.DEV`

## Routes

| Path | Purpose |
|------|---------|
| `/` | Dashboard KPIs, health matrix, recent fraud |
| `/tenants` | Tenant list + filters |
| `/tenants/new` | Provision tenant |
| `/tenants/$id` | Detail tabs (overview, users, billing, domains, audit) |
| `/users` | Cross-tenant user search |
| `/users/$id` | Disable, reset password, impersonate |
| `/billing` | Tenant billing statuses + dunning job |
| `/fraud` | Fraud queue + blocklist |
| `/domains` | Custom domains + force verify |
| `/health` | Probe matrix + history |
| `/maintenance` | Schedule maintenance window |
| `/audit` | Platform audit log |

## i18n

`en` + `pt-BR` — `src/lib/i18n/locales/`, storage key `platform-admin-locale`.

## Tests

Contract-first unit tests in `apps/platform-admin/tests/unit/` (JWT, API client, route coverage, tenants query).

Integration E2E (Phase 13): [platform-saas-e2e.md](./platform-saas-e2e.md) — `cargo test -p api-http platform_saas`.

**Implemented:** Phase 11.

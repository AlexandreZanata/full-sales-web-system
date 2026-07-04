# Client apps — Phase 39

> Commerce portal PWA (`apps/portal`) and field seller PWA (`apps/field`).

## Packages

| Package | Port | Actor | Dev login |
|---------|------|-------|-----------|
| `@full-sales/portal` | 5175 | CommerceContact | `portal@seed-store.com` / `secret123` |
| `@full-sales/field` | 5176 | Driver, Seller | `seller@test.com` / `secret123` |

## Stack

React 19, Vite, TanStack Router + Query, Tailwind v4, pt-BR default i18n, installable PWA (manifest + static SW).

## Portal routes

| Route | API |
|-------|-----|
| `/login` | `POST /v1/auth/login` |
| `/` | `GET /v1/portal/products` |
| `/products/$id` | product from catalog list |
| `/cart` | local cart + `POST /v1/portal/orders`, `POST …/submit` |
| `/orders` | `GET /v1/portal/orders` |
| `/orders/$id` | `GET`, `PUT`, `DELETE`, `POST …/submit` (draft) |

## Field routes

| Route | API |
|-------|-----|
| `/login` | `POST /v1/auth/login` |
| `/` | `GET /v1/sales` |
| `/sales/new` | `GET /v1/commerces`, `GET /v1/products`, stock hint |
| `/sales/$id` | `POST …/confirm`, `POST …/cancel` |

## Commands

```bash
pnpm dev:portal
pnpm dev:field
pnpm --filter @full-sales/portal lint test build
pnpm --filter @full-sales/field lint test build
pnpm test:e2e:portal
pnpm test:e2e:field
```

## Known gaps

See `.local/phases/39-field-client-apps/ROUTE-GAPS.md` for backend/UI routes not yet wired.

**Updated:** 2026-07-04

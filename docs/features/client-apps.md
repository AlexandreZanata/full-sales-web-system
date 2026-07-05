# Client apps — Phase 39

> Commerce portal PWA (`apps/portal`), field PWA (`apps/field`), KMP field app (`apps-mobile/field`), and KMP seller app (`apps-mobile/seller`).

## Packages

| Package | Port | Actor | Dev login |
|---------|------|-------|-----------|
| `@full-sales/portal` | 5175 | CommerceContact | `portal@seed-store.com` / `secret123` |
| `@full-sales/field` | 5176 | Driver, Seller | `seller@test.com` / `driver-a@test.com` / `secret123` |
| `apps-mobile/field` | — | Driver, Seller | KMP offline (39F) |
| `apps-mobile/seller` | — | Seller | KMP seller shell; auth (53); M3 theme + sales list (60); Room offline (55) |

## Stack

React 19, Vite, TanStack Router + Query, Tailwind v4, pt-BR default i18n, installable PWA (manifest + static SW).

## Tenant branding (Phase 41)

Portal and field shells call `GET /v1/settings` after login to show tenant `displayName` and presigned `logoUrl` in the header (5 min query staleTime). Admin configures branding at `/settings`.

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

| Route | API | Role |
|-------|-----|------|
| `/login` | `POST /v1/auth/login` | Driver, Seller |
| `/` | `GET /v1/sales` | Both |
| `/sales/new` | commerces, products, stock | Seller |
| `/sales/$id` | confirm / cancel | Both |
| `/deliveries` | `GET /v1/deliveries` | Driver |
| `/deliveries/$id` | start-transit, confirm + proof | Driver |

## Commands

```bash
pnpm dev:portal
pnpm dev:field
pnpm --filter @full-sales/portal lint test build
pnpm --filter @full-sales/field lint test build
pnpm test:e2e:portal
pnpm test:e2e:field
pnpm test:e2e:integration   # API on :8080 + dev seeds
```

## KMP offline app (39F)

Path: `apps-mobile/field/` · Room + WorkManager + Compose sales UI

```bash
cd apps-mobile/field
./gradlew :shared:check :androidApp:lint :androidApp:assembleDebug
./gradlew :androidApp:connectedDebugAndroidTest   # emulator
```

Emulator API: `http://10.0.2.2:8080`

## KMP seller app (Phase 52+)

Path: `apps-mobile/seller/` · Compose shell (Phase 57); `SellerApiClient` (54); Room + sync (55–56).

```bash
cd apps-mobile/seller
./gradlew :shared:check :androidApp:assembleDebug
```

See `apps-mobile/seller/README.md` for module layout and validation gate.

## Known gaps

See `.local/phases/39-field-client-apps/FOLLOW-UP-TASKS.md`.

**Updated:** 2026-07-05

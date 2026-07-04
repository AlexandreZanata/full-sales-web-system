# Admin Panel

> Standalone SPA at `apps/admin` (`@full-sales/admin`).  
> API contract: [API-CONTRACT.md](../API-CONTRACT.md) · Backend routes: [ROUTE-MATRIX.md](../ROUTE-MATRIX.md)

**Status:** ✅ Complete (Phases 27–36)

---

## Purpose

Internal operations console for **Admin** role: users, commerces, catalog, inventory, orders, deliveries, sales, signed reports, and audit log.

---

## Dev commands

```bash
pnpm dev:admin                              # http://127.0.0.1:5174
pnpm --filter @full-sales/admin lint test build
pnpm test:e2e:admin                         # Playwright (starts dev server)
```

Vite proxies `/v1` and `/health` to `http://127.0.0.1:8080` (override with `VITE_DEV_API_ORIGIN`).

**Dev login:** open `/login` and sign in with seeded credentials (`pnpm seed:dev`):

- **Admin:** `admin@test.com` / `secret123`

See [DEV-COMMANDS.md](../DEV-COMMANDS.md) for all seed users and `pnpm seed:dev` usage.

**Test credentials (integration seed):** `admin@test.com` / `secret123` when the API runs with test/seed data.

---

## Routes (23 screens)

| Label | Route | Domain |
|-------|-------|--------|
| Dashboard | `/` | Stats + recent sales |
| Users | `/users`, `/users/new`, `/users/$id` | CRUD + profiles |
| Commerces | `/commerces`, `/commerces/new`, `/commerces/$id` | CRUD + addresses + logo |
| Products | `/products`, `/products/new`, `/products/$id` | CRUD + images + stock |
| Inventory | `/inventory`, `/inventory/adjustments`, `/inventory/ledger` | Adjustments + ledger |
| Orders | `/orders`, `/orders/$id` | List + workflow actions |
| Deliveries | `/deliveries`, `/deliveries/$id` | List + read-only detail |
| Sales | `/sales`, `/sales/new`, `/sales/$id` | List + create + confirm/cancel |
| Reports | `/reports`, `/reports/new`, `/reports/$id` | Signed reports + verify link |
| Audit | `/audit` | Append-only event log |

Full API↔UI matrix: `.local/phases/_reference/UI-ROUTE-MATRIX.md`

---

## i18n

Full locale coverage (`en`, `pt-BR`) with `localStorage` persistence and nested message keys (~220).

| Scope | Keys |
|-------|------|
| Sidebar nav | `nav.*` |
| Login | `auth.*` |
| Shell / menu | `shell.*` |
| Shared actions / tables | `common.*` |
| Form labels & validation | `forms.fields.*`, `forms.validation.*` |
| API error mapping | `errors.*` |
| Status badges | `status.order.*`, `status.sale.*`, `status.delivery.*`, `status.report.*` |
| Roles / payment / addresses | `role.*`, `payment.*`, `addressType.*` |
| Domain screens | `dashboard.*`, `users.*`, `commerces.*`, `products.*`, `inventory.*`, `orders.*`, `deliveries.*`, `sales.*`, `reports.*`, `audit.*` |
| Uploads | `uploads.*` |

Helpers: `apps/admin/src/lib/i18n/labels.ts` (`translateOrderStatus`, filter labels, `formatPaginationSummary`, action error keys).

Catalogs: `apps/admin/src/lib/i18n/locales/`. Parity test: `apps/admin/tests/unit/i18n.test.ts`.

---

## Tech stack

| Layer | Choice |
|-------|--------|
| Framework | React 19 + Vite |
| Router | TanStack Router (file routes) |
| Server state | TanStack React Query v5 |
| Styling | Tailwind CSS v4 + CSS variables |
| Auth | JWT Bearer (`POST /v1/auth/login`, refresh, logout) |
| E2E | Playwright (`e2e/admin-*.spec.ts`) |

---

## Testing

| Layer | Command |
|-------|---------|
| Unit | `pnpm --filter @full-sales/admin test` |
| E2E | `pnpm test:e2e:admin` |
| CI | `.github/workflows/ci.yml` — lint, test, build, Playwright |

E2E specs mock `/v1/auth/login` and list endpoints so they run without a live API.

---

## Design

- White/off-white background, near-black text, solid black primary buttons
- 240px sidebar (desktop); slide-out drawer (mobile, 390px+ tested)
- Status badges: text label + colored dot (not color-only)
- Touch targets: buttons and nav links ≥ 40px (`min-h-10`)

Tokens: `apps/admin/src/styles/admin-theme.css`, `apps/admin/src/lib/admin-tokens.ts`.

---

## Implementation phases (local)

| Phase | Scope |
|-------|--------|
| 27 | Master index + UI↔API matrix |
| 28 | App scaffold, theme, shell, API client |
| 29 | Login, JWT session, dashboard |
| 30–35 | Domain screens (users → audit) |
| 36 | i18n foundation, E2E, route audit, CI, docs |
| 38 | Full admin i18n (en + pt-BR, all 23 screens) |

---

**Updated:** 2026-07-04 (Phase 38)

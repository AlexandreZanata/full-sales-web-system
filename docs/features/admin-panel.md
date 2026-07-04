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

**Dev login:** open `/login` and sign in with an Admin account (`POST /v1/auth/login`). In dev, **Enter admin shell (dev)** previews the layout without the API.

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

Full API↔UI matrix: `.local/phases/27-admin-panel-master/documentation/UI-ROUTE-MATRIX.md`

---

## i18n

Lightweight locale switcher (`en`, `pt-BR`) with `localStorage` persistence.

| Scope | Keys |
|-------|------|
| Sidebar nav | `nav.*` |
| Login | `auth.*` |
| Shell / menu | `shell.*` |
| Shared actions | `common.*` (pagination, confirm dialogs) |

Catalogs: `apps/admin/src/lib/i18n/locales/`.

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
| 36 | i18n, E2E, route audit, CI, docs |

---

**Updated:** 2026-07-04 (Phase 36)

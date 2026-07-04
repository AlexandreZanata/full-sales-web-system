# Admin Panel

> Standalone SPA at `apps/admin` (`@full-sales/admin`).  
> API contract: [API-CONTRACT.md](../API-CONTRACT.md) · Backend routes: [ROUTE-MATRIX.md](../ROUTE-MATRIX.md)

**Status:** 🟡 Sales complete (Phase 34) — reports and audit in Phases 35–36.

---

## Purpose

Internal operations console for **Admin** role: users, commerces, catalog, inventory, orders, deliveries, sales, signed reports, and audit log. Mirrors the layout and UX of the reference app (`open-3d-store-free-to-use/apps/admin`).

---

## Dev commands

```bash
pnpm dev:admin                              # http://127.0.0.1:5174
pnpm --filter @full-sales/admin lint test build
```

Vite proxies `/v1` and `/health` to `http://127.0.0.1:8080` (override with `VITE_DEV_API_ORIGIN`).

**Dev login:** open `/login` and sign in with an Admin account (`POST /v1/auth/login`). In dev, **Enter admin shell (dev)** still previews the layout without the API.

**Test credentials (integration seed):** `admin@test.com` / `secret123` when the API runs with test/seed data.

---

## Sidebar navigation

| Label | Route | Phase | UI |
|-------|-------|-------|-----|
| Dashboard | `/` | 29 | ✅ live |
| Users | `/users` | 30 | ✅ live |
| Commerces | `/commerces` | 31 | ✅ live |
| Products | `/products` | 32 | ✅ live |
| Inventory | `/inventory` | 32 | ✅ live |
| Orders | `/orders` | 33 | ✅ live |
| Deliveries | `/deliveries` | 33 | ✅ live |
| Sales | `/sales` | 34 | ✅ live |
| Reports | `/reports` | 35 | ✅ stub |
| Audit | `/audit` | 35 | ✅ stub |

---

## Tech stack

| Layer | Choice |
|-------|--------|
| Framework | React 19 + Vite |
| Router | TanStack Router (file routes) |
| Server state | TanStack React Query v5 |
| Styling | Tailwind CSS v4 + CSS variables |
| Auth | JWT Bearer (`POST /v1/auth/login`, refresh, logout) |

---

## Implementation plan (local)

| Phase | Folder | Scope |
|-------|--------|-------|
| **27** | `.local/phases/27-admin-panel-master/` | ✅ Master index + UI↔API matrix |
| **28** | `.local/phases/28-admin-shell/` | ✅ App scaffold, theme, shell, API client |
| **29** | `.local/phases/29-admin-auth-dashboard/` | ✅ Login, JWT session, dashboard |
| **30** | `.local/phases/30-admin-users/` | ✅ Users list, create, detail, profiles |
| **31** | `.local/phases/31-admin-commerces/` | ✅ Commerces list, create, detail, addresses, logo |
| **32** | `.local/phases/32-admin-products-inventory/` | ✅ Products CRUD, images, stock card, inventory hub |
| **33** | `.local/phases/33-admin-orders-deliveries/` | ✅ Orders list/detail/actions, deliveries list/detail |
| **34** | `.local/phases/34-admin-sales/` | ✅ Sales list, detail, create, confirm/cancel |
| **35–36** | `.local/phases/35-admin-reports-audit/` … | Reports, audit + governance |

**UI route matrix:** `.local/phases/27-admin-panel-master/documentation/UI-ROUTE-MATRIX.md`  
**Gap tracker:** `.local/phases/27-admin-panel-master/documentation/GAP-TASKS.md`

---

## Design

- White/off-white background, near-black text, solid black primary buttons
- 240px sidebar (desktop); slide-out drawer (mobile)
- Status badges: text label + colored dot

Tokens: `apps/admin/src/styles/admin-theme.css`, `apps/admin/src/lib/admin-tokens.ts`.

---

**Updated:** 2026-07-04 (Phase 34)

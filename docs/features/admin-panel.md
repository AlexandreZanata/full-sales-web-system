# Admin Panel

> Standalone SPA at `apps/admin` (`@full-sales/admin`).  
> API contract: [API-CONTRACT.md](../API-CONTRACT.md) · Backend routes: [ROUTE-MATRIX.md](../ROUTE-MATRIX.md)

**Status:** 🟡 Shell complete (Phase 28) — auth + domain screens in Phases 29–36.

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

**Dev login:** open `/login` → **Enter admin shell (dev)** previews the layout until Phase 29 wires `POST /v1/auth/login`.

---

## Sidebar navigation

| Label | Route | Phase | UI |
|-------|-------|-------|-----|
| Dashboard | `/` | 29 | ✅ stub |
| Users | `/users` | 30 | ✅ stub |
| Commerces | `/commerces` | 31 | ✅ stub |
| Products | `/products` | 32 | ✅ stub |
| Inventory | `/inventory` | 32 | ✅ stub |
| Orders | `/orders` | 33 | ✅ stub |
| Deliveries | `/deliveries` | 33 | ✅ stub |
| Sales | `/sales` | 34 | ✅ stub |
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
| **29** | `.local/phases/29-admin-auth-dashboard/` | Login, JWT session, dashboard |
| **30–36** | `.local/phases/30-admin-users/` … | Domain screens + governance |

**UI route matrix:** `.local/phases/27-admin-panel-master/documentation/UI-ROUTE-MATRIX.md`  
**Gap tracker:** `.local/phases/27-admin-panel-master/documentation/GAP-TASKS.md`

---

## Design

- White/off-white background, near-black text, solid black primary buttons
- 240px sidebar (desktop); slide-out drawer (mobile)
- Status badges: text label + colored dot

Tokens: `apps/admin/src/styles/admin-theme.css`, `apps/admin/src/lib/admin-tokens.ts`.

---

**Updated:** 2026-07-04

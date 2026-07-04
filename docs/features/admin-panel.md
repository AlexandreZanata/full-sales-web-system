# Admin Panel

> Standalone SPA at `apps/admin` (`@full-sales/admin`) — **planned** (Phases 28–36).  
> API contract: [API-CONTRACT.md](../API-CONTRACT.md) · Backend routes: [ROUTE-MATRIX.md](../ROUTE-MATRIX.md)

**Status:** ⬜ Not built — backend Admin API is complete (Phases 16–26); frontend work starts at Phase 28.

---

## Purpose

Internal operations console for **Admin** role: users, commerces, catalog, inventory, orders, deliveries, sales, signed reports, and audit log. Mirrors the layout and UX of the reference app (`open-3d-store-free-to-use/apps/admin`).

---

## Active implementation plan (local)

| Phase | Folder | Scope |
|-------|--------|-------|
| **27** | `.local/phases/27-admin-panel-master/` | Master index + UI↔API matrix + route audit |
| **28** | `.local/phases/28-admin-shell/` | App scaffold, theme, shell, API client |
| **29** | `.local/phases/29-admin-auth-dashboard/` | Login, JWT session, dashboard |
| **30** | `.local/phases/30-admin-users/` | Users + profiles |
| **31** | `.local/phases/31-admin-commerces/` | Commerces, addresses, logo |
| **32** | `.local/phases/32-admin-products-inventory/` | Products, stock, movements |
| **33** | `.local/phases/33-admin-orders-deliveries/` | Orders lifecycle + deliveries |
| **34** | `.local/phases/34-admin-sales/` | Sales admin view |
| **35** | `.local/phases/35-admin-reports-audit/` | Reports + audit log |
| **36** | `.local/phases/36-admin-governance/` | E2E, i18n, 100% route audit |

**UI route matrix:** `.local/phases/27-admin-panel-master/documentation/UI-ROUTE-MATRIX.md`  
**Gap tracker:** `.local/phases/27-admin-panel-master/documentation/GAP-TASKS.md`  
**Route audit:** `.local/phases/27-admin-panel-master/documentation/ROUTE-AUDIT.md`

---

## Sidebar navigation (target)

| Label | Route | Phase |
|-------|-------|-------|
| Dashboard | `/` | 29 |
| Users | `/users` | 30 |
| Commerces | `/commerces` | 31 |
| Products | `/products` | 32 |
| Inventory | `/inventory` | 32 |
| Orders | `/orders` | 33 |
| Deliveries | `/deliveries` | 33 |
| Sales | `/sales` | 34 |
| Reports | `/reports` | 35 |
| Audit | `/audit` | 35 |

---

## Tech stack (planned)

| Layer | Choice |
|-------|--------|
| Framework | React 19 + Vite |
| Router | TanStack Router (file routes) |
| Server state | TanStack React Query v5 |
| Styling | Tailwind CSS v4 + CSS variables |
| Auth | JWT Bearer (`POST /v1/auth/login`, refresh, logout) |

---

## Dev commands (after Phase 28)

```bash
pnpm dev:admin                              # Vite dev server
pnpm --filter @full-sales/admin lint test build
```

Full doc (routes, mobile layout, media uploads) will be promoted here in **Phase 36**.

---

**Updated:** 2026-07-04

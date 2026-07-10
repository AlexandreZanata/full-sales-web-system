# Route matrix — Full Sales Web System API

> Single source for REST coverage. **Status:** ✅ implemented · ⬜ pending · 🔒 public (no JWT)

Base path: `/v1/` unless noted. Error shape: RFC 9457 / `docs/API-CONTRACT.md`.

Admin UI coverage: `.local/phases/_reference/UI-ROUTE-MATRIX.md`

---

## Legend

| Column | Meaning |
|--------|---------|
| **Migrations** | Primary tables (see `backend/migrations/`) |
| **Rules** | Business rules from `docs/BUSINESS-RULES.md` / expansion |
| **API phase** | Backend work (Phases 16–26 — complete) |

---

## Health & meta

| Method | Path | Auth | Status | Migrations | Task |
|--------|------|------|--------|------------|------|
| GET | `/health` | 🔒 | ✅ | — | — |
| GET | `/health/ready` | 🔒 | ✅ | `ops.health_probe_results` | Phase 9 |
| GET | `/v1/status` | 🔒 | ✅ | — | Phase 9 |
| GET | `/v1/` | 🔒 | ✅ | — | — |

---

## Auth (`identity.users` + Redis refresh)

| Method | Path | Auth | Status | Migrations | Rules | Task |
|--------|------|------|--------|------------|-------|------|
| POST | `/v1/auth/login` | 🔒 rate limit | ✅ | `20260704120100` | BR-IA-* | — |
| POST | `/v1/auth/refresh` | 🔒 | ✅ | Redis (ADR-009) | — | — |
| POST | `/v1/auth/logout` | Bearer | ✅ | Redis | — | — |

---

## Users — Admin (`identity.users`, profiles)

| Method | Path | Auth | Status | Migrations | Rules | Task |
|--------|------|------|--------|------------|-------|------|
| POST | `/v1/users` | Admin | ✅ | `201`, `224`, `225` | BR-IA-001 | 17 |
| GET | `/v1/users` | Admin | ✅ | `201` | pagination | 17 |
| GET | `/v1/users/{id}` | Admin | ✅ | `201` | — | 17 |
| PATCH | `/v1/users/{id}/deactivate` | Admin | ✅ | `201` | soft delete | 17 |
| PUT | `/v1/users/{id}/driver-profile` | Admin | ✅ | `224` | CNH photo → media | 17 |
| PUT | `/v1/users/{id}/seller-profile` | Admin | ✅ | `225` | — | 17 |

---

## Commerces (`commerces.commerces`, `commerce_addresses`)

| Method | Path | Auth | Status | Migrations | Rules | Task |
|--------|------|------|--------|------------|-------|------|
| POST | `/v1/commerces` | Admin | ✅ | `202` | BR-IA-001 | — |
| GET | `/v1/commerces` | Admin, Driver, Seller | ✅ | `202` | read-only | 18 |
| GET | `/v1/commerces/{id}` | Admin, Driver, Seller | ✅ | `202` | — | 18 |
| PATCH | `/v1/commerces/{id}/deactivate` | Admin | ✅ | `202` | inactive blocks sales | 18 |
| GET | `/v1/commerces/{id}/addresses` | Admin, Driver, Seller | ✅ | `226` | — | 18 |
| POST | `/v1/commerces/{id}/addresses` | Admin | ✅ | `226` | primary per type | 18 |
| PATCH | `/v1/commerces/{id}/addresses/{addressId}` | Admin | ✅ | `226` | — | 18 |
| PUT | `/v1/commerces/{id}/logo` | Admin | ✅ | `227` | `logo_file_id` → media | 18 |
| GET | `/v1/commerces/cnpj-lookup` | Seller, Admin, reviewer | ✅ | `69` | BR-CO-012 | — |
| POST | `/v1/commerces/registrations` | Seller | ✅ | `69` | BR-CO-010 | — |
| GET | `/v1/commerces/registrations` | Seller (own), reviewer | ✅ | `69` | cursor + `filter[status]` | — |
| GET | `/v1/commerces/registrations/{id}` | Seller (own), reviewer | ✅ | `69` | — | — |
| PATCH | `/v1/commerces/registrations/{id}` | Seller (own pending), reviewer | ✅ | `69` | — | — |
| POST | `/v1/commerces/registrations/{id}/approve` | Admin, `can_review_commerce` | ✅ | `69` | BR-CO-011 | — |
| POST | `/v1/commerces/registrations/{id}/reject` | Admin, `can_review_commerce` | ✅ | `69` | — | — |

---

## Products & catalog (`inventory.products`, `product_images`)

| Method | Path | Auth | Status | Migrations | Rules | Task |
|--------|------|------|--------|------------|-------|------|
| POST | `/v1/products` | Admin | ✅ | `203`, `228` | active catalog | 19 |
| GET | `/v1/products` | Admin, Driver, Seller | ✅ | `203` | pagination + `active?` filter | 40 |
| GET | `/v1/products/{id}` | Admin, Driver, Seller | ✅ | `203` | — | 19 |
| PATCH | `/v1/products/{id}` | Admin | ✅ | `203`, `228`, `portal_home_content` | deactivate / reactivate / `isFeatured` | 40, 71 |
| GET | `/v1/products/{id}/images` | Admin | ✅ | `229` | list gallery | 40 |
| POST | `/v1/products/{id}/images` | Admin | ✅ | `229` | one primary | 19 |
| DELETE | `/v1/products/{id}/images/{imageId}` | Admin | ✅ | `229` | — | 19 |

---

## Product categories (`inventory.product_categories`) — Phase 43

| Method | Path | Auth | Status | Migrations | Rules | Task |
|--------|------|------|--------|------------|-------|------|
| GET | `/v1/categories` | Admin | ✅ | `product_categories` | pagination + `active?` | 43 |
| POST | `/v1/categories` | Admin | ✅ | `product_categories` | slug unique per tenant | 43 |
| GET | `/v1/categories/{id}` | Admin | ✅ | `product_categories` | — | 43 |
| PATCH | `/v1/categories/{id}` | Admin | ✅ | `product_categories` | — | 43 |
| DELETE | `/v1/categories/{id}` | Admin | ✅ | `product_categories` | soft deactivate | 43 |
| POST | `/v1/categories/reorder` | Admin | ✅ | `product_categories` | sort order | 43 |
| PUT | `/v1/categories/{id}/image` | Admin | ✅ | `product_categories`, media | ProductCategory entity | 43 |
| GET | `/v1/public/categories` | Public | ✅ | `product_categories` | active only | 43 |
| GET | `/v1/public/categories/{slug}` | Public | ✅ | `product_categories` | + products | 43 |
| GET | `/v1/portal/categories` | CommerceContact | ✅ | `product_categories` | — | 43 |
| GET | `/v1/portal/categories/{slug}` | CommerceContact | ✅ | `product_categories` | + products | 43 |

Products `POST/PATCH` accept `categoryId` (uuid); legacy `category` string rejected with 400.

---

## Inventory (`stock_balances`, `stock_movements`, `stock_reservations`)

| Method | Path | Auth | Status | Migrations | Rules | Task |
|--------|------|------|--------|------------|-------|------|
| GET | `/v1/inventory/products/{productId}/balance` | Admin, Driver, Seller | ✅ | `204`, `205` | BR-IN-* | 19 |
| POST | `/v1/inventory/movements` | Admin | ✅ | `204`, `215` | adjustment reason | 19 |
| GET | `/v1/inventory/products/{productId}/movements` | Admin | ✅ | `204` | audit read | 19 |

System-generated movements (sale confirm, delivery) — **no public POST**; wired in application TX.

---

## Sales — field flow (`sales.sales`, `sale_items`)

| Method | Path | Auth | Status | Migrations | Rules | Task |
|--------|------|------|--------|------------|-------|------|
| POST | `/v1/sales` | Driver, Seller, Admin | ✅ | `206`, `207` | BR-SA-*, idempotency | — |
| GET | `/v1/sales` | Admin; Driver own; Seller own | ✅ | `206`, `217` | filters | 51 |
| GET | `/v1/sales/{id}` | Driver own, Seller own, Admin | ✅ | `206` | — | 51 |
| POST | `/v1/sales/{id}/confirm` | Driver, Seller, Admin | ✅ | `206`, `204` | BR-IN-002 | — |
| POST | `/v1/sales/{id}/cancel` | Driver, Seller, Admin | ✅ | `216` | Pending only | — |
| POST | `/v1/sales/{id}/declare-payment` | Driver (owner) | ✅ | `235` | RN-PAG1–3 | 20 |

---

## Portal — CommerceContact (`orders`, JWT `commerceId`)

| Method | Path | Auth | Status | Migrations | Rules | Task |
|--------|------|------|--------|------------|-------|------|
| GET | `/v1/public/products` | Public | ✅ | `228`, `229` | BR-IA-003 | — |
| GET | `/v1/public/products/{id}` | Public | ✅ | `product_description` | Gallery detail | 48 |
| GET | `/v1/public/products/featured` | Public | ✅ | `portal_home_content` | `is_featured` products | 71 |
| GET | `/v1/public/products/popular` | Public | ✅ | `portal_home_content`, `product_sales_totals` | sales rank fallback | 71 |
| GET | `/v1/public/banners` | Public | ✅ | `portal_home_content` | `placement` query | 71 |
| GET | `/v1/public/promotions` | Public | ✅ | `portal_home_content` | offer cards | 71 |
| GET | `/v1/portal/products` | CommerceContact | ✅ | `228`, `229` | BR-IA-003 | — |
| GET | `/v1/portal/products/{id}` | CommerceContact | ✅ | `product_description` | Gallery detail | 48 |
| GET | `/v1/portal/orders` | CommerceContact | ✅ | `231` | RLS | — |
| GET | `/v1/portal/orders/{id}` | CommerceContact | ✅ | `231`, `232` | — | — |
| POST | `/v1/portal/orders` | CommerceContact | ✅ | `231`, `232` | Draft | — |
| PUT | `/v1/portal/orders/{id}` | CommerceContact | ✅ | `231` | Draft only | — |
| DELETE | `/v1/portal/orders/{id}` | CommerceContact | ✅ | `231` | Draft cancel | — |
| POST | `/v1/portal/orders/{id}/submit` | CommerceContact | ✅ | `231` | — | — |

---

## Portal home content — Admin CMS (Phase 71)

| Method | Path | Auth | Status | Migrations | Rules | Task |
|--------|------|------|--------|------------|-------|------|
| GET | `/v1/portal/banners` | Admin | ✅ | `portal_home_content` | tenant RLS | 71 |
| POST | `/v1/portal/banners` | Admin | ✅ | `portal_home_content` | media `PortalBanner` | 71 |
| PATCH | `/v1/portal/banners/{id}` | Admin | ✅ | `portal_home_content` | — | 71 |
| DELETE | `/v1/portal/banners/{id}` | Admin | ✅ | `portal_home_content` | — | 71 |
| GET | `/v1/portal/promotions` | Admin | ✅ | `portal_home_content` | tenant RLS | 71 |
| POST | `/v1/portal/promotions` | Admin | ✅ | `portal_home_content` | `background` enum | 71 |
| PATCH | `/v1/portal/promotions/{id}` | Admin | ✅ | `portal_home_content` | — | 71 |
| DELETE | `/v1/portal/promotions/{id}` | Admin | ✅ | `portal_home_content` | — | 71 |

---

## Orders — Admin (`orders.orders`, `order_items`, reservations)

| Method | Path | Auth | Status | Migrations | Rules | Task |
|--------|------|------|--------|------------|-------|------|
| GET | `/v1/orders` | Admin | ✅ | `231`, `217` | pagination, filters | 21 |
| GET | `/v1/orders/{id}` | Admin | ✅ | `231`, `232` | — | 21 |
| POST | `/v1/orders/{id}/approve` | Admin | ✅ | `230`, `231` | RN2 | — |
| POST | `/v1/orders/{id}/reject` | Admin | ✅ | `231` | RN10 | — |
| POST | `/v1/orders/{id}/cancel` | Admin | ✅ | `231`, `230` | RN6 pre-InTransit | 21 |
| POST | `/v1/orders/{id}/start-picking` | Admin | ✅ | `231` | state machine | 21 |

---

## Deliveries (`deliveries.deliveries`)

| Method | Path | Auth | Status | Migrations | Rules | Task |
|--------|------|------|--------|------------|-------|------|
| POST | `/v1/orders/{id}/delivery` | Admin | ✅ | `233`, `231` | 1:1 DE-004, assign driver | 22 |
| GET | `/v1/deliveries` | Admin; Driver own | ✅ | `233`, `234` RLS | RN8 | 22 |
| GET | `/v1/deliveries/{id}` | Admin; Driver assigned | ✅ | `233` | — | 22 |
| POST | `/v1/deliveries/{id}/start-transit` | Driver assigned | ✅ | `233`, `231` | order → InTransit | 22 |
| POST | `/v1/deliveries/{id}/confirm` | Driver assigned | ✅ | `233`, `206`, `204`, `230` | RN4, RN5, sale TX | 22 |

---

## Site settings (`shared.tenants`)

| Method | Path | Auth | Status | Migrations | Rules | Task |
|--------|------|------|--------|------------|-------|------|
| GET | `/v1/settings` | Any authenticated tenant role | ✅ | `236`, `sales_contact_phone` | presigned logo URL + sales phone | 41, 50 |
| GET | `/v1/public/settings` | Public | ✅ | `sales_contact_phone` | guest portal branding subset | 50 |
| PATCH | `/v1/settings` | Admin | ✅ | `236`, `sales_contact_phone` | display name + sales phone | 41, 50 |
| PUT | `/v1/settings/logo` | Admin | ✅ | `236` | `logo_file_id` → media | 41 |

---

## Media (`media.files` + MinIO)

| Method | Path | Auth | Status | Migrations | Rules | Task |
|--------|------|------|--------|------------|-------|------|
| POST | `/v1/media/upload` | Admin, Driver, Seller, CommerceContact | ✅ | `222` | RN7, ADR-011 | 23 |
| GET | `/v1/media/{id}/url` | scoped by entity ownership | ✅ | `222` | presign ~15 min | 23 |

---

## Reports (`reports.reports`, `signing_keys`)

| Method | Path | Auth | Status | Migrations | Rules | Task |
|--------|------|------|--------|------------|-------|------|
| POST | `/v1/reports` | Admin | ✅ | `208`, `209`, `221` | UC-002, ADR-003, v2 payload | 24 |
| GET | `/v1/reports` | Admin | ✅ | `209` | list by period | 24 |
| GET | `/v1/reports/{id}` | Admin, Driver scoped | ✅ | `209` | — | 24 |
| GET | `/v1/reports/{id}/export` | Admin | ✅ | `209`, `236` | PDF/CSV/XLSX derived view | 42 |
| GET | `/v1/reports/{id}/verify` | 🔒 public rate limit | ✅ | `209` | ADR-007, BR-RE-002 | 24 |

---

## Audit (`audit.events`)

| Method | Path | Auth | Status | Migrations | Rules | Task |
|--------|------|------|--------|------------|-------|------|
| GET | `/v1/audit/events` | Admin | ✅ | `220` | read-only, RN-PAG3 | 26 |

---

## Authorization quick reference

| Role | Typical routes |
|------|----------------|
| **Admin** | users, commerces write, products, inventory adjust, orders approve/reject/cancel, reports generate, delivery assign |
| **Driver** | sales, deliveries own, commerces/products read, verify report |
| **Seller** | sales (field + own list), commerces/products read, commerce registration submit |
| **CommerceContact** | `/v1/portal/*` only (+ scoped media upload) |

---

**Updated:** 2026-07-05 (Phase 51 — Seller own sales list)

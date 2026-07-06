# API Contract — Driver/Seller Control System

> Version all public APIs from day one. Define OpenAPI before handlers.
> Base path: `/v1/`

---

## Base URL

```
https://api.example.com/v1
```

## Authentication

| Mechanism | Header / body |
|-----------|---------------|
| Access token | `Authorization: Bearer <jwt>` |
| Refresh token | `POST /v1/auth/refresh` with opaque token in body |
| Tenant context | Set via JWT claim + Postgres `app.tenant_id` (not client header in prod) |

## Error format (all endpoints)

```json
{
  "error": {
    "code": "INSUFFICIENT_STOCK",
    "message": "Human-readable safe message",
    "correlationId": "550e8400-e29b-41d4-a716-446655440000"
  }
}
```

RFC 9457 alignment — see `agent-rules/10-api-design/rest-conventions.md`.

## Pagination (list endpoints)

All `GET` collection routes use **cursor pagination** unless noted below.

| Param | Default | Max | Notes |
|-------|---------|-----|-------|
| `limit` | 20 | 100 | Page size |
| `cursor` | — | — | UUIDv7 of last item from previous page |

**Response envelope (cursor):**

```json
{
  "data": [ ... ],
  "pagination": {
    "next_cursor": "550e8400-e29b-41d4-a716-446655440000",
    "has_more": true,
    "limit": 20
  }
}
```

**Filters:** `filter[field]` or `filter[field][op]=value` — whitelist per route.  
**Sort:** `sort=-field,other` — whitelist per route (where supported).

**Validation errors:** `invalid_pagination`, `invalid_filter_field`, `invalid_sort_field` (+ `field`).

**Rust helpers:** `application::list_query` (VOs), `api_http::list_query` (parser + `build_cursor_page`).

### Offset exception — `GET /v1/reports` only

Admin report history supports arbitrary page jumps. Uses legacy offset params:

| Param | Default | Max |
|-------|---------|-----|
| `page` | 1 | — |
| `pageSize` | 20 | 50 |

**Response:** `{ "items": [...], "page", "pageSize", "total" }`

---

## Auth

### `POST /v1/auth/login`

- **Auth:** Public (rate limited)
- **Body (allow-list):** `{ "email", "password" }`
- **Response 200:** `{ "accessToken", "refreshToken", "expiresIn" }`
- **Response 401:** Invalid credentials
- **Response 429:** Rate limited

### `POST /v1/auth/refresh`

- **Body:** `{ "refreshToken" }`
- **Response 200:** New access + refresh tokens

### `POST /v1/auth/logout`

- **Auth:** Bearer JWT
- **Effect:** Revoke refresh token in Redis

---

## Users (Admin)

### `POST /v1/users`

- **Auth:** Admin
- **Body:** `{ "name", "email", "password", "role" }`
- **Response 201:** User (no password fields)

### `GET /v1/users`

- **Auth:** Admin
- **Query (cursor):** `limit` (default 20, max 100), `cursor`, `filter[active]`, `filter[role]` (whitelist: `Admin`, `Driver`, `Seller`, `CommerceContact`)
- **Response 200:** `{ "data": [...], "pagination": { "next_cursor", "has_more", "limit" } }`
- **Errors 400:** `invalid_pagination`, `invalid_filter_field`

### `GET /v1/users/{id}`

- **Auth:** Admin
- **Response 200 / 404:** `USER_NOT_FOUND`

### `PATCH /v1/users/{id}/deactivate`

- **Auth:** Admin
- **Effect:** Sets `active = false` (idempotent)
- **Response 200:** User (no password fields)

### `PUT /v1/users/{id}/driver-profile`

- **Auth:** Admin
- **Body:** `{ "cnhNumber", "cnhCategory", "cnhPhotoFileId?", "vehiclePlate", "vehicleModel", "vehicleCapacityKg?" }`
- **Precondition:** User role `Driver`
- **Response 200:** DriverProfile

### `PUT /v1/users/{id}/seller-profile`

- **Auth:** Admin
- **Body:** `{ "operatingRegion?", "monthlyTargetAmount?" }`
- **Precondition:** User role `Seller`
- **Response 200:** SellerProfile

---

## Commerces

### `POST /v1/commerces`

- **Auth:** Admin only (BR-IA-001)
- **Body:** `{ "cnpj", "legalName", "tradeName?", "address", "contact" }`
- **Response 201:** Commerce

### `GET /v1/commerces`

- **Auth:** Admin, Driver, Seller (read)
- **Query (cursor):** `limit`, `cursor`, `filter[active]` — filtered in SQL
- **Response 200:** `{ "data": [...], "pagination": { "next_cursor", "has_more", "limit" } }`

### `GET /v1/commerces/{id}`

- **Auth:** Admin, Driver, Seller
- **Response 200 / 404**

### `PATCH /v1/commerces/{id}/deactivate`

- **Auth:** Admin
- **Effect:** Sets `active = false` (BR-CO-002)
- **Response 200:** Commerce

### `GET /v1/commerces/{id}/addresses`

- **Auth:** Admin, Driver, Seller
- **Query (cursor):** `limit`, `cursor`
- **Response 200:** `{ "data": [ ... addresses ... ], "pagination": { "next_cursor", "has_more", "limit" } }`

### `POST /v1/commerces/{id}/addresses`

- **Auth:** Admin
- **Body:** `{ "type", "street", "number", "complement?", "district", "city", "state", "postalCode", "isPrimary?" }` — `type`: `"Billing"` \| `"Delivery"`
- **Response 201:** CommerceAddress

### `PATCH /v1/commerces/{id}/addresses/{addressId}`

- **Auth:** Admin
- **Body:** partial address fields + `isPrimary?`
- **Response 200:** CommerceAddress

### `PUT /v1/commerces/{id}/logo`

- **Auth:** Admin
- **Body:** `{ "fileId" }` — media file from `POST /v1/media/upload`
- **Response 200:** Commerce

---

## Products

### `POST /v1/products`

- **Auth:** Admin
- **Body:** `{ "name", "sku", "priceAmount", "priceCurrency", "categoryId?", "unitOfMeasure?" }`
- **Response 201:** Product detail (`categoryId`, `categoryName`, `categorySlug`, `unitOfMeasure`)
- **400:** `VALIDATION_ERROR` when legacy `category` string is sent — use `categoryId`

### `GET /v1/products`

- **Auth:** Admin, Driver, Seller
- **Query (cursor):** `limit` (default 20, max 100), `cursor?`, `filter[active]` (`true` | `false`; omit filter for all)
- **Response 200:** `{ "data": Product[], "pagination": { "next_cursor", "has_more", "limit" } }` — each item includes optional `categoryId`, `categoryName`, `categorySlug`
- **400:** `invalid_pagination` | `invalid_filter_field`

### `GET /v1/products/{id}`

- **Auth:** Admin, Driver, Seller
- **Response 200 / 404:** `PRODUCT_NOT_FOUND` — includes `categoryId`, `categoryName`, `categorySlug`

### `GET /v1/products/top-selling`

- **Auth:** Admin, Driver, Seller
- **Query:** `limit?` (default `5`, max `20` for ranked slice; validated against standard `limit` bounds)
- **Response 200:** Ranked list in cursor envelope — `{ "data": [{ "productId", "name", "sku", "unitsSold" }], "pagination": { "has_more": false, "limit" } }` — active products ranked by confirmed-sale units (see BR-SA-001)
- **Note:** Empty until at least one sale is **Confirmed**; `Pending` create does not increment totals

### `PATCH /v1/products/{id}`

- **Auth:** Admin
- **Body:** `{ "name?", "priceAmount?", "priceCurrency?", "active?", "categoryId?", "unitOfMeasure?" }` — `categoryId: null` clears assignment
- **Response 200:** Product detail

---

## Product categories (Phase 43)

### `GET /v1/categories`

- **Auth:** Admin
- **Query (cursor):** `limit`, `cursor?`, `filter[active]?`
- **Response 200:** Cursor envelope — categories (`id`, `name`, `slug`, `description?`, `sortOrder`, `active`, `imageFileId?`, `thumbUrl?`, `productCount?`)

### `POST /v1/categories`

- **Auth:** Admin
- **Body:** `{ "name", "description?", "sortOrder?", "active?", "slug?" }`
- **Response 201:** Category

### `GET /v1/categories/{id}`

- **Auth:** Admin
- **Response 200 / 404:** `CATEGORY_NOT_FOUND`

### `PATCH /v1/categories/{id}`

- **Auth:** Admin
- **Body:** `{ "name?", "description?", "sortOrder?", "active?", "slug?" }`
- **Response 200:** Category

### `DELETE /v1/categories/{id}`

- **Auth:** Admin
- **Effect:** Soft-deactivate (`active = false`); products keep assignment
- **Response 204**

### `POST /v1/categories/reorder`

- **Auth:** Admin
- **Body:** `{ "orderedIds": uuid[] }`
- **Response 204**

### `PUT /v1/categories/{id}/image`

- **Auth:** Admin
- **Body:** `{ "fileId" }` — media entity type `ProductCategory`
- **Response 200:** Category with `thumbUrl`

---

## Product images

### `GET /v1/products/{id}/images`

- **Auth:** Admin
- **Query (cursor):** `limit`, `cursor?`
- **Response 200:** `{ "data": ProductImage[], "pagination": { ... } }` — `id`, `fileId`, `isPrimary`, `sortOrder`
- **404:** `PRODUCT_NOT_FOUND` when product missing in tenant

### `POST /v1/products/{id}/images`

- **Auth:** Admin
- **Body:** `{ "fileId", "isPrimary?", "sortOrder?" }`
- **Response 201:** ProductImage

### `DELETE /v1/products/{id}/images/{imageId}`

- **Auth:** Admin
- **Response 204**

---

## Inventory

### `GET /v1/inventory/balances`

- **Auth:** Admin
- **Query (cursor):** `limit`, `cursor?`, `filter[name][like]?`, `filter[sku][like]?`
- **Response 200:** Cursor envelope — products with `balanceTotal`, `reserved`, and `available` (tenant pool minus active reservations)

### `GET /v1/inventory/products/{productId}/balance`

- **Auth:** Driver, Seller, Admin
- **Response 200:** `{ "productId", "available", "asOf" }`

### `POST /v1/inventory/movements`

- **Auth:** Admin (adjustments); system on sale confirm
- **Body:** `{ "productId", "movementType", "quantity", "reason?" }` — API accepts `movementType: "Adjustment"` only; `reason` required
- **Response 201:** StockMovement

### `GET /v1/inventory/products/{productId}/movements`

- **Auth:** Admin
- **Query (cursor):** `limit`, `cursor?`, `filter[created_at][gte]?`, `filter[created_at][lte]?` (RFC 3339)
- **Response 200:** Cursor envelope — StockMovement list (newest first; append-only audit read)

---

## Sales

### `POST /v1/sales`

- **Auth:** Driver, Seller, Admin
- **Body (allow-list):** `{ "commerceId", "driverId?", "items": [{ "productId", "quantity" }], "paymentMethod" }` — `paymentMethod`: `"cash"` \| `"pix"` \| `"credit"` \| `"debit"` (ADR-006)
- **driverId:** Required when caller is **Admin** (must reference an active Driver). Omitted for Driver/Seller — server uses JWT user. Rejected if Driver/Seller sends a value that does not match JWT user.
- **Idempotency:** `Idempotency-Key` header recommended
- **Response 201:** Sale in `Pending` status — **total computed server-side**

### `POST /v1/sales/{id}/confirm`

- **Auth:** Driver, Seller, Admin
- **Response 200:** Sale `Confirmed` + stock movement
- **Response 409:** Insufficient stock / invalid transition

### `POST /v1/sales/{id}/cancel`

- **Auth:** Driver, Seller, Admin
- **Precondition:** Sale status `Pending`
- **Response 200:** Sale `Cancelled`

### `GET /v1/sales/{id}`

- **Auth:** Driver (own), Seller (own), Admin (all)
- **Response 200 / 404**

### `GET /v1/sales`

- **Auth:** Admin, Driver (own), Seller (own)
- **Query (cursor):** `limit`, `cursor`, `filter[commerce_id]`, `filter[driver_id]` (Admin only; ignored for Driver/Seller), `filter[status]`, `filter[created_at][gte]`, `filter[created_at][lte]`
- **Response 200:** `{ "data": [...], "pagination": { "next_cursor", "has_more", "limit" } }`
- **Errors 400:** `invalid_pagination`, `invalid_filter_field`

### `POST /v1/sales/{id}/declare-payment`

- **Auth:** Driver (must match `sale.driver_id` — RN-PAG2)
- **Body:** `{ "method", "received", "notes?" }` — `method`: declared payment method string
- **Response 200:** Sale with declaration fields
- **Response 403:** `UNAUTHORIZED_PAYMENT_DECLARATION`

---

## Portal — Products (Phase 14)

### `GET /v1/public/products`

- **Auth:** none (public catalog)
- **Tenant:** `PUBLIC_CATALOG_TENANT_ID` env, or dev seed tenant in local environments
- **Query (cursor):** `limit`, `cursor`, `filter[category_slug]` (category **slug**)
- **Response 200:** `{ "data": [...], "pagination": { "next_cursor", "has_more", "limit" } }` — active products with `categoryId`, `categoryName`, `categorySlug`, optional `primaryImageUrl`

### `GET /v1/public/products/{id}`

- **Auth:** none
- **Tenant:** same as public product list
- **Response 200:** Active product detail with `unitOfMeasure`, `primaryImageUrl`, `imageUrls[]`, optional `description`
- **Response 404:** inactive or unknown product

### `GET /v1/public/media/{id}/content`

- **Auth:** none
- **Precondition:** file is a primary image of an **active** product in the public catalog tenant
- **Response 200:** image bytes (`image/*`)

### `GET /v1/public/catalog/events`

- **Auth:** none
- **Content-Type:** `text/event-stream`
- **Event:** `catalog.changed` — emitted when admin mutates products, categories, or product images
- **Heartbeat:** `: ping` every 25s

### `GET /v1/portal/products`

- **Auth:** CommerceContact only
- **Query (cursor):** `limit`, `cursor`, `filter[category_slug]` (category **slug**)
- **Response 200:** Same cursor envelope as public products

### `GET /v1/portal/products/{id}`

- **Auth:** CommerceContact only
- **Response 200:** Active product detail with `unitOfMeasure`, `primaryImageUrl`, `imageUrls[]`, optional `description`
- **Response 404:** inactive or unknown product

### `GET /v1/public/categories`

- **Auth:** none
- **Query (cursor):** `limit`, `cursor`
- **Response 200:** `{ "data": [...], "pagination": { "next_cursor", "has_more", "limit" } }` — active categories ordered by `sortOrder`

### `GET /v1/public/categories/{slug}`

- **Auth:** none
- **Query (cursor on nested products):** `limit`, `cursor`
- **Response 200:** Category fields + `products[]` + `pagination` (cursor envelope for products)

### `GET /v1/portal/categories`

- **Auth:** CommerceContact
- **Query (cursor):** `limit`, `cursor`
- **Response 200:** Same cursor envelope as public categories

### `GET /v1/portal/categories/{slug}`

- **Auth:** CommerceContact
- **Query (cursor on nested products):** `limit`, `cursor`
- **Response 200:** Category + `products[]` + `pagination`

---

## Portal — Orders (Phase 14)

### `GET /v1/portal/orders`

- **Auth:** CommerceContact (JWT `commerceId` — RLS scoped)
- **Query (cursor):** `limit`, `cursor`, `filter[status]`
- **Response 200:** `{ "data": [...], "pagination": { "next_cursor", "has_more", "limit" } }` — orders for contact's commerce only

### `GET /v1/portal/orders/{id}`

- **Auth:** CommerceContact
- **Response 200 / 404**

### `POST /v1/portal/orders`

- **Auth:** CommerceContact
- **Body:** `{ "deliveryAddressId", "notes?", "items": [{ "productId", "quantity" }] }`
- **Response 201:** Order `Draft` — totals computed server-side

### `PUT /v1/portal/orders/{id}`

- **Auth:** CommerceContact
- **Precondition:** `Draft` only
- **Body:** same as create
- **Response 200:** Updated draft

### `DELETE /v1/portal/orders/{id}`

- **Auth:** CommerceContact
- **Precondition:** `Draft` only
- **Response 204:** Draft cancelled

### `POST /v1/portal/orders/{id}/submit`

- **Auth:** CommerceContact
- **Precondition:** `Draft`, non-empty items
- **Response 200:** `PendingApproval`

---

## Admin — Orders (Phase 14)

### `GET /v1/orders`

- **Auth:** Admin
- **Query (cursor):** `limit`, `cursor`, `filter[status]`, `filter[commerce_id]`, `filter[created_at][gte]`, `filter[created_at][lte]`
- **Response 200:** `{ "data": [...], "pagination": { "next_cursor", "has_more", "limit" } }`

### `GET /v1/orders/{id}`

- **Auth:** Admin
- **Response 200:** Order detail + optional delivery summary
- **Response 404:** `ORDER_NOT_FOUND`

### `POST /v1/orders/{id}/approve`

- **Auth:** Admin
- **Precondition:** `PendingApproval`
- **Response 200:** `Approved` + stock reservations (RN2)
- **Response 409:** `INSUFFICIENT_STOCK` or `INVALID_ORDER_TRANSITION`

### `POST /v1/orders/{id}/reject`

- **Auth:** Admin
- **Body:** `{ "reason": string }` — required (RN10)
- **Response 200:** `Rejected`
- **Response 400:** `REJECTION_REASON_REQUIRED`

### `POST /v1/orders/{id}/cancel`

- **Auth:** Admin
- **Precondition:** Approved or Picking (not InTransit — RN6)
- **Response 200:** `Cancelled` + reservations released
- **Response 409:** `INVALID_ORDER_TRANSITION`

### `POST /v1/orders/{id}/start-picking`

- **Auth:** Admin
- **Precondition:** `Approved`
- **Response 200:** `Picking`
- **Response 409:** `INVALID_ORDER_TRANSITION`

---

## Deliveries

### `POST /v1/orders/{id}/delivery`

- **Auth:** Admin
- **Body:** `{ "driverId" }`
- **Precondition:** Order `Approved` or `Picking`; no existing delivery (1:1)
- **Response 201:** Delivery in `Waiting` status

### `GET /v1/deliveries`

- **Auth:** Admin (all); Driver (own via RLS)
- **Query (cursor):** `limit`, `cursor`, `filter[status]`, `filter[created_at][gte]`, `filter[created_at][lte]`
- **Response 200:** `{ "data": [...], "pagination": { "next_cursor", "has_more", "limit" } }`

### `GET /v1/deliveries/{id}`

- **Auth:** Admin; Driver (assigned)
- **Response 200 / 404:** `DELIVERY_NOT_FOUND`
- **Response body:** delivery fields plus `orderItems[]` (`id`, `productId`, `quantity`) for driver confirm UI

### `POST /v1/deliveries/{id}/start-transit`

- **Auth:** Assigned Driver
- **Precondition:** Order must be in `Picking` (call `POST /v1/orders/{id}/start-picking` after approve)
- **Effect:** Delivery → `InTransit`; order → `InTransit`
- **Response 200:** Delivery
- **Response 409:** `INVALID_DELIVERY_TRANSITION` or `INVALID_ORDER_TRANSITION`

### `POST /v1/deliveries/{id}/confirm`

- **Auth:** Assigned Driver
- **Body:** `{ "proofFileId", "items": [{ "orderItemId", "quantityDelivered" }], "latitude?", "longitude?", "receivedByName?" }`
- **Precondition:** `InTransit`; proof required (RN4)
- **Response 200:** Delivery `Delivered` + `saleId` on response
- **Response 422:** `PROOF_REQUIRED`

---

## Site settings

### `GET /v1/settings`

- **Auth:** Any authenticated role in tenant
- **Response 200:** `{ "displayName", "logoFileId?", "logoUrl?", "salesContactPhone?" }` — `logoUrl` is S3 presigned or `/v1/media/{fileId}/content` in local dev

### `GET /v1/public/settings`

- **Auth:** none (public catalog tenant)
- **Response 200:** `{ "displayName", "salesContactPhone?" }` — branding subset for guest portal

### `PATCH /v1/settings`

- **Auth:** Admin
- **Body:** `{ "displayName?", "salesContactPhone?" }` — phone: digits only after normalize, 10–15 chars; empty string clears
- **Response 200:** Updated settings

### `PUT /v1/settings/logo`

- **Auth:** Admin
- **Body:** `{ "fileId" }` — upload via `POST /v1/media/upload` with `entityType: "Tenant"` first
- **Response 200:** Updated settings with browser-loadable `logoUrl`

---

## Media

### `POST /v1/media/upload`

- **Auth:** Admin; Driver/Seller/CommerceContact (entity-scoped — see BUSINESS-RULES matrix)
- **Content-Type:** `multipart/form-data` — fields: `file`, `entityType`, `entityId`
- **Response 201:** `{ "id", "entityType", "entityId", "mimeType", "sizeBytes", "sha256" }`
- **Response 400:** `INVALID_MIME`, `FILE_TOO_LARGE`

### `GET /v1/media/{id}/url`

- **Auth:** Same scope as upload for the file's entity
- **Response 200:** `{ "url", "expiresAt" }` — presigned URL (~15 min TTL)
- **Response 404:** `MEDIA_NOT_FOUND`

---

## Reports

### `POST /v1/reports`

- **Auth:** Admin
- **Body:** `{ "reportType", "periodStart", "periodEnd", "driverId?", "commerceId?" }` — `reportType`: `"DailyDriver"` \| `"CommercePeriod"` \| `"Consolidated"`
- **Response 201:** Report with `id`, `signature`, `publicKeyId`

### `GET /v1/reports`

- **Auth:** Admin
- **Query (offset — see [Offset exception](#offset-exception--get-v1reports-only)):** `page`, `pageSize`
- **Response 200:** `{ "items": Report[], "page", "pageSize", "total" }`

### `GET /v1/reports/{id}`

- **Auth:** Admin; Driver (when `driverId` in canonical payload matches JWT user)
- **Response 200:** Report metadata + payload
- **Response 404:** `REPORT_NOT_FOUND`

### `GET /v1/reports/{id}/export`

- **Auth:** Admin
- **Query:** `format=pdf|csv|xlsx`
- **Response 200:** File stream (`Content-Disposition: attachment; filename="report-{type}-{periodStart}.{ext}"`)
- **Response 400:** `UNSUPPORTED_FORMAT`
- **Response 404:** `REPORT_NOT_FOUND`

Derived from signed canonical JSON — verification remains on `GET …/verify`.

### `GET /v1/reports/{id}/verify`

- **Auth:** Public (rate limited by IP — ADR-007)
- **Response 200:** `{ "valid": true | false, "reportId" }`
- **Response 429:** `RATE_LIMITED`

---

## Audit

### `GET /v1/audit/events`

- **Auth:** Admin
- **Query (cursor):** `limit`, `cursor`, `filter[actor_id]`, `filter[action]`, `filter[created_at][gte]`, `filter[created_at][lte]`
- **Response 200:** `{ "data": [...], "pagination": { "next_cursor", "has_more", "limit" } }` — append-only audit events (`audit.events`), newest first
- **Response 403:** Non-admin roles

---

## Health

### `GET /health`

- **Auth:** None
- **Response 200:** `{ "status": "ok" }`

---

## OpenAPI

Full schema: [`docs/openapi.yaml`](openapi.yaml) — all implemented `/v1/*` routes (Phases 16–26).

### Example: create sale

```http
POST /v1/sales
Authorization: Bearer <jwt>
Content-Type: application/json
Idempotency-Key: 550e8400-e29b-41d4-a716-446655440000

{
  "commerceId": "0192a1b2-c3d4-7890-abcd-ef1234567890",
  "items": [{ "productId": "0192a1b2-c3d4-7890-abcd-ef1234567891", "quantity": 2 }],
  "paymentMethod": "cash"
}
```

**Response 201:**

```json
{
  "id": "0192a1b2-c3d4-7890-abcd-ef1234567892",
  "commerceId": "0192a1b2-c3d4-7890-abcd-ef1234567890",
  "driverId": "0192a1b2-c3d4-7890-abcd-ef1234567893",
  "status": "Pending",
  "paymentMethod": "Cash",
  "totalAmount": 2000,
  "totalCurrency": "BRL",
  "items": [
    {
      "productId": "0192a1b2-c3d4-7890-abcd-ef1234567891",
      "quantity": 2,
      "unitPriceAmount": 1000,
      "unitPriceCurrency": "BRL",
      "lineTotalAmount": 2000
    }
  ]
}
```

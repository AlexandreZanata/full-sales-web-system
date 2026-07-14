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

## Testing

Route coverage is gated by Phase 17:

- Drift: `pnpm verify:api-route-inventory` (`API-CONTRACT.md` ↔ `routes.rs`)
- Markers: `pnpm verify:route-contract-manifest` (every `T-17-*` ID has an `api-http` test marker)
- Strategy: [TESTING-STRATEGY.md](TESTING-STRATEGY.md)

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

### `GET /v1/commerces/cnpj-lookup`

- **Auth:** Seller (submit), Admin / users with review privilege
- **Query:** `cnpj` (digits, validated BR-CO-001)
- **Response 200:** Normalized lookup payload (`cnpj`, `legalName`, `tradeName`, `address`, optional `phone`, `email`, `registrationStatus`, `mainCnae`, `partners`, `upstreamSnapshot`, `provider`, `fetchedAt`)
- **Upstream:** Configurable via `CNPJ_LOOKUP_PROVIDER` (`brasilapi` default, `opencnpj`, `mock` for tests). Response shape unchanged.
- **400:** `INVALID_CNPJ`
- **404:** `CNPJ_NOT_FOUND`
- **502:** `CNPJ_LOOKUP_UNAVAILABLE` (client may fall back to manual entry)
- **429:** `RATE_LIMITED`

### `POST /v1/commerces/registrations`

- **Auth:** Seller (BR-CO-010)
- **Body:** `{ "cnpj", "legalName", "tradeName?", "contact", "deliveryAddress", "registrationMode", "lookupSnapshot?" }`
- **Response 201:** Commerce registration (`registrationStatus: PendingReview`, `active: true` — visible in catalog; admin may reject or deactivate)
- **409:** `CNPJ_ALREADY_REGISTERED`

### `GET /v1/commerces/registrations`

- **Auth:** Seller (own submissions) / Admin or `can_review_commerce` (all)
- **Query (cursor):** `limit`, `cursor`, `filter[status]` (`PendingReview` | `Active` | `Rejected`)
- **Response 200:** Cursor list of registration resources

### `GET /v1/commerces/registrations/{id}`

- **Auth:** Submit owner or reviewer
- **Response 200 / 404**

### `PATCH /v1/commerces/registrations/{id}`

- **Auth:** Submit owner while `PendingReview`, or reviewer
- **Body:** partial `legalName`, `tradeName`, `contact`
- **Response 200:** Updated registration

### `POST /v1/commerces/registrations/{id}/approve`

- **Auth:** Admin or `can_review_commerce` (BR-CO-011)
- **Response 200:** `registrationStatus: Active`, `active: true`

### `POST /v1/commerces/registrations/{id}/reject`

- **Auth:** Reviewer
- **Body:** `{ "reason" }` (required, non-empty)
- **Response 200:** `registrationStatus: Rejected`

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
- **Body:** `{ "name?", "priceAmount?", "priceCurrency?", "active?", "categoryId?", "unitOfMeasure?", "description?", "isFeatured?" }` — `categoryId: null` clears assignment
- **Response 200:** Product detail (includes `isFeatured`)

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

## Portal — Home content (Phase 71)

### `GET /v1/public/banners`

- **Auth:** none
- **Tenant:** `PUBLIC_CATALOG_TENANT_ID` env, or dev seed tenant locally
- **Query:** `placement?` (default `hero`), `limit?` (default `10`, max `20`)
- **Response 200:** `{ "data": [{ "id", "imageUrl", "linkUrl?", "altText?" }], "pagination": { "has_more": false, "limit" } }`

### `GET /v1/public/promotions`

- **Auth:** none
- **Tenant:** same as public product list
- **Query:** `limit?` (default `4`, max `20`)
- **Response 200:** `{ "data": [{ "id", "headline", "discountText", "background", "categorySlug?", "linkUrl?", "imageUrl?" }], "pagination": { "has_more": false, "limit" } }` — `background`: `yellow` \| `green`

### `GET /v1/public/products/featured`

- **Auth:** none
- **Tenant:** same as public product list
- **Query:** `limit?` (default `12`, max `50`)
- **Response 200:** Cursor envelope of active products where `is_featured = true`, same fields as public product list

### `GET /v1/public/products/popular`

- **Auth:** none
- **Tenant:** same as public product list
- **Query:** `limit?` (default `12`, max `50`)
- **Response 200:** Cursor envelope of active products ranked by confirmed-sale units; falls back to catalog list order when no sales totals exist

### `GET /v1/portal/banners`

- **Auth:** Admin
- **Query:** `limit?` (default `50`, max `100`)
- **Response 200:** `{ "data": [{ "id", "placement", "imageFileId", "linkUrl?", "altText?", "sortOrder", "active" }], "pagination": { "has_more": false, "limit" } }`

### `POST /v1/portal/banners`

- **Auth:** Admin
- **Body:** `{ "placement?", "imageFileId", "linkUrl?", "altText?", "sortOrder?", "active?" }` — `imageFileId` references `media.files` with `entityType: PortalBanner`
- **Response 201:** Banner row

### `PATCH /v1/portal/banners/{id}`

- **Auth:** Admin
- **Body:** partial banner fields (`linkUrl` / `altText` may be set to `null`)
- **Response 200:** Banner row
- **Response 404:** `BANNER_NOT_FOUND`

### `DELETE /v1/portal/banners/{id}`

- **Auth:** Admin
- **Response 204**
- **Response 404:** `BANNER_NOT_FOUND`

### `GET /v1/portal/promotions`

- **Auth:** Admin
- **Query:** `limit?` (default `50`, max `100`)
- **Response 200:** `{ "data": [{ "id", "headline", "discountText", "background", "categorySlug?", "linkUrl?", "imageFileId?", "sortOrder", "active" }], "pagination": { "has_more": false, "limit" } }`

### `POST /v1/portal/promotions`

- **Auth:** Admin
- **Body:** `{ "headline", "discountText", "background", "categorySlug?", "linkUrl?", "imageFileId?", "sortOrder?", "active?" }` — `background`: `yellow` \| `green`
- **Response 201:** Promotion row

### `PATCH /v1/portal/promotions/{id}`

- **Auth:** Admin
- **Body:** partial promotion fields
- **Response 200:** Promotion row
- **Response 404:** `PROMOTION_NOT_FOUND`

### `DELETE /v1/portal/promotions/{id}`

- **Auth:** Admin
- **Response 204**
- **Response 404:** `PROMOTION_NOT_FOUND`

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

### `GET /v1/media/{id}/content`

- **Auth:** Same scope as upload for the file's entity
- **Response 200:** Binary body with content-type from stored media
- **Response 404:** `MEDIA_NOT_FOUND`

**Note:** Public catalog uses `GET /v1/public/media/{id}/content` without JWT.

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
- **Response 200:** `{ "data": [AuditEvent], "pagination": { ... } }` — append-only audit events (`audit.events`), newest first; max **90-day** date range per query
- **Response 403:** Non-admin roles

**AuditEvent fields:** `id`, `tenantId?`, `actorId`, `actorType`, `action`, `resourceType`, `resourceId`, `metadata?`, `ip?`, `correlationId?`, `createdAt`

**Implemented:** Phase 10.

### `GET /v1/fraud/alerts`

- **Auth:** Admin
- **Query:** `limit` (default 20, max 50)
- **Response 200:** Recent fraud flags for the tenant (RLS-scoped array)
- **Response 403:** Non-admin roles

**Implemented:** Phase 6 — high-severity alerts emit log-only email stub.

---

## Health

### `GET /health`

- **Auth:** None
- **Response 200:** `{ "status": "ok" }`

### `GET /health/ready`

- **Auth:** None
- **Response 200:** `{ "status": "ready", "components": { "postgres", "redis", "minio" } }` — each with `status`, optional `latencyMs`, optional `details`
- **Response 503:** `{ "status": "not_ready", ... }` when any configured critical dependency is down
- **Critical deps:** Postgres always; Redis when `REDIS_URL` set; MinIO when `STORAGE_*` set

**Implemented:** Phase 9.

### `GET /v1/status`

- **Auth:** None (public aggregated status — no secrets)
- **Response 200:** `{ "status": "operational" | "degraded", "components": { "api", "portal", "payments", "storage" } }`
- **Response 503:** Overall `status: "down"`

**Implemented:** Phase 9.

### `GET /v1/`

- **Auth:** None
- **Response 200:** `{ "version": "1", "status": "ok" }` — version stub (reserved for future metadata)

**Implemented:** Phase 1.

---

## Platform SaaS (Phases 1–13)

> **Status:** Implemented — auth, lifecycle, billing, fraud, domains, operations, health, audit, UIs, integration suite.  
> ADRs: [ADR-013](adr/ADR-013-platform-admin-identity.md) … [ADR-018](adr/ADR-018-tenant-asaas-payments.md).  
> E2E: [features/platform-saas-e2e.md](features/platform-saas-e2e.md).

### Platform error codes (additions)

| Code | HTTP | When |
|------|------|------|
| `TENANT_SUSPENDED` | 403 | Tenant suspended — mutating API blocked (BR-PL-001) |
| `SUBSCRIPTION_PAST_DUE` | 402 | Billing action required before operation |
| `DOMAIN_NOT_VERIFIED` | 409 | Domain not in `Active` state |
| `FRAUD_BLOCKED` | 403 | Velocity or blocklist (BR-FR-001, BR-FR-002) |

---

## Platform auth (`/v1/platform/*`)

### `POST /v1/platform/auth/login`

- **Auth:** Public
- **Body:** `{ "email", "password" }`
- **Response 200:** `{ "mfaRequired": true, "mfaToken": "..." }` or tokens if MFA already satisfied
- **Response 401:** Invalid credentials

### `POST /v1/platform/auth/mfa/verify`

- **Auth:** `mfaToken` from login step
- **Body:** `{ "code" }`
- **Response 200:** `{ "accessToken", "refreshToken", "expiresIn" }`

### `POST /v1/platform/auth/refresh`

- **Auth:** Public (refresh token in body)
- **Body:** `{ "refreshToken" }`
- **Response 200:** New platform access + refresh tokens
- **Response 401:** Invalid or revoked refresh token

### `POST /v1/platform/auth/logout`

- **Auth:** PlatformAdmin (Bearer)
- **Body:** `{ "refreshToken" }` (optional — revokes refresh when present)
- **Response 204:** Session revoked

---

## Platform tenants

### `GET /v1/platform/tenants`

- **Auth:** PlatformAdmin (RLS bypass — ADR-016)
- **Query (cursor):** `limit`, `cursor`, `filter[status]`, `filter[plan_id]`, `filter[display_name][contains]`, `sort=-created_at`
- **Response 200:** Cursor envelope — tenant summary rows

### `POST /v1/platform/tenants`

- **Auth:** PlatformAdmin
- **Body:** `{ "legalName", "displayName", "planId", "adminEmail", "cnpj" }`
- **Response 201:** Tenant + provisioning job id
- **Response 403:** Non-PlatformAdmin

### `GET /v1/platform/tenants/{id}`

- **Auth:** PlatformAdmin
- **Response 200:** Full tenant aggregate

### `PATCH /v1/platform/tenants/{id}`

- **Auth:** PlatformAdmin
- **Body:** `{ "status"?, "planId"?, "trialEndsAt"?, "graceExtendedUntil"?, "suspendedReason"?, "settings"? }`
- **Response 200:** Updated tenant
- **Response 409:** `InvalidTenantTransition`

### `POST /v1/platform/tenants/{id}/suspend`

- **Auth:** PlatformAdmin
- **Body:** `{ "reason" }` (required, min 3 chars)
- **Response 200:** Updated tenant with `status: Suspended`

### `POST /v1/platform/tenants/{id}/reactivate`

- **Auth:** PlatformAdmin
- **Response 200:** Updated tenant with `status: Active`

### `POST /v1/platform/tenants/{id}/offboard`

- **Auth:** PlatformAdmin
- **Response 200:** Tenant `status: Offboarding`, `offboardingScheduledAt` set

### `POST /v1/platform/jobs/offboarding`

- **Auth:** PlatformAdmin
- **Response 200:** `{ "processed": [...], "lgpdExport": "stub" }` — anonymizes PII after 90-day retention

---

## Platform users

### `GET /v1/platform/users`

- **Auth:** PlatformAdmin
- **Query (cursor):** `limit`, `cursor`, `filter[tenant_id]`, `filter[role]`, `filter[active]`, `filter[email][prefix]`, `sort` (`createdAt` | `email` | `name`)
- **Response 200:** Cross-tenant user list with `tenant` summary (no password fields)

**Implemented:** Phase 8.

### `GET /v1/platform/users/{id}`

- **Auth:** PlatformAdmin
- **Response 200:** User detail including `lastLoginAt` when tracked

**Implemented:** Phase 8.

### `PATCH /v1/platform/users/{id}`

- **Auth:** PlatformAdmin
- **Body:** `{ "role"?: "Admin" | "Driver" | "Seller" }`
- **Response 200:** Updated user
- **Response 400:** `LAST_ADMIN_REQUIRED` when demoting/disabling sole Admin
- **Audit:** `user.patch`

**Implemented:** Phase 8.

### `POST /v1/platform/users/{id}/reset-password`

- **Auth:** PlatformAdmin
- **Response 202:** `{ "queued": true, "temporaryPassword" }` — ponytail: email queue stub; password rotated immediately
- **Audit:** `user.reset_password`

**Implemented:** Phase 8.

### `POST /v1/platform/users/{id}/disable`

- **Auth:** PlatformAdmin
- **Response 200:** `{ "active": false }`
- **Audit:** `user.disable`

**Implemented:** Phase 8.

### `POST /v1/platform/users/{id}/enable`

- **Auth:** PlatformAdmin
- **Response 200:** `{ "active": true }`
- **Audit:** `user.enable`

**Implemented:** Phase 8.

---

## Platform workforce and support

### `GET /v1/platform/tenants/{id}/users`

- **Auth:** PlatformAdmin
- **Query (cursor):** `limit`, `cursor`
- **Response 200:** All employees for tenant

**Implemented:** Phase 8.

### `GET /v1/platform/tenants/{id}/stats`

- **Auth:** PlatformAdmin
- **Response 200:** `{ "users", "drivers", "sellers", "commerces", "orders", "mrrMinor", "mrrCurrency" }`

**Implemented:** Phase 8.

### `GET /v1/platform/tenants/{id}/orders`

- **Auth:** PlatformAdmin (read-only)
- **Query:** Same cursor filters as tenant `GET /v1/orders`
- **Audit:** `support.orders.list`

**Implemented:** Phase 8.

### `GET /v1/platform/tenants/{id}/sales`

- **Auth:** PlatformAdmin (read-only)
- **Audit:** `support.sales.list`

**Implemented:** Phase 8.

### `GET /v1/platform/tenants/{id}/products`

- **Auth:** PlatformAdmin (read-only)
- **Audit:** `support.products.list`

**Implemented:** Phase 8.

### `POST /v1/platform/maintenance`

- **Auth:** PlatformAdmin
- **Body:** `{ "tenantId"?, "message", "startsAt", "endsAt" }` — omit `tenantId` for global window
- **Response 201:** Maintenance window
- **Middleware:** `503 MAINTENANCE` on tenant/public routes during active window (except `/health`, `/v1/platform/*`, `GET /v1/settings`)
- **Settings:** `maintenanceBanner` on `GET /v1/settings` and `GET /v1/public/settings` when applicable
- **Audit:** `maintenance.schedule`

**Implemented:** Phase 8.

### `PATCH /v1/platform/tenants/{id}/features`

- **Auth:** PlatformAdmin
- **Body:** `{ "onlinePayments"?, "customDomain"?, "apiRateTier"?: "standard" | "pro" | "enterprise" }`
- **Response 200:** Resolved effective flags (plan defaults + `settings.feature_flags` overrides)
- **Audit:** `tenant.features.patch`

**Implemented:** Phase 8 — defaults per plan in `billing.plans.feature_limits`; overrides in `shared.tenants.settings.feature_flags`.

---

## Platform impersonation

### `POST /v1/platform/impersonate`

- **Auth:** PlatformAdmin
- **Body:** `{ "tenantId", "reason" }`
- **Response 201:** `{ "impersonationToken", "expiresAt", "tenantId" }` — 15 min TTL (BR-AU-001)
- **Response 403:** Impersonation disabled or fraud block

### `POST /v1/platform/impersonate/end`

- **Auth:** PlatformAdmin (impersonation or platform Bearer)
- **Response 204:** Impersonation session ended

---

## Platform operations

### `GET /v1/platform/health/matrix`

- **Auth:** PlatformAdmin
- **Response 200:** `{ "probes": { "postgres", "redis", "minio", "asaas", "dns", "webhook_queue" } }` — each with `status`, `latencyMs`, `checkedAt`, `uptime24hPct`, optional `details`

**Implemented:** Phase 9.

### `GET /v1/platform/health/history`

- **Auth:** PlatformAdmin
- **Query:** `probe` (required), `since` (RFC3339, required)
- **Response 200:** `{ "probe", "data": [{ "status", "latencyMs", "checkedAt", "details" }] }`

**Implemented:** Phase 9.

### `GET /v1/platform/fraud/events`

- **Auth:** PlatformAdmin
- **Query (cursor):** `limit`, `cursor`, `filter[status]`, `filter[severity]`, `filter[tenant_id]`
- **Response 200:** `{ "data": [FraudEvent], "pagination": { ... } }`

**Implemented:** Phase 6.

### `POST /v1/platform/fraud/events/{id}/resolve`

- **Auth:** PlatformAdmin
- **Body:** `{ "resolution": "blocked" | "whitelisted" | "dismissed", "note" }`
- **Response 200:** Updated event

**Implemented:** Phase 6.

### `POST /v1/platform/blocklist`

- **Auth:** PlatformAdmin
- **Body:** `{ "email"?, "cnpj"?, "ip"?, "cardFingerprint"?, "reason", "expiresAt"? }`
- **Response 201:** Blocklist entry

**Implemented:** Phase 6.

### `DELETE /v1/platform/blocklist/{id}`

- **Auth:** PlatformAdmin
- **Response 204:** Entry removed
- **Response 404:** Entry not found

**Implemented:** Phase 6.

### `GET /v1/platform/domains`

- **Auth:** PlatformAdmin
- **Query (cursor):** `limit`, `cursor`, `filter[status]`, `filter[tenant_id]`
- **Response 200:** All tenant domains

**Implemented:** Phase 7.

### `PATCH /v1/platform/domains/{id}`

- **Auth:** PlatformAdmin
- **Body:** `{ "status"?: "Active" | "Detached", "isPrimary"?: true }`
- **Response 200:** Updated domain
- **Audit:** `domain.patch`

**Implemented:** Phase 7.

### `POST /v1/platform/domains/{id}/force-verify`

- **Auth:** PlatformAdmin
- **Response 200:** Domain → `Verified` then `Active`
- **Audit:** `domain.force_verify`

**Implemented:** Phase 7.

### `POST /v1/platform/jobs/domain-verification`

- **Auth:** PlatformAdmin
- **Response 200:** `{ "verified": [ domainId ], "failed": [ domainId ] }` — DNS TXT poll for all `Verifying` domains

**Implemented:** Phase 7 — schedule every 5 min (ADR-017).

### `GET /v1/platform/audit/events`

- **Auth:** PlatformAdmin
- **Query (cursor):** `limit`, `cursor`, `filter[tenant_id]`, `filter[actor_id]`, `filter[action]`, `filter[created_at][gte/lte]` — max 90-day range
- **Response 200:** Cross-tenant audit log

**Implemented:** Phase 10.

### `POST /v1/platform/tenants/{id}/export`

- **Auth:** PlatformAdmin
- **Response 202:** Export job — LGPD JSON bundle ZIP (users, commerces, orders, sales; no secrets)
- **Audit:** `tenant.export.requested`

**Implemented:** Phase 10.

### `GET /v1/platform/tenants/{id}/export/{jobId}`

- **Auth:** PlatformAdmin
- **Response 200:** Job status + presigned `downloadUrl` when completed

**Implemented:** Phase 10.

### `POST /v1/settings/data-export`

- **Auth:** Tenant Admin
- **Response 202:** Export job for own tenant

**Implemented:** Phase 10.

### `GET /v1/settings/data-export/{jobId}`

- **Auth:** Tenant Admin
- **Response 200:** Job status + presigned URL

**Implemented:** Phase 10.

---

## Billing (`/v1/billing/*`)

### `POST /v1/billing/webhooks/asaas`

- **Auth:** `asaas-access-token` header must match `ASAAS_WEBHOOK_TOKEN` (constant-time compare)
- **Body:** Asaas event envelope — `{ "id", "event", "payment"?, "subscription"?, "invoice"? }`
- **Response 200:** `{ "received": true }` or `{ "received": true, "duplicate": true }` — idempotent on `id` (BR-BI-001)
- **Response 401:** `WEBHOOK_UNAUTHORIZED`

**Implemented:** Phase 4 — persists to `billing.payment_events`; applies tenant/subscription/invoice side effects for `PAYMENT_CONFIRMED`, `PAYMENT_OVERDUE`, `SUBSCRIPTION_DELETED`.

**Handled events (v1):** `PAYMENT_CREATED`, `PAYMENT_CONFIRMED`, `PAYMENT_RECEIVED`, `PAYMENT_OVERDUE`, `PAYMENT_DELETED`, `PAYMENT_REFUNDED`, `SUBSCRIPTION_CREATED`, `SUBSCRIPTION_UPDATED`, `SUBSCRIPTION_DELETED`, `INVOICE_CREATED`, `INVOICE_UPDATED`, `INVOICE_AUTHORIZED`, `INVOICE_CANCELED`

### `GET /v1/billing/subscription`

- **Auth:** Tenant Admin
- **Response 200:** `{ "plan", "status", "tenantStatus", "currentPeriodEnd", "trialEndsAt" }`
- **Response 402:** `SUBSCRIPTION_PAST_DUE`

**Implemented:** Phase 4.

### `GET /v1/billing/invoices`

- **Auth:** Tenant Admin
- **Query (cursor):** `limit`, `cursor`, `filter[status]`, `sort=-due_date`
- **Response 200:** Invoice history mirrored from Asaas

**Implemented:** Phase 4 — cursor list with RLS.

### `GET /v1/billing/invoices/{id}`

- **Auth:** Tenant Admin
- **Response 200:** `{ "id", "amountMinor", "currency", "dueDate", "status", "paidAt", "pdfUrl"? }`

**Implemented:** Phase 4.

### `POST /v1/billing/payment-methods`

- **Auth:** Tenant Admin
- **Body:** `{ "type": "credit_card", "creditCardToken" }` — token from Asaas.js tokenization
- **Response 201:** Payment method attached

**Implemented:** Phase 4 — attaches token to Asaas customer.

### `POST /v1/platform/jobs/dunning`

- **Auth:** PlatformAdmin
- **Response 200:** `{ "processed": [...], "emailNotifications": "stub" }` — suspends `PastDue` tenants after 7-day grace (BR-BI-003, ADR-014)

### `POST /v1/billing/subscription/cancel`

- **Auth:** Tenant Admin
- **Response 202:** `{ "status": "Cancelled", "cancelAtPeriodEnd": true }` — gateway cancel when `asaas_subscription_id` present; local status → `Cancelled`
- **Response 400:** `INVALID_SUBSCRIPTION_STATUS` when not `Active`/`PastDue`
- **Response 404:** No subscription for tenant

**Implemented:** Phase 17J.

---

## Settings — domains and payments

### `GET /v1/settings/domains`

- **Auth:** Tenant Admin
- **Response 200:** `{ "data": [ TenantDomain ] }`

**Implemented:** Phase 7.

### `POST /v1/settings/domains`

- **Auth:** Tenant Admin (Pro+ plan)
- **Body:** `{ "hostname" }`
- **Response 201:** Domain + DNS TXT challenge (`txtRecord`, `txtValue`)
- **Response 403:** `PLAN_FEATURE_UNAVAILABLE` — plan does not include custom domain
- **Response 400:** `RESERVED_HOSTNAME`, `HOSTNAME_TAKEN`, `VALIDATION_ERROR`

**Implemented:** Phase 7 — TXT at `_fullsales-verify.<hostname>` (ADR-017).

### `GET /v1/settings/domains/{id}/verify`

- **Auth:** Tenant Admin
- **Response 200:** `{ "status", "txtRecord", "txtValue", "verifiedAt" }`

**Implemented:** Phase 7.

### `DELETE /v1/settings/domains/{id}`

- **Auth:** Tenant Admin
- **Precondition:** Domain not already `Detached`; detach allowed from `Pending`, `Verifying`, `Verified`, `Active`, or `Failed`
- **Response 204:** Domain detached

**Implemented:** Phase 7.

### `POST /v1/settings/domains/{id}/set-primary`

- **Auth:** Tenant Admin
- **Body:** none
- **Response 200:** Updated domain — activates if `Verified`, sets `isPrimary`; previous primary detached (BR-DM-001)
- **Response 400:** `INVALID_TRANSITION` when domain is not verified/active

**Implemented:** Phase 7.

### `GET /v1/settings/payments`

- **Auth:** Tenant Admin
- **Response 200:** `{ "enabled", "methods": { "pix", "credit", "boleto" }, "autoCapture", "asaas": { "connected", "apiKeyLast4?", "connectedAt?" } }`

**Implemented:** Phase 5.

### `PUT /v1/settings/payments`

- **Auth:** Tenant Admin (Pro+ for `enabled: true`)
- **Body:** `{ "enabled", "methods": { "pix", "credit", "boleto" }, "autoCapture" }`
- **Response 200:** Updated settings
- **Response 403:** `PLAN_FEATURE_UNAVAILABLE` (Starter plan)

**Implemented:** Phase 5.

### `POST /v1/settings/payments/asaas/connect`

- **Auth:** Tenant Admin (Pro+)
- **Body:** `{ "apiKey" }` — encrypted at rest with AES-256-GCM (ADR-018)
- **Response 200:** `{ "connected": true, "accountName" }`
- **Response 400:** `INVALID_ASAAS_CREDENTIALS`

**Implemented:** Phase 5 — validates via Asaas `GET /myAccount`; audit event `tenant.asaas.connected`.

### `DELETE /v1/settings/payments/asaas/connect`

- **Auth:** Tenant Admin
- **Response 204:** Credentials removed; online payments disabled

**Implemented:** Phase 5 — audit event `tenant.asaas.disconnected`.

### `GET /v1/settings/payments/balance`

- **Auth:** Tenant Admin
- **Response 200:** `{ "balanceMinor", "currency" }` — proxied from tenant Asaas account (read-only)
- **Rate limit:** 30 req/min per tenant; **cache:** 60s

**Implemented:** Phase 5 — no withdraw through platform v1.

### `GET /v1/settings/payments/transactions`

- **Auth:** Tenant Admin
- **Query:** `offset`, `limit` (max 50)
- **Response 200:** `{ "data": [ { "id", "type", "amountMinor", "date", "description?" } ], "hasMore" }`
- **Rate limit:** 30 req/min per tenant; **cache:** 60s

**Implemented:** Phase 5.

### Public checkout subset

`GET /v1/public/settings` includes optional `paymentMethods` when tenant online payments are enabled (portal checkout).

---

## OpenAPI

Full schema: [`docs/openapi.yaml`](openapi.yaml) — all implemented `/v1/*` routes (Phases 16–26).

**Testing / drift:** Every `### \`METHOD /path\`` heading in this file must match a `.route(...)` in `backend/crates/api-http/src/routes.rs` (CI: `pnpm verify:api-route-inventory`). See [TESTING-STRATEGY.md](TESTING-STRATEGY.md).

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

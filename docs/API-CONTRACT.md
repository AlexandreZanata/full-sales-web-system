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

| Param | Default | Max |
|-------|---------|-----|
| `page` | 1 | — |
| `pageSize` | 20 | 50 |

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
- **Response 200:** Paginated user list

---

## Commerces

### `POST /v1/commerces`

- **Auth:** Admin only (BR-IA-001)
- **Body:** `{ "cnpj", "legalName", "tradeName?", "address", "contact" }`
- **Response 201:** Commerce

### `GET /v1/commerces`

- **Auth:** Admin, Driver, Seller (read)
- **Response 200:** Paginated list

### `GET /v1/commerces/{id}`

- **Auth:** Admin, Driver, Seller
- **Response 200 / 404**

---

## Products

### `POST /v1/products`

- **Auth:** Admin
- **Body:** `{ "name", "sku", "priceAmount", "priceCurrency" }`
- **Response 201:** Product

### `GET /v1/products`

- **Auth:** Admin, Driver, Seller
- **Response 200:** Paginated list

---

## Inventory

### `GET /v1/inventory/products/{productId}/balance`

- **Auth:** Driver, Seller, Admin
- **Response 200:** `{ "productId", "available", "asOf" }`

### `POST /v1/inventory/movements`

- **Auth:** Admin (adjustments); system on sale confirm
- **Body:** `{ "productId", "movementType", "quantity", "reason?" }`
- **Response 201:** StockMovement

---

## Sales

### `POST /v1/sales`

- **Auth:** Driver, Seller, Admin
- **Body (allow-list):** `{ "commerceId", "items": [{ "productId", "quantity" }], "paymentMethod" }` — `paymentMethod`: `"cash"` \| `"pix"` \| `"credit"` \| `"debit"` (ADR-006)
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

- **Auth:** Driver (own), Admin (all)
- **Response 200 / 404**

### `GET /v1/sales`

- **Auth:** Admin, Driver (own sales filter)
- **Query:** `commerceId?`, `from?`, `to?`, pagination
- **Response 200:** Paginated list

---

## Portal — Products (Phase 14)

### `GET /v1/portal/products`

- **Auth:** CommerceContact only
- **Query:** pagination, `category?`
- **Response 200:** Active products with optional `primaryImageUrl` (presigned ~15 min)

---

## Portal — Orders (Phase 14)

### `GET /v1/portal/orders`

- **Auth:** CommerceContact (JWT `commerceId` — RLS scoped)
- **Query:** `status?`, pagination
- **Response 200:** Orders for contact's commerce only

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

---

## Reports

### `POST /v1/reports`

- **Auth:** Admin
- **Body:** `{ "reportType", "periodStart", "periodEnd", "driverId?", "commerceId?" }`
- **Response 201:** Report with `id`, `signature`, `publicKeyId`

### `GET /v1/reports/{id}`

- **Auth:** Admin, Driver (if scoped)
- **Response 200:** Report metadata + payload

### `GET /v1/reports/{id}/verify`

- **Auth:** Public (rate limited by IP — ADR-007)
- **Response 200:** `{ "valid": true | false, "reportId" }`

---

## Health

### `GET /health`

- **Auth:** None
- **Response 200:** `{ "status": "ok" }`

---

## OpenAPI

Full schema: [`docs/openapi.yaml`](openapi.yaml) — Phase 3 (sales + products endpoints).

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

# Business Rules

> Format: GIVEN / WHEN / THEN / AND. Each rule has an ID and maps to domain tests.

**TypeScript domain tests:** `packages/domain/src/**/*.test.ts` (run `pnpm --filter @full-sales/domain test:coverage`).

| Rule ID | Test file(s) |
|---------|----------------|
| BR-IA-001 | `backend/crates/api-http/tests/auth.rs` |
| BR-IA-002 | `backend/crates/application/src/auth.rs`, `backend/crates/api-http/tests/auth.rs`, `backend/crates/domain-identity/tests/business_rules.rs` |
| BR-IA-003 | `backend/crates/domain-identity/tests/commerce_contact.rs`, `backend/crates/infra-postgres/tests/identity_profiles.rs` |
| BR-CO-001 | `backend/crates/domain-commerces/src/cnpj.rs`, `backend/crates/api-http/tests/auth.rs` |
| BR-CO-010 | `backend/crates/api-http/tests/commerce_registrations.rs`, `backend/crates/domain-commerces/tests/commerce_registration.rs` |
| BR-CO-011 | `backend/crates/api-http/tests/commerce_registrations.rs` |
| BR-CO-012 | `backend/crates/api-http/tests/commerce_registrations.rs` |
| BR-CO-004 | `backend/crates/domain-commerces/tests/commerce_address.rs` |
| BR-CO-005 | `backend/crates/domain-commerces/tests/commerce_address.rs` |
| BR-IN-003 | `packages/domain/src/sales/sale.test.ts` |
| RN2 | `backend/crates/domain-inventory/tests/stock_reservation.rs`, `backend/crates/infra-postgres/tests/inventory_catalog.rs` |
| RN3 | `backend/crates/domain-orders/tests/order_item.rs` |
| RN4 | `backend/crates/domain-deliveries/tests/proof_required.rs` |
| RN5 | `backend/crates/domain-deliveries/tests/partial_delivery.rs` |
| RN6 | `backend/crates/domain-orders/tests/cancel_order.rs`, `backend/crates/application/src/orders.rs` |
| RN10 | `backend/crates/domain-orders/tests/reject_order.rs` |
| BR-SA-001 | `packages/domain/src/sales/sale.test.ts` |
| BR-SA-002 | `packages/domain/src/value-objects/money.test.ts`, `packages/domain/src/sales/sale-item.test.ts`, `packages/domain/src/sales/sale.test.ts` |
| BR-SA-003 | `packages/domain/src/sales/sale.test.ts` |
| BR-SA-004 | `backend/crates/api-http/tests/sales.rs` (`contract_top_selling_products_after_confirm_then_lists_product`) |
| BR-BI-001 | `backend/crates/api-http/tests/billing_webhook.rs` |
| BR-BI-002 | `backend/crates/domain-billing/tests/business_rules.rs`, `backend/crates/api-http/tests/billing_subscription.rs` |
| BR-BI-003 | `backend/crates/api-http/tests/billing_subscription.rs` |

---

## Identity & Access

### BR-IA-001 — Admin-only direct commerce registration

```
GIVEN a User with role Driver or Seller
WHEN they attempt POST /v1/commerces (admin direct create)
THEN the operation is rejected with Forbidden
AND no Commerce record is created

GIVEN a User with role Seller
WHEN they POST /v1/commerces/registrations with valid payload
THEN a Commerce is created with registrationStatus PendingReview (see BR-CO-010)
```

### BR-IA-002 — Inactive user cannot authenticate

```
GIVEN a User with active = false
WHEN login is attempted with valid credentials
THEN authentication fails
AND no tokens are issued
```

### BR-IA-003 — CommerceContact scoped to own commerce

```
GIVEN a User with role CommerceContact linked to commerce X
WHEN they attempt to access portal data for commerce Y
THEN the operation is rejected with Forbidden
AND RLS or domain scope checks return empty/forbidden
```

---

## Commerces

### BR-CO-001 — Invalid CNPJ rejected at creation

```
GIVEN a CNPJ string with invalid check digits
WHEN Commerce.create is called
THEN construction fails with InvalidCnpj
AND nothing is persisted
```

### BR-CO-002 — Commerce deactivation

```
GIVEN an active Commerce with no Pending sales
WHEN Admin deactivates the Commerce
THEN active becomes false
AND new Sales referencing this Commerce are rejected
```

### BR-CO-010 — Seller submit creates pending commerce

```
GIVEN a User with role Seller
WHEN they POST a valid registration to /v1/commerces/registrations
THEN a Commerce is created with registrationStatus PendingReview and active true
AND the commerce appears in the seller commerce catalog until admin rejects or deactivates
AND submittedByUserId is set
AND a Delivery address is persisted
```

### BR-CO-011 — Only review privilege can approve

```
GIVEN a Commerce in PendingReview
WHEN a User without review privilege calls POST .../approve
THEN 403 Forbidden
```

### BR-CO-012 — CNPJ lookup does not bypass validation

```
GIVEN lookup returns data for a CNPJ
WHEN seller submits registration
THEN domain still validates CNPJ check digits (BR-CO-001)
AND required address/contact fields are present
```

### BR-CO-004 — Inactive commerce cannot add delivery addresses

```
GIVEN a Commerce with active = false
WHEN a Delivery address is added for order use
THEN the operation fails with InactiveCommerceCannotAddDeliveryAddress
AND Billing addresses may still be added for archival purposes
```

### BR-CO-005 — Order requires valid delivery address

```
GIVEN an Order being created for Commerce C
WHEN delivery_address_id references a Billing address, another commerce, or C is inactive
THEN order creation fails
AND only a Delivery address belonging to an active Commerce C is accepted
```

---

## Inventory

### BR-IN-001 — Insufficient stock blocks sale confirmation

```
GIVEN a Product with available balance of 5 units
WHEN a Sale is confirmed requiring 10 units of that Product
THEN confirmation fails with InsufficientStock
AND stock balance remains 5
AND Sale status remains Pending
```

### BR-IN-002 — Sale confirmation creates outbound movement

```
GIVEN a Pending Sale with valid items and sufficient stock
WHEN the Sale is confirmed
THEN a StockMovement of type SaleOutbound is recorded
AND stock balance is reduced
AND both occur in the same database transaction
```

### BR-IN-003 — Inactive product cannot be sold

```
GIVEN a Product marked inactive
WHEN a SaleItem referencing that Product is added
THEN the operation fails with InactiveProduct
```

### RN2 — Reserve on approve, release on cancel, consume on delivery

```
GIVEN an order in PendingApproval with items within tenant available stock
WHEN Admin approves
THEN stock_reservations are created Active
AND stock_balances quantity is NOT reduced yet

GIVEN an approved order cancelled before InTransit
WHEN release_reservations runs
THEN Active reservations become Released

GIVEN an approved order with confirmed delivery
WHEN consume_reservations runs
THEN Active reservations become Consumed
AND StockMovement SaleOutbound reduces driver balance (Phase 12–13)
```

### RN3 — Unit price frozen at order item creation

```
GIVEN a product price changes after an order item is added in Draft
WHEN the order is fulfilled
THEN line uses unit_price frozen at item creation
AND total is computed from frozen line totals only
```

### RN6 — Cancel before InTransit releases reservation

```
GIVEN an order in Approved or Picking (not InTransit)
WHEN the order is cancelled
THEN status becomes Cancelled
AND all Active stock_reservations for the order become Released

GIVEN an order in InTransit
WHEN cancel is attempted
THEN the operation fails with InvalidOrderTransition
```

### RN10 — Reject requires rejection reason

```
GIVEN an order in PendingApproval
WHEN Admin rejects without a non-empty rejection_reason
THEN rejection fails with RejectionReasonRequired
AND status remains PendingApproval
```

### RN4 — Proof photo required for delivery confirmation

```
GIVEN a delivery InTransit
WHEN the assigned driver confirms without proof_file_id
THEN confirmation fails with ProofRequired
AND delivery status remains InTransit

GIVEN a delivery InTransit with valid proof_file_id
WHEN the assigned driver confirms
THEN delivery status becomes Delivered
```

### RN5 — Partial delivery sets PartiallyDelivered

```
GIVEN an order InTransit with items where quantity_delivered < quantity_requested
WHEN delivery is confirmed
THEN order status becomes PartiallyDelivered
AND quantity_delivered is persisted on each order item

GIVEN all items fully delivered
WHEN delivery is confirmed
THEN order status becomes Delivered
```

---

## Sales

### BR-SA-001 — Cannot confirm empty sale

```
GIVEN a Sale with zero items
WHEN confirm is called
THEN the operation fails with EmptySale
AND status remains Pending
```

### BR-SA-002 — Total computed from items only

```
GIVEN a Sale with items totaling Money(15000, BRL)
WHEN client sends total Money(10000, BRL) in request body
THEN the persisted total is Money(15000, BRL)
AND client-supplied total is ignored
```

### BR-SA-003 — Cancel pending sale only

```
GIVEN a Sale in status Confirmed
WHEN cancel is called
THEN the operation fails with InvalidSaleTransition
```

### BR-SA-004 — Confirmed sales increment product sales totals

```
GIVEN a Pending sale with line items
WHEN the sale is Confirmed (POST /v1/sales/{id}/confirm or delivery confirm path)
THEN sales.product_sales_totals.units_sold increases by each line quantity per product
AND GET /v1/products/top-selling returns products ordered by units_sold DESC
WHEN the sale remains Pending or is Cancelled before confirm
THEN product sales totals are unchanged
```

---

## Reports

### BR-RE-001 — Canonical payload before signature

```
GIVEN assembled report data for a period
WHEN Report.generate is called
THEN canonical JSON is produced with deterministic key ordering
AND SHA-256 hash of canonical bytes is signed with Ed25519
AND signature is stored with public_key_id
```

### BR-RE-002 — Tampered report fails verification

```
GIVEN a persisted Report
WHEN any byte of canonical_payload is altered
THEN GET /v1/reports/{id}/verify returns valid: false
```

---

## API authorization matrix

> Source of truth: `backend/crates/api-http/src/routes.rs` + per-handler `require_*` checks.  
> **Yes** = allowed · **No** = 403 Forbidden · **Own** = scoped to authenticated user · **Scoped** = entity ownership / JWT commerce · **Public** = no Bearer token.

| Route | Admin | Driver | Seller | CommerceContact | Public |
|-------|-------|--------|--------|-----------------|--------|
| `GET /health` | Public | Public | Public | Public | Yes |
| `GET /v1/` | Public | Public | Public | Public | Yes |
| `POST /v1/auth/login` | Public | Public | Public | Public | Yes |
| `POST /v1/auth/refresh` | Public | Public | Public | Public | Yes |
| `POST /v1/auth/logout` | Yes | Yes | Yes | Yes | No |
| `POST /v1/users` | Yes | No | No | No | No |
| `GET /v1/users` | Yes | No | No | No | No |
| `GET /v1/users/{id}` | Yes | No | No | No | No |
| `PATCH /v1/users/{id}/deactivate` | Yes | No | No | No | No |
| `PUT /v1/users/{id}/driver-profile` | Yes | No | No | No | No |
| `PUT /v1/users/{id}/seller-profile` | Yes | No | No | No | No |
| `POST /v1/commerces` | Yes | No | No | No | No |
| `GET /v1/commerces` | Yes | Yes | Yes | No | No |
| `GET /v1/commerces/{id}` | Yes | Yes | Yes | No | No |
| `PATCH /v1/commerces/{id}/deactivate` | Yes | No | No | No | No |
| `GET /v1/commerces/{id}/addresses` | Yes | Yes | Yes | No | No |
| `POST /v1/commerces/{id}/addresses` | Yes | No | No | No | No |
| `PATCH /v1/commerces/{id}/addresses/{addressId}` | Yes | No | No | No | No |
| `PUT /v1/commerces/{id}/logo` | Yes | No | No | No | No |
| `POST /v1/products` | Yes | No | No | No | No |
| `GET /v1/products` | Yes | Yes | Yes | No | No |
| `GET /v1/products/{id}` | Yes | Yes | Yes | No | No |
| `PATCH /v1/products/{id}` | Yes | No | No | No | No |
| `GET /v1/products/{id}/images` | Yes | No | No | No | No |
| `POST /v1/products/{id}/images` | Yes | No | No | No | No |
| `DELETE /v1/products/{id}/images/{imageId}` | Yes | No | No | No | No |
| `GET /v1/inventory/products/{productId}/balance` | Yes | Yes | Yes | No | No |
| `POST /v1/inventory/movements` | Yes | No | No | No | No |
| `GET /v1/inventory/products/{productId}/movements` | Yes | No | No | No | No |
| `POST /v1/sales` | Yes | Yes | Yes | No | No |
| `GET /v1/sales` | Yes | Own | Own | No | No |
| `GET /v1/sales/{id}` | Yes | Own | Own | No | No |
| `POST /v1/sales/{id}/confirm` | Yes | Yes | Yes | No | No |
| `POST /v1/sales/{id}/cancel` | Yes | Yes | Yes | No | No |
| `POST /v1/sales/{id}/declare-payment` | No | Own | No | No | No |
| `GET /v1/portal/products` | No | No | No | Yes | No |
| `GET /v1/portal/orders` | No | No | No | Scoped | No |
| `GET /v1/portal/orders/{id}` | No | No | No | Scoped | No |
| `POST /v1/portal/orders` | No | No | No | Scoped | No |
| `PUT /v1/portal/orders/{id}` | No | No | No | Scoped | No |
| `DELETE /v1/portal/orders/{id}` | No | No | No | Scoped | No |
| `POST /v1/portal/orders/{id}/submit` | No | No | No | Scoped | No |
| `GET /v1/orders` | Yes | No | No | No | No |
| `GET /v1/orders/{id}` | Yes | No | No | No | No |
| `POST /v1/orders/{id}/approve` | Yes | No | No | No | No |
| `POST /v1/orders/{id}/reject` | Yes | No | No | No | No |
| `POST /v1/orders/{id}/cancel` | Yes | No | No | No | No |
| `POST /v1/orders/{id}/start-picking` | Yes | No | No | No | No |
| `POST /v1/orders/{id}/delivery` | Yes | No | No | No | No |
| `GET /v1/deliveries` | Yes | Own | No | No | No |
| `GET /v1/deliveries/{id}` | Yes | Own | No | No | No |
| `POST /v1/deliveries/{id}/start-transit` | No | Own | No | No | No |
| `POST /v1/deliveries/{id}/confirm` | No | Own | No | No | No |
| `POST /v1/media/upload` | Yes | Scoped | Scoped | Scoped | No |
| `GET /v1/media/{id}/url` | Yes | Scoped | Scoped | Scoped | No |
| `POST /v1/reports` | Yes | No | No | No | No |
| `GET /v1/reports` | Yes | No | No | No | No |
| `GET /v1/reports/{id}` | Yes | Own | No | No | No |
| `GET /v1/reports/{id}/verify` | Public | Public | Public | Public | Yes |
| `GET /v1/audit/events` | Yes | No | No | No | No |

**Media scoped access:** Admin — all entities. Driver — own `User`, any `Product`, assigned `Delivery`. Seller — any `Product`. CommerceContact — own `Commerce` only.

**Portal scoped access:** CommerceContact JWT `commerceId` — RLS limits orders to own commerce (BR-IA-003).

---

## Domain expansion (planned — Phases 07–15)

> Full GIVEN/WHEN/THEN in `.local/phases/0d-domain-expansion/documentation/BUSINESS-RULES-EXPANSION.md`.  
> Implement and promote to this file per phase. **RN1 credit limit revoked.**

| Rule ID | Summary | Phase | Test file (TBD) |
|---------|---------|-------|-----------------|
| RN2 | Reserve on approve; deduct on delivery | 10, 12–13 | `domain-inventory/tests/stock_reservation.rs`, `infra-postgres/tests/inventory_catalog.rs` |
| RN3 | Unit price frozen at order item creation | 11 | `domain-orders/tests/order_item.rs` |
| RN4 | Proof photo required for Delivered | 12 | `domain-deliveries/tests/proof_required.rs` |
| RN5 | Partial qty → PartiallyDelivered | 12 | `domain-deliveries/tests/partial_delivery.rs` |
| RN6 | Cancel before InTransit releases reservation | 11 | `domain-orders/tests/cancel_order.rs`, `application/src/orders.rs` |
| RN7 | Mime + size validation before storage | 07 | `domain-media/tests/upload_validation.rs` |
| RN8 | Role-scoped RLS visibility | 08, 11–12, 14 | `infra-postgres/tests/orders.rs`, `infra-postgres/tests/deliveries.rs` |
| RN9 | Reports exclude non-final orders | 15 | `domain-reports/tests/settlement_payload.rs` |
| RN10 | Reject requires rejection_reason | 11 | `domain-orders/tests/reject_order.rs` |
| RN-PAG1 | Declaration optional — never blocks sales | 13 | `domain-sales/tests/declared_payment.rs` |
| RN-PAG2 | Only responsible seller/driver declares | 13 | `domain-sales/tests/declared_payment.rs` |
| RN-PAG3 | Declaration changes append audit.events | 13 | `infra-postgres/tests/declared_payment_audit.rs` |
| RN-PAG4 | Report includes non-fiscal disclaimer | 15 | `domain-reports/tests/settlement_payload.rs` |
| BR-IA-003 | CommerceContact scoped to own commerce | 08, 14 | `domain-identity/tests/commerce_contact.rs` |
| ~~RN1~~ | ~~Credit limit blocks order~~ | — | **Revoked** (Phase 0d) |

---

## Platform SaaS (proposed — Phases 1–13)

> Locked in Phase 0. Implement and wire tests per phase. Spec: `.local/phases/0-platform-vision-decisions/_reference/PLATFORM-SAAS-SPEC.md`.

### BR-PL-001 — Suspended tenant blocks mutations

```
GIVEN a Tenant with status Suspended
WHEN any mutating /v1/* request is made except billing self-service or Asaas webhooks
THEN the operation is rejected with TENANT_SUSPENDED
AND no domain data is modified
```

### BR-PL-002 — PlatformAdmin-only tenant provision

```
GIVEN an unauthenticated or tenant-scoped JWT
WHEN POST /v1/platform/tenants is called
THEN the operation is rejected with Forbidden
AND no Tenant is created
```

### BR-BI-001 — Asaas webhook idempotency

```
GIVEN a PaymentEvent with asaas_event_id X already persisted
WHEN POST /v1/billing/webhooks/asaas delivers the same event id again
THEN the handler returns 200 without duplicating side effects
AND subscription/tenant state is unchanged from first processing
```

### BR-BI-002 — Trial expiry

```
GIVEN a Tenant in Trial with trial_ends_at in the past
WHEN the trial expiry job runs
THEN the tenant transitions to Suspended OR Active based on payment method on file
AND no further trial extension occurs without PlatformAdmin action
```

### BR-BI-003 — Past due grace period

```
GIVEN a Tenant in PastDue for 7 consecutive days without PAYMENT_CONFIRMED
WHEN the dunning job runs on day 8
THEN the tenant transitions to Suspended
AND mutating APIs are blocked per BR-PL-001
```

### BR-FR-001 — Payment velocity limit

```
GIVEN a tenant exceeds configured card/PIX attempts per hour
WHEN a new payment is initiated
THEN the operation is rejected with FRAUD_BLOCKED
AND a FraudEvent is recorded for PlatformAdmin review
```

### BR-FR-002 — Blocklist rejection

```
GIVEN a CNPJ or email on the platform blocklist
WHEN tenant provision or payment is attempted with that identifier
THEN the operation is rejected with FRAUD_BLOCKED
AND an audit event is emitted
```

### BR-DM-001 — One active primary domain

```
GIVEN a Tenant already has an Active primary TenantDomain
WHEN another domain is activated as primary
THEN the previous primary transitions to Detached or non-primary
AND at most one Active primary domain exists per tenant
```

### BR-AU-001 — PlatformAdmin audit

```
GIVEN a PlatformAdmin performs any mutating /v1/platform/* action or starts impersonation
WHEN the use case completes
THEN an immutable audit.events row is appended with actor, action, target tenant, correlation id
AND the event cannot be updated or deleted
```

# Business Rules

> Format: GIVEN / WHEN / THEN / AND. Each rule has an ID and maps to domain tests.

**TypeScript domain tests:** `packages/domain/src/**/*.test.ts` (run `pnpm --filter @full-sales/domain test:coverage`).

| Rule ID | Test file(s) |
|---------|----------------|
| BR-IA-001 | `backend/crates/api-http/tests/auth.rs` |
| BR-IA-002 | `backend/crates/application/src/auth.rs`, `backend/crates/api-http/tests/auth.rs`, `backend/crates/domain-identity/tests/business_rules.rs` |
| BR-IA-003 | `backend/crates/domain-identity/tests/commerce_contact.rs`, `backend/crates/infra-postgres/tests/identity_profiles.rs` |
| BR-CO-001 | `backend/crates/domain-commerces/src/cnpj.rs`, `backend/crates/api-http/tests/auth.rs` |
| BR-CO-004 | `backend/crates/domain-commerces/tests/commerce_address.rs` |
| BR-CO-005 | `backend/crates/domain-commerces/tests/commerce_address.rs` |
| BR-IN-003 | `packages/domain/src/sales/sale.test.ts` |
| RN2 | `backend/crates/domain-inventory/tests/stock_reservation.rs`, `backend/crates/infra-postgres/tests/inventory_catalog.rs` |
| BR-SA-001 | `packages/domain/src/sales/sale.test.ts` |
| BR-SA-002 | `packages/domain/src/value-objects/money.test.ts`, `packages/domain/src/sales/sale-item.test.ts`, `packages/domain/src/sales/sale.test.ts` |
| BR-SA-003 | `packages/domain/src/sales/sale.test.ts` |

---

## Identity & Access

### BR-IA-001 — Admin-only commerce registration

```
GIVEN a User with role Driver or Seller
WHEN they attempt to register a Commerce
THEN the operation is rejected with Forbidden
AND no Commerce record is created
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

## Authorization matrix (summary)

| Action | Admin | Driver | Seller | CommerceContact |
|--------|-------|--------|--------|-----------------|
| Register Commerce | Yes | No | No | No |
| Register User | Yes | No | No | No |
| Create Sale | Yes | Yes | Yes | No |
| Confirm Sale | Yes | Yes | Yes | No |
| Generate Report | Yes | No* | No | No |
| Verify Report signature | Yes | Yes | Yes | No |
| Portal login | No | No | No | Yes |

\* Driver may receive pre-generated reports — refine in use cases.

Full matrix: extend in use case authorization tables.

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
| RN6 | Cancel before InTransit releases reservation | 11 | `domain-orders/tests/cancel_order.rs` |
| RN7 | Mime + size validation before storage | 07 | `domain-media/tests/upload_validation.rs` |
| RN8 | Role-scoped RLS visibility | 08, 11–12, 14 | `infra-postgres/tests/rls_expansion.rs` |
| RN9 | Reports exclude non-final orders | 15 | `domain-reports/tests/settlement_payload.rs` |
| RN10 | Reject requires rejection_reason | 11 | `domain-orders/tests/reject_order.rs` |
| RN-PAG1 | Declaration optional — never blocks sales | 13 | `domain-sales/tests/declared_payment.rs` |
| RN-PAG2 | Only responsible seller/driver declares | 13 | `domain-sales/tests/declared_payment.rs` |
| RN-PAG3 | Declaration changes append audit.events | 13 | `infra-postgres/tests/declared_payment_audit.rs` |
| RN-PAG4 | Report includes non-fiscal disclaimer | 15 | `domain-reports/tests/settlement_payload.rs` |
| BR-IA-003 | CommerceContact scoped to own commerce | 08, 14 | `domain-identity/tests/commerce_contact.rs` |
| ~~RN1~~ | ~~Credit limit blocks order~~ | — | **Revoked** (Phase 0d) |

# Business Rules

> Format: GIVEN / WHEN / THEN / AND. Each rule has an ID and maps to domain tests.

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

| Action | Admin | Driver | Seller |
|--------|-------|--------|--------|
| Register Commerce | Yes | No | No |
| Register User | Yes | No | No |
| Create Sale | Yes | Yes | Yes |
| Confirm Sale | Yes | Yes | Yes |
| Generate Report | Yes | No* | No |
| Verify Report signature | Yes | Yes | Yes |

\* Driver may receive pre-generated reports — refine in use cases.

Full matrix: extend in use case authorization tables.

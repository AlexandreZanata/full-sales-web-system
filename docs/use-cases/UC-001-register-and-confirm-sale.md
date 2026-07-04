# Use Case: UC-001 — Register and Confirm Sale

---

## Metadata

| Field | Value |
|-------|-------|
| ID | UC-001 |
| Actor | Driver or Seller |
| Status | Approved (from product spec) |

## Preconditions

- User authenticated with role Driver or Seller
- Commerce exists and is active
- Products exist with sufficient stock for requested quantities

## Main flow (happy path)

1. Driver selects active Commerce.
2. Driver adds SaleItems (product + quantity) — system loads unit price from Product.
3. Driver selects PaymentMethod.
4. System creates Sale in status `Pending`; total computed from items (BR-SA-002).
5. Driver confirms sale.
6. System validates stock (BR-IN-001), transitions Sale to `Confirmed`, records `SaleOutbound` movement (BR-IN-002) in one transaction.
7. System returns confirmed Sale with total and timestamp.

## Alternate flows

### AF-1: Insufficient stock

- **When:** Confirm requested with quantity exceeding balance
- **Then:** 409 `INSUFFICIENT_STOCK`; Sale remains `Pending`; stock unchanged

### AF-2: Cancel before confirm

- **When:** Driver cancels Pending sale
- **Then:** Sale → `Cancelled`; no stock movement

### AF-3: Inactive commerce

- **When:** Commerce is deactivated
- **Then:** Create sale rejected with `COMMERCE_INACTIVE`

## Business rules applied

| Rule ID | Description |
|---------|-------------|
| BR-SA-001 | No empty sale confirmation |
| BR-SA-002 | Total from items only |
| BR-IN-001 | Insufficient stock blocks confirm |
| BR-IN-002 | Confirm creates outbound movement |
| BR-IN-003 | Inactive product rejected |

## Domain events raised

| Event | When |
|-------|------|
| `SaleCreated` | After step 4 |
| `SaleConfirmed` | After step 6 |
| `StockMovementRecorded` | After step 6 (same transaction) |

## Authorization

| Role | Create | Confirm | Cancel | View own |
|------|--------|---------|--------|----------|
| Admin | Yes | Yes | Yes | Yes |
| Driver | Yes | Yes | Yes | Yes |
| Seller | Yes | Yes | Yes | Yes |

## API mapping

| Step | Endpoint |
|------|----------|
| 4 | `POST /v1/sales` |
| 6 | `POST /v1/sales/{id}/confirm` |
| AF-2 | `POST /v1/sales/{id}/cancel` |

## Out of scope

- Payment gateway capture (PaymentMethod recorded only)
- ICP-Brasil fiscal document

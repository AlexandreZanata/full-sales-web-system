# State Machines

> Invalid transitions throw domain errors — never silent. Terminal states allow no further transitions.

---

## Sale (`SaleStatus`)

| State | Description |
|-------|-------------|
| `Pending` | Created, items may be added, not yet confirmed |
| `Confirmed` | Stock deducted, sale finalized |
| `Cancelled` | Voided before or without confirmation |

### Valid transitions

| From | To | Trigger | Allowed roles |
|------|-----|---------|---------------|
| — | `Pending` | `Sale.create` | Driver, Seller, Admin |
| `Pending` | `Confirmed` | `Sale.confirm` | Driver, Seller, Admin |
| `Pending` | `Cancelled` | `Sale.cancel` | Driver, Seller, Admin |
| `Confirmed` | — | *(terminal)* | — |
| `Cancelled` | — | *(terminal)* | — |

### Invalid transitions (must error)

| From | To | Error |
|------|-----|-------|
| `Confirmed` | `Pending` | `InvalidSaleTransition` |
| `Confirmed` | `Cancelled` | `InvalidSaleTransition` |
| `Cancelled` | `Confirmed` | `InvalidSaleTransition` |
| `Cancelled` | `Pending` | `InvalidSaleTransition` |

**Side effects on `Pending → Confirmed`:** BR-IN-002 — `StockMovement` (`SaleOutbound`) in same transaction.

---

## User (`active` flag)

Not a multi-state FSM — boolean lifecycle:

| State | Transition | Trigger |
|-------|------------|---------|
| `active = true` | `active = false` | Admin deactivates |
| `active = false` | `active = true` | Admin reactivates |

Deactivated users cannot authenticate (BR-IA-002).

---

## Commerce (`active` flag)

Same pattern as User. Deactivated commerce blocks new sales (BR-CO-002).

---

## Report

Reports are **immutable** after generation — no state transitions. Verification is read-only.

| State | Meaning |
|-------|---------|
| Generated | Persisted with signature — terminal |

---

## StockMovement

Movements are **append-only** — no edits or deletes. Corrections via compensating `Adjustment` movement.

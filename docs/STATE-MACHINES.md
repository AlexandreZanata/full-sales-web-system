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

---

## Order (`OrderStatus`)

| State | Description |
|-------|-------------|
| `Draft` | Created; items may be added or changed |
| `PendingApproval` | Submitted; awaiting admin decision (DE-002) |
| `Approved` | Admin approved; stock reserved (RN2) |
| `Rejected` | Admin rejected with reason (RN10) |
| `Picking` | Warehouse picking in progress |
| `InTransit` | Driver assigned and en route |
| `Delivered` | Fully delivered |
| `PartiallyDelivered` | Partial qty delivered (RN5) |
| `Cancelled` | Voided before or during pre-transit fulfillment |

### Valid transitions

| From | To | Trigger | Allowed roles |
|------|-----|---------|---------------|
| — | `Draft` | `Order.create` | CommerceContact, Seller, Admin |
| `Draft` | `PendingApproval` | `Order.submit` | Creator, Admin |
| `PendingApproval` | `Approved` | `Order.approve` | Admin |
| `PendingApproval` | `Rejected` | `Order.reject` | Admin |
| `Approved` | `Picking` | `Order.start_picking` | Admin |
| `Picking` | `InTransit` | `Order.mark_in_transit` | Admin, Driver |
| `InTransit` | `Delivered` | Delivery confirm | Driver |
| `InTransit` | `PartiallyDelivered` | Partial delivery confirm | Driver |
| `Draft` | `Cancelled` | `Order.cancel` | Admin, creator |
| `PendingApproval` | `Cancelled` | `Order.cancel` | Admin, creator |
| `Approved` | `Cancelled` | `Order.cancel` | Admin, creator |
| `Picking` | `Cancelled` | `Order.cancel` | Admin, creator |
| `Rejected` | — | *(terminal)* | — |
| `Delivered` | — | *(terminal)* | — |
| `PartiallyDelivered` | — | *(terminal)* | — |
| `Cancelled` | — | *(terminal)* | — |

### Invalid transitions (must error)

| From | To | Error |
|------|-----|-------|
| `InTransit` | `Cancelled` | `InvalidOrderTransition` (RN6) |
| `Delivered` | `Approved` | `InvalidOrderTransition` |
| `Rejected` | `Approved` | `InvalidOrderTransition` |
| Any terminal | Any | `InvalidOrderTransition` |

**Side effects on `PendingApproval → Approved`:** RN2 — `StockReservation` (`Active`) in same transaction as status update.

**Side effects on cancel from `Approved`/`Picking`:** RN6 — Active reservations become `Released`.

---

## Delivery (`DeliveryStatus`)

| State | Description |
|-------|-------------|
| `Waiting` | Created with assigned driver; not yet en route |
| `InTransit` | Driver started transit |
| `Delivered` | Confirmed with proof photo (RN4) |
| `Failed` | Delivery could not be completed |

### Valid transitions

| From | To | Trigger | Allowed roles |
|------|-----|---------|---------------|
| — | `Waiting` | `Delivery.create` | Admin |
| `Waiting` | `InTransit` | `Delivery.start_transit` | Assigned Driver |
| `InTransit` | `Delivered` | `Delivery.confirm` | Assigned Driver |
| `Waiting` / `InTransit` | `Failed` | mark failed | Admin |

### Invalid transitions (must error)

| From | To | Error |
|------|-----|-------|
| `InTransit` | `Delivered` (no proof) | `ProofRequired` (RN4) |
| Confirm by non-assigned driver | — | `DriverNotAssigned` |

**Side effects on `InTransit → Delivered`:** Order → `Delivered` or `PartiallyDelivered` (RN5); sale created; stock deducted; reservations consumed (Phase 12 preview TX).

---

## Tenant (`TenantStatus`)

> ADR-015. Drives API access and billing dunning.

| State | Description |
|-------|-------------|
| `Provisioning` | Created; initial setup in progress |
| `Trial` | Starter trial active (`trial_ends_at` in future) |
| `Active` | Paid or trial converted; full access |
| `PastDue` | Subscription payment overdue (grace period) |
| `Suspended` | Grace expired or manual suspend — hard block mutations |
| `Offboarding` | Scheduled deletion; read-only export window |
| `Deleted` | Terminal — PII anonymized; RLS returns zero rows |

### Valid transitions

| From | To | Trigger | Actor |
|------|-----|---------|-------|
| — | `Provisioning` | `Tenant.create` | PlatformAdmin |
| `Provisioning` | `Trial` | attach Starter + set trial | PlatformAdmin / system |
| `Provisioning` | `Active` | attach paid plan | PlatformAdmin |
| `Trial` | `Active` | trial converts or paid | Asaas webhook / PlatformAdmin |
| `Trial` | `Suspended` | trial expired unpaid | system job |
| `Active` | `PastDue` | `PAYMENT_OVERDUE` webhook | system |
| `PastDue` | `Active` | `PAYMENT_CONFIRMED` webhook | system |
| `PastDue` | `Suspended` | grace day 8 or manual | system / PlatformAdmin |
| `Active` | `Suspended` | manual fraud / abuse | PlatformAdmin |
| `Suspended` | `Active` | payment cleared or manual reactivate | PlatformAdmin / webhook |
| `Active` / `Suspended` | `Offboarding` | offboard requested | PlatformAdmin |
| `Offboarding` | `Deleted` | retention elapsed (90 days) | system job |
| `Deleted` | — | *(terminal)* | — |

### Invalid transitions (must error)

| From | To | Error |
|------|-----|-------|
| `Deleted` | any | `InvalidTenantTransition` |
| `Offboarding` | `Active` | `InvalidTenantTransition` (must reactivate before offboard) |

**Side effects on `→ Suspended`:** BR-PL-001 — block mutating APIs except billing self-service.

---

## Subscription (`SubscriptionStatus`)

> Mirrors Asaas subscription + local dunning. ADR-014.

| State | Description |
|-------|-------------|
| `Pending` | Created locally; Asaas subscription not yet confirmed |
| `Active` | Current period paid or in good standing |
| `PastDue` | Overdue charge; tenant in grace |
| `Cancelled` | End of period or `SUBSCRIPTION_DELETED` |
| `Expired` | Terminal — no renewal |

### Valid transitions

| From | To | Trigger |
|------|-----|---------|
| — | `Pending` | `Subscription.create` |
| `Pending` | `Active` | first `PAYMENT_CONFIRMED` |
| `Active` | `PastDue` | `PAYMENT_OVERDUE` |
| `PastDue` | `Active` | `PAYMENT_CONFIRMED` |
| `Active` / `PastDue` | `Cancelled` | cancel at period end / webhook |
| `Cancelled` | `Expired` | period end |
| `Expired` | — | *(terminal)* |

---

## Domain (`DomainStatus`)

> ADR-017. Custom hostname per tenant.

| State | Description |
|-------|-------------|
| `Pending` | Added; DNS challenge issued |
| `Verifying` | Background job polling DNS |
| `Verified` | TXT matched; awaiting TLS |
| `Active` | TLS provisioned; traffic routed |
| `Failed` | Verification or TLS failed |
| `Detached` | Removed by tenant or PlatformAdmin |

### Valid transitions

| From | To | Trigger |
|------|-----|---------|
| — | `Pending` | `TenantDomain.add` |
| `Pending` | `Verifying` | challenge issued |
| `Verifying` | `Verified` | DNS TXT match |
| `Verifying` | `Failed` | 72 h timeout |
| `Verified` | `Active` | Caddy TLS ready |
| `Active` | `Detached` | detach / plan downgrade |
| `Failed` | `Verifying` | retry verify |
| `Detached` | — | *(terminal)* |

**Invariant:** BR-DM-001 — at most one `Active` primary domain per tenant.

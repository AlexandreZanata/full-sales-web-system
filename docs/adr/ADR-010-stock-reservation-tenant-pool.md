# ADR-010: Stock reservations use tenant pool until driver assigned

**Status:** Accepted  
**Date:** 2026-07-04  
**Deciders:** Phase 0d domain expansion sign-off

## Context

ADR-005 scopes **stock balances** per driver for field sales. The B2B order portal adds an approval → picking → assign driver → deliver flow where stock must be **reserved** at admin approval — often **before** a driver is known (DE-001).

Options:

- **A)** Driver-scoped reservations — reserve from a specific driver's balance at approval.
- **B)** Tenant warehouse pool — reserve from tenant-level available stock; bind to driver at picking/assign; deduct from assigned driver's balance at delivery.

## Decision

**Option B — tenant warehouse pool until driver assigned.**

`inventory.stock_reservations` track quantity against **tenant-level available stock** (sum of driver balances minus existing active reservations, or a dedicated tenant pool column if introduced in Phase 10). At **Picking** or driver assignment, the reservation binds to `driver_id`. At **delivery confirm**, the reservation is consumed and `StockMovement` (`SaleOutbound`) deducts from the **assigned driver's** balance (ADR-005 unchanged for the deduction target).

Field sales (`sales.order_id IS NULL`) continue to deduct directly on `Sale.confirm` with no reservation.

## Consequences

### Positive

- Admin can approve portal orders without pre-selecting a driver
- Matches central-warehouse B2B picking workflow
- ADR-005 field-sale path unchanged

### Negative

- Reservation availability query is more complex than single-driver balance
- Must prevent double-reserve across tenant pool and driver assignment race (same TX as approve)

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| A — Driver-scoped at approval | Requires driver at approval time; blocks B2B central stock |
| Hybrid per-order driver pick at draft | Extra UX step; still forces driver before stock check |

## Related

- [ADR-005](ADR-005-inventory-driver-scope.md) — driver balance scope at deduction
- DE-001 in `.local/phases/0d-domain-expansion/documentation/OPEN-DECISIONS-EXPANSION.md`

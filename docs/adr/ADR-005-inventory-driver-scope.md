# ADR-005: Inventory scoped per driver

**Status:** Accepted  
**Date:** 2026-07-04  
**Deciders:** Product spec (driver/seller README), Phase 0 sign-off

## Context

Stock can be tracked per driver/vehicle, per commerce, or hybrid. OD-005 affects aggregate boundaries and mobile UX.

## Decision

**Driver-scoped inventory:** each `Driver` has their own stock balance per `Product`. `StockMovement.responsible_id` identifies the driver. Sale confirmation deducts from the **confirming driver's** balance. Commerce is the sale destination, not the stock location.

Cache key pattern: `stock:{driver_id}:{product_id}`.

## Consequences

### Positive

- Matches field sales model (driver carries stock to commerces)
- Clear ownership for adjustments and audits
- Aligns with product README §4.3 (“por motorista/veículo”)

### Negative

- Commerce-level stock view requires aggregation query (read model)
- Vehicle-as-entity deferred — driver user is the scope key for MVP

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| Commerce stock | Does not match driver field-sales flow |
| Hybrid (both scopes) | Unnecessary complexity for MVP |

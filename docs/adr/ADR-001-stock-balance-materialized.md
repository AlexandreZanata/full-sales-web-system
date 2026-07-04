# ADR-001: Stock balance — materialized projection + Redis cache

**Status:** Accepted  
**Date:** 2026-07-04  
**Deciders:** Product spec (driver/seller README), Phase 0 sign-off

## Context

Stock reads happen on every sale confirmation and field check. Summing all `StockMovement` rows on every read is correct but slow at scale. OD-001 asked: computed on-the-fly vs materialized.

## Decision

Use a **materialized balance projection** in Postgres (updated in the same transaction as each `StockMovement` insert) plus a **Redis invalidate-on-write cache** (`stock:{driver_id}:{product_id}`, 60s TTL) for hot reads. Postgres projection remains source of truth; Redis is optional acceleration per [CACHING-STRATEGY.md](../CACHING-STRATEGY.md).

## Consequences

### Positive

- Fast balance reads for drivers in the field
- Consistent with sale + movement single-transaction invariant (BR-IN-002)
- Redis failure degrades gracefully to Postgres projection

### Negative

- Trigger/projection logic must stay in sync with movement types
- Extra invalidation ops on every write

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| Sum movements on every read | Simple but does not meet field UX at scale |
| Redis-only balance | Violates “Redis never source of truth” rule |
| Cached aggregate without Postgres projection | Harder to recover if cache is cold or lost |

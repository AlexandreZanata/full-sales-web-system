# Caching Strategy (Redis)

**Rule:** Redis is **never** source of truth. If Redis is down, the system remains functional (slower), querying Postgres directly.

---

## Cache table

| Use case | Key pattern | TTL | Invalidation |
|----------|-------------|-----|--------------|
| Session / refresh token | `session:{user_id}` | 7 days | Explicit logout or expiry |
| Stock balance (read) | `stock:{driver_id}:{product_id}` | 60s | Invalidate on `StockMovement` write |
| Login rate limit | `ratelimit:login:{ip}` | 1 min window | Natural expiry |
| Commerce profile (read) | `commerce:{commerce_id}` | 5 min | Invalidate on commerce update |

---

## Stock cache strategy

Per [ADR-001](adr/ADR-001-stock-balance-materialized.md): **materialized Postgres projection** + **invalidate-on-write** Redis cache. Default for Phase 3 MVP.

---

## References

- `agent-rules/05-performance-and-scalability/caching-strategy.md`
- Redis docs: https://redis.io/docs/latest/

# ADR-009: Ephemeral state — Redis vs PostgreSQL

| Status | Accepted |
|--------|----------|
| Date | 2026-07-04 |
| Context | Phase 1c database hardening — schema boundary decisions |

---

## Decision

Keep the following **out of PostgreSQL**; use **Redis** (existing infra):

| Concern | Storage | Rationale |
|---------|---------|-----------|
| Idempotency-Key (`POST /v1/sales`) | Redis | TTL-friendly; no long-term audit need; already implemented |
| Refresh sessions | Redis | Short-lived tokens; [ENTITY-SPEC-refresh-session](../../.local/phases/01b-database-modularization/modules/01-identity/ENTITY-SPEC-refresh-session.md) |
| Domain event outbox | **Deferred** | No cross-service consumers yet; add `shared.outbox` when async delivery is required |

---

## Consequences

- No `shared.idempotency_keys` table in Phase 1c.
- Audit trail for business actions uses `audit.events` (PostgreSQL), not idempotency keys.
- Revisit outbox when report generation or external webhooks need guaranteed delivery.

---

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| PostgreSQL idempotency table | Duplicates Redis; adds migration complexity without new requirement |
| Immediate outbox table | YAGNI — no consumer |

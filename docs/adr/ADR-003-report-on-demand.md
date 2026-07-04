# ADR-003: Report generation on-demand via API

**Status:** Accepted  
**Date:** 2026-07-04  
**Deciders:** Product spec (driver/seller README), Phase 0 sign-off

## Context

Reports can be generated when an Admin requests them or by a scheduled job (e.g. daily close). OD-003 asked which model to implement first.

## Decision

**On-demand generation** via `POST /v1/reports` (UC-002). Admin triggers report for a chosen period and scope. **Scheduled daily close** is deferred to roadmap backlog ([ROADMAP.md](../ROADMAP.md) — Future).

Empty periods produce a signed report with **zero totals** (not 404) so verification and audit trail remain consistent.

## Consequences

### Positive

- Simpler Phase 5 infrastructure — no cron/worker required initially
- Admin controls period and filters explicitly
- Empty-period signed reports preserve integrity proof

### Negative

- No automatic end-of-day reports until scheduled jobs are added
- Large periods may need async generation later (not in MVP)

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| Scheduled cron only | Delays MVP; admin cannot ad-hoc generate |
| 404 on empty period | Breaks verify flow and audit expectations |

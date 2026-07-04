# ADR-004: Ed25519 key rotation every 180 days

**Status:** Accepted  
**Date:** 2026-07-04  
**Deciders:** Product spec (driver/seller README), Phase 0 sign-off

## Context

Reports are signed with Ed25519. Key compromise or rotation requires verifying old reports with the key that signed them. OD-004 asked rotation period: 90d / 180d / manual.

## Decision

Rotate signing keys every **180 days**. Each key has a stable `public_key_id` (e.g. `ed25519-2026-01`). **Previous public keys are retained** (read-only in config/secret store) so `GET /v1/reports/{id}/verify` validates against the `public_key_id` stored on the report.

## Consequences

### Positive

- Balanced security vs operational overhead
- Old reports remain verifiable indefinitely
- `public_key_id` on Report enables key lookup without ambiguity

### Negative

- Must manage multiple active public keys in verify path
- Rotation runbook required in Phase 5

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| 90 days | Higher ops burden for internal sales reports |
| Manual only | No predictable security baseline |
| Re-sign all reports on rotation | Impractical; breaks immutability |

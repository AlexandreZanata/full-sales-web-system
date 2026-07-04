# ADR-007: Public report verification endpoint

**Status:** Accepted  
**Date:** 2026-07-04  
**Deciders:** Product spec (driver/seller README §9 Phase 5), Phase 0 sign-off

## Context

Signed reports prove integrity. Verify endpoint can require auth or be public. OD-007 asked: public vs authenticated.

## Decision

`GET /v1/reports/{id}/verify` is **public** (no authentication). Returns only `{ "valid": true | false, "reportId" }` — no PII or full payload. Rate-limited by IP (same sliding window as login, separate key prefix).

Report **metadata and payload** remain protected on `GET /v1/reports/{id}` (authenticated).

## Consequences

### Positive

- Anyone with report ID can confirm signature (shareable proof)
- Matches product roadmap “verificação pública da assinatura”
- Minimal data exposure on public endpoint

### Negative

- Report IDs must be unguessable (UUIDv7) to prevent enumeration
- Rate limiting required to prevent abuse

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| Auth required for verify | Blocks sharing verification links with commerces |
| Return full payload on verify | Unnecessary PII exposure on public route |

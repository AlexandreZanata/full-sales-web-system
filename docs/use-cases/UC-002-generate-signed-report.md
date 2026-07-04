# Use Case: UC-002 — Generate and Verify Signed Report

---

## Metadata

| Field | Value |
|-------|-------|
| ID | UC-002 |
| Actor | Admin |
| Status | Approved (from product spec) |

## Preconditions

- User authenticated with role Admin
- Sales exist in the requested period
- Ed25519 signing key configured in server secrets

## Main flow (happy path)

1. Admin requests report for period (`reportType`, `periodStart`, `periodEnd`, optional filters).
2. Application aggregates sales data for scope (driver, commerce, or consolidated).
3. System serializes data to canonical JSON (deterministic key order).
4. System computes SHA-256 hash and signs with Ed25519 private key (BR-RE-001).
5. System persists Report with `canonical_payload`, `signature`, `public_key_id`.
6. Admin receives Report id and metadata.
7. Any party calls verify endpoint with Report id.
8. System recomputes hash and validates signature — returns `valid: true` (BR-RE-002).

## Alternate flows

### AF-1: Empty period

- **When:** No sales in period
- **Then:** Report generated with zero totals and signed (ADR-003)

### AF-2: Tampered payload

- **When:** Payload altered after generation
- **Then:** Verify returns `valid: false`

## Business rules applied

| Rule ID | Description |
|---------|-------------|
| BR-RE-001 | Canonical JSON + Ed25519 signature |
| BR-RE-002 | Tamper detection on verify |

## Domain events raised

| Event | When |
|-------|------|
| `ReportGenerated` | After step 5 |

## Authorization

| Role | Generate | View | Verify |
|------|----------|------|--------|
| Admin | Yes | Yes | Yes |
| Driver | No | Pre-generated only | Yes |
| Seller | No | No | Yes |

## API mapping

| Step | Endpoint |
|------|----------|
| 1–6 | `POST /v1/reports` |
| 7–8 | `GET /v1/reports/{id}/verify` |
| Export | `GET /v1/reports/{id}/export?format=pdf\|csv\|xlsx` |

## Out of scope

- ICP-Brasil qualified signature

See [DIGITAL-SIGNATURE.md](../DIGITAL-SIGNATURE.md).

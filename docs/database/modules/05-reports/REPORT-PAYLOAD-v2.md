# Report canonical payload — version 2 (declared settlement)

> Promoted from `.local/phases/15-reports-settlement/documentation/`.  
> **Database:** `reports.reports` schema unchanged — expansion is in `canonical_payload` JSON only.

---

## Version

| Field | Value |
|-------|-------|
| `version` | `2` — includes `declaredSettlement` block |

Version 1 payloads (if any) lack `version` or use `1`. Verify endpoint accepts any stored payload; assembly uses v2 from Phase 15 onward.

---

## Top-level fields

| JSON path | Type | Required | Notes |
|-----------|------|----------|-------|
| `version` | number | yes | `2` |
| `period.start` | string (RFC3339 UTC) | yes | Inclusive period start |
| `period.end` | string (RFC3339 UTC) | yes | Inclusive period end |
| `driverId` | UUID string | yes | Report scope |
| `sales` | array | yes | Objective delivery facts (RN9 filtered) |
| `declaredSettlement` | object | yes | Self-reported totals + disclaimer (RN-PAG4) |

---

## `sales[]` item

| Field | Type | Notes |
|-------|------|-------|
| `saleId` | UUID | |
| `orderId` | UUID | Omitted for field sales without order |
| `commerceId` | UUID | |
| `amountCents` | number | Minor units |
| `currency` | string | e.g. `BRL` |

**RN9:** Include only sales where linked order is `Delivered` or `PartiallyDelivered`, or field sale with no order and `Confirmed` status.

---

## `declaredSettlement`

| Field | Type | Notes |
|-------|------|-------|
| `totalDeclaredCents` | number | Sum of declared received amounts |
| `currency` | string | Matches sales currency |
| `byPaymentMethod` | object | Keys: `Cash`, `Pix`, `Card`, `Boleto`, `Other` |
| `disclaimer` | string | Fixed: `Self-declared by seller. Not fiscal or bank proof.` |

Undeclared sales contribute `0` to settlement totals; block is still present.

---

## Signing

Same Ed25519 pipeline as v1 — SHA-256 digest of canonical JSON bytes, then sign. See [DIGITAL-SIGNATURE.md](../../DIGITAL-SIGNATURE.md).

Implementation: `domain-reports` — `ReportAssemblyInput::assemble`, `sign_canonical_payload`, `verify_canonical_payload`.

Tests: `backend/crates/domain-reports/tests/settlement_payload.rs`.

**Updated:** 2026-07-04

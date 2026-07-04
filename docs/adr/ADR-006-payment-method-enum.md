# ADR-006: PaymentMethod enum values

**Status:** Accepted  
**Date:** 2026-07-04  
**Deciders:** Product spec (driver/seller README), Phase 0 sign-off

## Context

Sales record how payment was received (no gateway capture in MVP). OD-006 asked for enum values.

## Decision

```rust
pub enum PaymentMethod {
    Cash,
    Pix,
    Credit,
    Debit,
}
```

Serialized in API as camelCase strings: `"cash"`, `"pix"`, `"credit"`, `"debit"`. Payment is **recorded only** — no payment processor integration in MVP (UC-001 out of scope).

## Consequences

### Positive

- Covers standard Brazilian field payment types
- Stable API schema for reports and filters
- Easy to extend later (`Voucher`, etc.) via new enum variant + migration

### Negative

- No split-payment or mixed methods in one sale (future enhancement)

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| Free-text string | Validation drift and report inconsistency |
| Include `Boleto` | Less common for immediate field sales |

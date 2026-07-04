# Testing Strategy

**Policy:** Read `agent-rules/04-testing/contract-first-tests.md` before writing any test.

---

## Test pyramid

| Level | Scope | Tool | Coverage target |
|-------|-------|------|-----------------|
| Unit (domain) | Entities, VOs, pure rules | `#[cfg(test)]` | **100% — CI blocks merge** |
| Application | Use cases with in-memory fake repos | manual trait mocks | ≥ 90% |
| Integration | Real Postgres/Redis repos | `testcontainers-rs` | All repository adapters |
| E2E | Full HTTP flow | `reqwest` + test server | Critical paths only |

Distribution target: 75% unit / 20% integration / 5% E2E (harness pyramid).

---

## TDD cycle (Red → Green → Refactor)

1. **Red:** Write domain test in GIVEN/WHEN/THEN before implementation exists.
2. **Green:** Minimum code to pass.
3. **Refactor:** Clean up while tests stay green.

Example (maps to BR-IN-001):

```rust
#[test]
fn given_insufficient_stock_when_confirm_sale_then_returns_error() {
    // GIVEN a product with balance of 5 units
    let inventory = InventoryFixture::with_balance(5);

    // WHEN attempting to confirm sale of 10 units
    let result = inventory.register_outbound(10, OutboundReason::Sale);

    // THEN InsufficientStock error, balance unchanged
    assert!(matches!(result, Err(InventoryError::InsufficientStock { .. })));
    assert_eq!(inventory.current_balance(), 5);
}
```

**Contract-first:** assertions come from [BUSINESS-RULES.md](BUSINESS-RULES.md), not from copying production code.

---

## CI gates (non-negotiable)

| Gate | Command |
|------|---------|
| Format | `cargo fmt --check` |
| Lint | `cargo clippy --workspace -- -D warnings` |
| No unwrap in prod | `#![deny(clippy::unwrap_used)]` in lib crates (tests exempt) |
| Domain coverage | `cargo llvm-cov --workspace` — domain crates 100% |
| Security audit | `cargo audit` — no high/critical |

---

## E2E critical scenarios

| ID | Flow |
|----|------|
| E2E-001 | Login → create sale → confirm → stock reduced |
| E2E-002 | Generate report → verify signature valid |
| E2E-003 | Login rate limit triggers after N failures |
| E2E-004 | Cross-tenant access denied (RLS) |

---

## References

- `agent-rules/04-testing/contract-first-tests.md`
- `agent-rules/04-testing/tdd.md`
- `agent-rules/04-testing/test-pyramid.md`
- testcontainers-rs: https://github.com/testcontainers/testcontainers-rs

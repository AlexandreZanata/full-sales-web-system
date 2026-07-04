# Development Commands

Quick reference for local development (after Phase 1 scaffold exists).

---

## Database

```bash
# Run migrations
sqlx migrate run

# Create new migration
sqlx migrate add <name>
```

---

## Tests

```bash
# Domain unit tests (fast, no infra)
cargo test -p domain-identity -p domain-commerces -p domain-inventory -p domain-sales -p domain-reports -p domain-shared

# Integration tests (starts containers)
cargo test --test integration -- --test-threads=1

# E2E
cargo test --test e2e
```

---

## Quality gates (CI parity)

```bash
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo llvm-cov --workspace --html
cargo audit
```

---

## Run API locally

```bash
# From backend/
cargo run -p api-http
```

Requires `.env` with `DATABASE_URL`, `REDIS_URL`, `JWT_SECRET`, `ED25519_PRIVATE_KEY` — see `.env.example` *(Phase 1)*.

---

## Agent harness

```bash
./agent-harness/resolve-rules.sh api auth domain rust
./agent-harness/generate-task-rules.sh api auth sale
```

---

## Phase 0 documentation (pre-scaffold)

```bash
chmod +x scripts/validate-phase0-docs.sh
./scripts/validate-phase0-docs.sh
```

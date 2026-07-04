# Development Commands

Quick reference for local development. All Rust commands run from `backend/`.

---

## Setup

```bash
cd backend
cp .env.example .env   # adjust DATABASE_URL, API_PORT if needed
```

---

## Database

```bash
cd backend

# Run migrations (requires DATABASE_URL)
sqlx migrate run

# Create new migration
sqlx migrate add <name>
```

PostgreSQL 18+ provides native `uuidv7()`. Migrations live in `backend/migrations/`.

---

## Tests

```bash
cd backend

# Domain unit tests (fast, no infra)
cargo test -p domain-shared

# All workspace tests (includes api-http integration tests)
cargo test --workspace

# Domain coverage gate (CI parity)
cargo llvm-cov -p domain-shared --fail-under-lines 100 --summary-only
```

Future phases add: `domain-identity`, `domain-commerces`, integration, E2E.

---

## Quality gates (CI parity)

```bash
cd backend
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo llvm-cov -p domain-shared --fail-under-lines 100 --summary-only
cargo audit
```

---

## Run API locally

```bash
cd backend
cargo run -p api-http
# GET http://127.0.0.1:8080/health → {"status":"ok"}
```

If port 8080 is in use, set `API_PORT` in `.env` (e.g. `18080`).

Requires `.env` with `DATABASE_URL`, `REDIS_URL`, `JWT_SECRET`, `ED25519_PRIVATE_KEY` — see `.env.example`. Only `API_HOST` / `API_PORT` are required for `/health` in Phase 1.

---

## Agent harness

```bash
./agent-harness/resolve-rules.sh api auth domain rust
./agent-harness/generate-task-rules.sh api auth sale
```

---

## Phase 0 documentation validation

```bash
./scripts/validate-phase0-docs.sh
```

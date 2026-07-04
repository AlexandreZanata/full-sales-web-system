# Development Commands

Quick reference for local development.

---

## Monorepo (root)

```bash
pnpm install
pnpm verify          # lint + test + build + Rust checks
pnpm validate:openapi # OpenAPI 3.1 schema validation (swagger-cli)
pnpm dev             # API (Rust) + web (Vite) in parallel
pnpm lint && pnpm test && pnpm build
```

| App | Command | URL |
|-----|---------|-----|
| API | `pnpm dev:api` or `cd backend && cargo run -p api-http` | `http://127.0.0.1:8080/health` |
| Web | `pnpm dev:web` | `http://127.0.0.1:5173` |

Copy `.env.example` (root) and `backend/.env.example` for local configuration.

---

## Backend (Rust)

All Rust commands run from `backend/`.

## Database

```bash
cd backend

# Run migrations (requires DATABASE_URL)
sqlx migrate run

# RLS integration tests (requires Docker)
cargo test -p infra-postgres --test integration rls_

# Create new migration (requires matching MIGRATION-SPEC in phase 1b docs)
sqlx migrate add <module>_<name>
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

RLS isolation (Phase 1b):

```bash
cd backend
cargo test -p infra-postgres --test integration rls_
```

---

## Quality gates (CI parity)

```bash
pnpm validate:openapi   # from repo root — validates docs/openapi.yaml

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

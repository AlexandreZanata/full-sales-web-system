# Development Commands

Quick reference for local development.

---

## Monorepo (root)

```bash
pnpm install
pnpm verify          # lint + test + build + Rust checks
pnpm validate:openapi # OpenAPI 3.1 schema validation (swagger-cli)
pnpm dev             # API (Rust) + web (Vite) in parallel
./scripts/dev-frontends.sh   # all Vite frontends only (HMR, no Docker)
pnpm dev:frontends   # same as the script above
pnpm lint && pnpm test && pnpm build
```

| App | Command | URL |
|-----|---------|-----|
| **All frontends** | `./scripts/dev-frontends.sh` or `pnpm dev:frontends` | web + admin + portal + field + platform-admin (Vite HMR) |
| API | `pnpm dev:api` or `cd backend && cargo run -p api-http` | `http://127.0.0.1:8080/health` |
| Web | `pnpm dev:web` | `http://127.0.0.1:5173` |
| Admin | `pnpm dev:admin` | `http://127.0.0.1:5174` |
| Portal | `pnpm dev:portal` | `http://127.0.0.1:5175` |
| Field | `pnpm dev:field` | `http://127.0.0.1:5176` |
| Platform Admin | `pnpm dev:platform-admin` | `http://127.0.0.1:5177` |

Admin quality gates:

```bash
pnpm --filter @full-sales/admin lint test build
pnpm test:e2e:admin    # Playwright — login, orders, mobile nav
```

See [features/admin-panel.md](features/admin-panel.md) for routes, i18n, and E2E details.

Portal and field PWAs (Phase 39):

```bash
pnpm --filter @full-sales/portal lint test build
pnpm --filter @full-sales/field lint test build
pnpm test:e2e:portal
pnpm test:e2e:field
```

See [features/client-apps.md](features/client-apps.md) and [features/seller-mobile-app.md](features/seller-mobile-app.md).

### Mobile — Kotlin Multiplatform

| App | Path | Check |
|-----|------|-------|
| Field (driver) | `apps-mobile/field` | `cd apps-mobile/field && ./gradlew :shared:check :androidApp:assembleDebug` |
| Seller | `apps-mobile/seller` | `pnpm mobile:seller:check` (shared check + composeApp compile + lint + assembleDebug) |

Seller iOS shared compile (CI — macOS runner):

```bash
cd apps-mobile/seller && ./gradlew :shared:compileKotlinIosSimulatorArm64 :composeApp:compileKotlinIosSimulatorArm64
```

| Platform | API base URL |
|----------|----------------|
| Android emulator | `http://10.0.2.2:8080/v1` |
| iOS simulator | `http://127.0.0.1:8080/v1` |
| iOS physical device | `http://<host-lan-ip>:8080/v1` |

Copy `.env.example` (root) and `backend/.env.example` for local configuration.

### Dev database seed (Phase 37)

Populates **Dev Tenant** with data for every admin screen. Requires Postgres + migrations applied.

```bash
pnpm seed:dev
```

Guard: set `ALLOW_DEV_SEED=1` (the script sets it). Never use in production.

| Role | Email | Password |
|------|-------|----------|
| Admin | `admin@test.com` | `secret123` |
| Driver A | `driver-a@test.com` | `secret123` |
| Driver B | `driver-b@test.com` | `secret123` |
| Seller | `seller@test.com` | `secret123` |
| Commerce contact | `portal@seed-store.com` | `secret123` |

Re-running is idempotent (skips when seed data is already present). Conflicting dev emails in other tenants are removed on seed.

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

---

## Platform SaaS E2E (Phase 13)

```bash
cd backend && cargo test -p api-http platform_saas -- --test-threads=1
./scripts/platform-saas-acceptance.sh
./scripts/platform-saas-payment-e2e.sh   # manual Asaas sandbox steps
./scripts/platform-saas-domain-e2e.sh    # manual custom domain steps
```

Optional live sandbox: `ASAAS_SANDBOX=1 cargo test -p api-http platform_saas_sandbox -- --ignored`

See [features/platform-saas-e2e.md](features/platform-saas-e2e.md).

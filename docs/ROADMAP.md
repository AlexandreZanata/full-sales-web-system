# Roadmap

Build phases aligned with domain dependencies. Local task breakdown: `.local/phases/`.

---

## Phase 1 — Foundation

- Cargo workspace + CI (`fmt`, `clippy`, `llvm-cov`)
- `domain-shared`: `Money`, `TenantId`, base `DomainError`
- Initial migrations (UUIDv7, RLS enabled)
- `GET /health`

**Exit criteria:** CI green; domain-shared 100% unit coverage.

---

## Phase 2 — Identity & Commerces

- User registration (admin), login (JWT + refresh)
- Commerce registration with CNPJ validation
- RBAC middleware skeleton
- Domain tests 100% for `domain-identity`, `domain-commerces`

**Exit criteria:** E2E login; admin can create commerce; driver cannot.

---

## Phase 3 — Inventory

- Product CRUD (admin)
- StockMovement, balance calculation
- Redis stock cache (invalidate-on-write)

**Exit criteria:** BR-IN-001..003 tested; integration tests with testcontainers.

---

## Phase 4 — Sales ✅

- Full flow: driver registers sale → confirm → stock deduction
- Transactional sale + outbound movement (BR-IN-002)
- Idempotency on `POST /v1/sales`

**Exit criteria:** E2E-001 green; contract tests for all sale endpoints.

---

## Phase 5 — Reports & Signature

- Report generation + Ed25519 signing
- `GET /v1/reports/{id}/verify`
- E2E-002 signature verification

**Exit criteria:** BR-RE-001..002; canonical JSON tests.

---

## Phase 6 — Observability & Hardening

- Structured tracing + request correlation
- Audit log (who did what, when)
- Security review: rate limits, RLS audit, secrets rotation doc

**Exit criteria:** OWASP threat table in [SECURITY.md](SECURITY.md) fully mitigated.

---

## Future (backlog)

- Web/mobile client (simple UI — complexity stays in backend)
- ICP-Brasil integration (if legally required)
- Scheduled daily report jobs
- Payment gateway adapters

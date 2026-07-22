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
- Audit log (`audit.events` schema — DB layer in Phase 1c; API wiring in Phase 6)
- Security review: rate limits, RLS audit, secrets rotation doc

**Exit criteria:** OWASP threat table in [SECURITY.md](SECURITY.md) fully mitigated.

---

---

## Phase 7 — Mobile clients ✅ (seller KMP)

- KMP seller app (`apps-mobile/seller`) — Seller-only auth, **local-first SQLite** (Phase 16), M3 UI
- Compose Multiplatform shared UI (`composeApp`) — Android Room + iOS SQLDelight LocalStore
- CI: `seller-kmp`, `seller-ios` jobs; quality gate in `pnpm mobile:seller:check`
- Docs: [features/seller-mobile-app.md](features/seller-mobile-app.md), [ADR-051](adr/ADR-051-seller-kmp-app.md) (incl. Phase 16 amendment)
- Play readiness: [docs/mobile/seller-play-store.md](mobile/seller-play-store.md) — release AAB, HTTPS API, `pnpm mobile:seller:play-preflight`

**Exit criteria:** Shared unit tests (API, sync, repositories); Android lint + assemble; iOS simulator compile; documented routes and manual acceptance script.

---

## Future (backlog)

- Web/mobile client (simple UI — complexity stays in backend)
- ICP-Brasil integration (if legally required)
- Scheduled daily report jobs
- Payment gateway adapters

---

## Platform SaaS (Phases 1–13) ✅

Local task breakdown: `.local/phases/1-super-admin-rbac/` … `13-integration-e2e/`.

| Phase | Scope | Exit |
|-------|-------|------|
| 1 | PlatformAdmin auth, MFA, impersonation, RLS bypass | `platform_auth.rs` |
| 2 | Tenant provision, lifecycle, suspension gate | `tenant_lifecycle.rs` |
| 3 | Asaas webhook, idempotency | `billing_webhook.rs` |
| 4 | Subscription billing, dunning, tenant billing API | `billing_subscription.rs` |
| 5 | Tenant Asaas connect, portal checkout | `billing_tenant_payments.rs` |
| 6 | Fraud velocity, blocklist | `platform_fraud.rs` |
| 7 | Custom domains, DNS verify | `tenant_domains.rs` |
| 8 | Platform operations (users, support, maintenance) | `platform_operations.rs` |
| 9 | Health matrix, readiness probes | `health_monitoring.rs` |
| 10 | Audit trail, LGPD export | `audit_compliance.rs` |
| 11 | Platform Admin SPA | [features/platform-admin-ui.md](features/platform-admin-ui.md) |
| 12 | Tenant billing UI (admin) | [features/tenant-billing-ui.md](features/tenant-billing-ui.md) |
| 13 | Integration E2E, PO acceptance | [features/platform-saas-e2e.md](features/platform-saas-e2e.md) |

**Phase 13 exit gate:** PO signs acceptance checklist; `cargo test -p api-http platform_saas` green in CI.

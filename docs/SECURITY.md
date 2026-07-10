# Security

Aligned with OWASP Top 10:2025 — see `agent-rules/03-security/README.md`.

---

## Controls by layer

| Layer | Measure |
|-------|---------|
| Passwords | Argon2id; never log plaintext; never return in errors |
| Tokens | JWT access 15 min + opaque refresh in Redis (revocable logout) |
| Authorization | RBAC by `Role` in **middleware** — never frontend-only |
| Rate limiting | Redis sliding window on login and report generation |
| Multi-tenant isolation | PostgreSQL RLS on all sensitive tables |
| PII | CNPJ and contact data masked in logs (`Debug` impl) |
| Transport | TLS mandatory in production (Caddy/Nginx) |
| Signing keys | Ed25519 private key in secret manager only |

---

## RBAC enforcement

```
Request → auth middleware (JWT validate)
       → tenant middleware (set app.tenant_id for RLS)
       → role middleware (check Role vs route policy)
       → handler
```

Application layer **re-checks** authorization for use cases — defense in depth (BOLA prevention).

### Tenant roles (`Role`)

| Role | Scope |
|------|-------|
| `Admin` | Full tenant management |
| `Driver` | Field sales + assigned stock |
| `Seller` | Field sales + order creation |
| `CommerceContact` | Own commerce portal |

### PlatformAdmin (Phase 1 — ADR-013, ADR-016)

| Actor | Auth | `tenant_id` in JWT | RLS |
|-------|------|-------------------|-----|
| **PlatformAdmin** | `POST /v1/platform/auth/login` + MFA | No | `app.bypass_rls = true` on `/v1/platform/*` |
| **Impersonation** | `POST /v1/platform/impersonate` | `actingTenantId` + `Admin` role | Tenant-scoped — **no** bypass |
| **Tenant users** | `POST /v1/auth/login` | Yes | `app.tenant_id` from JWT |

### Authorization matrix (Phase 1)

| Route prefix | PlatformAdmin | Impersonating Admin | Tenant Admin | Driver/Seller |
|--------------|---------------|---------------------|--------------|---------------|
| `/v1/platform/*` | Yes (MFA) | No — use platform token | **Forbidden** | **Forbidden** |
| `/v1/users`, `/v1/commerces`, … | No — impersonate first | Yes (scoped tenant) | Yes | Role-limited |
| `/v1/auth/*` | Tenant flow only | Tenant flow only | Yes | Yes |

**Tests:** `backend/crates/api-http/tests/platform_auth.rs`, `platform_auth_matrix.rs`, `auth_matrix.rs`.

### Tenant suspension gate (Phase 2 — ADR-015, BR-PL-001)

| Tenant `status` | Mutating `/v1/*` | `GET` reads | `/v1/billing/*` |
|-----------------|------------------|-------------|-----------------|
| `Trial`, `Active`, `PastDue` | Allowed | Allowed | Allowed |
| `Suspended`, `Offboarding` | **403 `TENANT_SUSPENDED`** | Allowed | Allowed |
| `Deleted` | Blocked | Denied | Denied |

Middleware: `tenant_gate_middleware` after `auth_middleware` on protected tenant routes.

**Tests:** `backend/crates/api-http/tests/tenant_lifecycle.rs`.

### Asaas webhook (Phase 3 — ADR-014, BR-BI-001)

| Control | Implementation |
|---------|----------------|
| Auth | `asaas-access-token` header — constant-time compare vs `ASAAS_WEBHOOK_TOKEN` |
| Idempotency | `billing.payment_events.asaas_event_id` UNIQUE — duplicate → 200 no-op |
| PII in logs | API keys masked via `infra-asaas::sanitize` |

**Tests:** `backend/crates/api-http/tests/billing_webhook.rs`, `infra-asaas/tests/asaas_client.rs`.

---

## JWT + refresh flow

| Token | Storage | TTL | Revocation |
|-------|---------|-----|------------|
| Access JWT | Client | 15 min | Expires naturally |
| Refresh opaque | Redis `session:{user_id}` | 7 days | Delete on logout |

Blacklist compromised access tokens in Redis if needed before expiry.

---

## Row-Level Security

Every request sets Postgres session variable:

```sql
SET app.tenant_id = '<uuid>';
```

RLS policies filter all tenant-scoped tables. See [ARCHITECTURE.md](ARCHITECTURE.md).

---

## Threat mapping (initial)

| OWASP | Threat | Mitigation |
|-------|--------|------------|
| A01 Broken Access Control | Cross-tenant sale access | RLS + RBAC middleware |
| A02 Security Misconfiguration | Debug endpoints in prod | CI + env separation |
| A03 Supply Chain | Crate vulnerabilities | `cargo audit` in CI |
| A04 Cryptographic Failures | Weak passwords | Argon2id |
| A07 Authentication Failures | Brute force login | Rate limit + lockout |
| A09 Security Logging | Missing audit trail | Structured tracing + audit log (Phase 6) |

---

## References

- `agent-rules/03-security/authentication.md`
- `agent-rules/03-security/authorization.md`
- `agent-rules/03-security/secrets-and-credentials.md`
- `agent-rules/03-security/audit-logging.md`

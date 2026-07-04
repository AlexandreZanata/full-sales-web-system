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

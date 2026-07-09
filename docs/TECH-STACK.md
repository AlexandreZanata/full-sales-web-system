# Technology Stack

---

## Core choices

| Layer | Choice | Rationale |
|-------|--------|-----------|
| Language | **Rust** (edition 2024) | Memory safety, performance, strong types for domain modeling |
| Web framework | **Axum** | Native Tokio integration, typed extractors, composable middleware |
| Database | **PostgreSQL 18.4** | Native UUIDv7 (`uuidv7()`), performance, Row-Level Security |
| Cache / sessions | **Redis 8.8** | Stock cache, light queues, rate limiting, token blacklist |
| DB driver | **sqlx** (compile-time checked queries) | SQL errors at compile time, not in production |
| Async runtime | **Tokio** | De facto Rust async standard |
| Authentication | **JWT (access) + opaque refresh token in Redis** | Stateless validation, revocable via Redis |
| Password hashing | **Argon2id** (`argon2` crate) | OWASP recommended, GPU-resistant |
| Digital signature | **Ed25519** (`ed25519-dalek`) | Fast, secure, open source, no CA cost |
| Observability | **tracing** + **tracing-subscriber** | Structured logs, `request_id` correlation |
| Migrations | **sqlx-cli** | Schema versioning alongside code |
| Integration tests | **testcontainers-rs** | Real Postgres/Redis per test run |

---

## External references

| Topic | URL |
|-------|-----|
| Rust edition guide | https://doc.rust-lang.org/edition-guide/ |
| Axum | https://docs.rs/axum/latest/axum/ |
| sqlx | https://github.com/launchbadge/sqlx |
| PostgreSQL RLS | https://www.postgresql.org/docs/current/ddl-rowsecurity.html |
| PostgreSQL UUIDv7 | https://www.postgresql.org/docs/current/datatype-uuid.html |
| Argon2 (OWASP) | https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html |
| Ed25519 | https://ed25519.cr.yp.to/ |

---

## CNPJ validation

Reuse logic from [`@br-validators`](https://github.com/AlexandreZanata/br-validators) for CNPJ Value Object — includes RFB alphanumeric format support.

## CNPJ lookup upstream (Phase 70)

`GET /v1/commerces/cnpj-lookup` proxies to a configurable provider behind `CnpjLookupProvider`:

| Provider | Env | Notes |
|----------|-----|-------|
| `brasilapi` (default) | `CNPJ_LOOKUP_URL` optional | No API key |
| `opencnpj` | `CNPJ_LOOKUP_URL`, `CNPJ_LOOKUP_API_KEY` required | [OpenCNPJ](https://github.com/AlexandreZanata/OpenCNPJ) at `https://api.comerc.app.br` |
| `mock` | — | CI and local contract tests |

API key is server-side only; clients use the unchanged Full Sales lookup contract.

---

## Production deployment (planned)

| Component | Choice |
|-----------|--------|
| TLS termination | Caddy or Nginx reverse proxy |
| Secrets | Environment variables / secret manager (Ed25519 private key never in DB) |

See [SECURITY.md](SECURITY.md) and [DIGITAL-SIGNATURE.md](DIGITAL-SIGNATURE.md).

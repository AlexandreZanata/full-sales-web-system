# Architecture

**Style:** DDD + Hexagonal (Ports & Adapters)

---

## Layer diagram

```
┌─────────────────────────────────────────────────────┐
│              Interface (HTTP / Axum)                 │
│     handlers, DTOs, extractors, middlewares        │
└───────────────────────┬─────────────────────────────┘
                        │ calls
┌───────────────────────▼─────────────────────────────┐
│            Application (Use Cases)                   │
│   orchestrates domain, transactions, publishes events│
└───────────────────────┬─────────────────────────────┘
                        │ uses
┌───────────────────────▼─────────────────────────────┐
│              Domain (pure core)                      │
│  Entities, Value Objects, Aggregates, business rules │
│     ZERO infra imports (no sqlx, no axum, no redis)  │
└───────────────────────▲─────────────────────────────┘
                        │ implemented by
┌───────────────────────┴─────────────────────────────┐
│                 Infrastructure                       │
│  Repositories (sqlx/Postgres), Cache (Redis),        │
│  Signature (Ed25519), Clock, IDs (UUIDv7)            │
└─────────────────────────────────────────────────────┘
```

**Non-negotiable:** `domain-*` crates never import `sqlx`, `axum`, or `redis`. Domain defines **traits** (ports); infrastructure implements **adapters**.

---

## Error handling: never-throw / Result

No `panic!`, `.unwrap()`, or `.expect()` in production code. All domain and application functions return `Result<T, DomainError>` with typed enum errors.

```rust
pub enum InventoryError {
    InsufficientStock {
        product_id: ProductId,
        available: u32,
        requested: u32,
    },
    InactiveProduct(ProductId),
    CommerceNotFound(CommerceId),
}
```

---

## Multi-tenancy: Row-Level Security (RLS)

**Not** schema-per-tenant — operational simplicity first:

- Single database, single migration stream.
- Sensitive tables include `tenant_id UUID NOT NULL`.
- **PostgreSQL RLS** ensures a session only sees rows for its `tenant_id`, even if application code has a bug.

```sql
ALTER TABLE sales ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation ON sales
    USING (tenant_id = current_setting('app.tenant_id')::uuid);
```

See [ADR-002](adr/ADR-002-tenant-platform-org.md) for `tenant_id` semantics.

---

## Repository layout (hybrid monorepo)

See [ADR-008](adr/ADR-008-hybrid-monorepo.md).

```
/
├── backend/                  # Rust Cargo workspace (domain, API, infra)
├── apps/
│   ├── api/                  # pnpm meta-package → delegates to backend/crates/api-http
│   └── web/                  # Vite + React client shell
├── packages/
│   ├── domain/               # TS client types (no HTTP/ORM)
│   └── application/          # TS ApplicationError + ports
├── package.json              # pnpm root scripts: lint, test, build, verify
└── pnpm-workspace.yaml
```

| Layer | Path | Runtime |
|-------|------|---------|
| Domain (business rules) | `backend/crates/domain-*` | Rust |
| Application (use cases) | `backend/crates/application` | Rust |
| HTTP `/v1/` | `backend/crates/api-http` | Axum |
| Web UI | `apps/web` | React |

---

## Cargo workspace layout

```
backend/
├── Cargo.toml                     # workspace root
├── crates/
│   ├── domain-identity/           # User, Role, auth rules
│   ├── domain-commerces/          # Commerce, Cnpj VO
│   ├── domain-inventory/          # Product, StockMovement
│   ├── domain-sales/              # Sale, SaleItem
│   ├── domain-reports/            # Report, signature trait
│   ├── domain-shared/             # Money, TenantId, base DomainError
│   ├── application/               # Use cases
│   ├── infra-postgres/            # sqlx repository adapters
│   ├── infra-redis/               # Cache, sessions, rate limit
│   ├── infra-crypto/              # Ed25519 adapter
│   └── api-http/                  # Axum routes, main.rs
├── migrations/                    # sqlx migrations
├── tests/
│   ├── integration/
│   └── e2e/
└── docs/
    └── adr/
```

---

## Domain events

Events are past tense, immutable, raised by aggregates:

| Event | Context | When |
|-------|---------|------|
| `SaleConfirmed` | Sales | Sale moves to Confirmed |
| `StockMovementRecorded` | Inventory | Entry/exit recorded |
| `ReportGenerated` | Reports | Report persisted with signature |

Full catalog: extend in [DOMAIN-MODEL.md](DOMAIN-MODEL.md) as implemented.

---

## References

- Harness: `agent-rules/AGENT-CORE-PRINCIPLES.md`
- Layering: `agent-rules/02-architecture/layering.md`
- Bounded contexts: `agent-rules/02-architecture/bounded-contexts.md`

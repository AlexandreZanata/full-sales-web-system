# Architecture Decision Records

Store resolved decisions from [OPEN-DECISIONS.md](../OPEN-DECISIONS.md) here.

Template: `agent-rules/11-documentation-and-glossary/adr-template.md`

| ADR | Title | Status |
|-----|-------|--------|
| [ADR-001](ADR-001-stock-balance-materialized.md) | Stock balance — materialized + Redis cache | Accepted |
| [ADR-002](ADR-002-tenant-platform-org.md) | Tenant = platform owner organization | Accepted |
| [ADR-003](ADR-003-report-on-demand.md) | Report generation on-demand via API | Accepted |
| [ADR-004](ADR-004-ed25519-key-rotation.md) | Ed25519 key rotation every 180 days | Accepted |
| [ADR-005](ADR-005-inventory-driver-scope.md) | Inventory scoped per driver | Accepted |
| [ADR-006](ADR-006-payment-method-enum.md) | PaymentMethod enum values | Accepted |
| [ADR-007](ADR-007-public-report-verify.md) | Public report verification endpoint | Accepted |
| [ADR-008](ADR-008-hybrid-monorepo.md) | Rust backend + pnpm web client monorepo | Accepted |
| [ADR-009](ADR-009-ephemeral-state-redis.md) | Redis for ephemeral state; outbox deferred | Accepted |
| [ADR-010](ADR-010-stock-reservation-tenant-pool.md) | Stock reservations — tenant pool until driver assigned | Accepted |
| [ADR-011](ADR-011-object-storage-minio.md) | Self-hosted MinIO for object storage | Accepted |
| [ADR-012](ADR-012-commerce-registration-workflow.md) | Seller commerce registration + admin review | Accepted |

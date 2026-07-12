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
| [ADR-013](ADR-013-platform-admin-identity.md) | PlatformAdmin identity model | Accepted |
| [ADR-014](ADR-014-asaas-platform-billing.md) | Asaas platform billing + webhooks | Accepted |
| [ADR-015](ADR-015-tenant-lifecycle.md) | Tenant lifecycle state machine | Accepted |
| [ADR-016](ADR-016-platform-admin-rls-bypass.md) | RLS bypass for PlatformAdmin | Accepted |
| [ADR-017](ADR-017-custom-domain-verification.md) | Custom domain verification + routing | Accepted |
| [ADR-018](ADR-018-tenant-asaas-payments.md) | Tenant payment collection via Asaas | Accepted |
| [ADR-019](ADR-019-nginx-cloudflare-edge.md) | Nginx Ingress + Cloudflare edge | Accepted |

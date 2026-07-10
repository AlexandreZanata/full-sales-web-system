# Platform SaaS integration and E2E (Phase 13)

> Aggregated contract suite + manual PO acceptance scripts.

## Automated contract suite

```bash
cd backend
cargo test -p api-http platform_saas -- --test-threads=1
./scripts/platform-saas-acceptance.sh   # from repo root
```

| Module | Path | Coverage |
|--------|------|----------|
| Lifecycle | `tests/platform_saas/lifecycle.rs` | provision → trial → pay → active → suspend → reactivate; dunning |
| Isolation | `tests/platform_saas/isolation.rs` | RLS cross-tenant; platform lists both tenants |
| Impersonation | `tests/platform_saas/impersonation_audit.rs` | Scoped token + audit trail |
| Webhook/fraud/domain | `tests/platform_saas/webhook_fraud_domain.rs` | Idempotency, fraud velocity, mock DNS verify |

Related per-phase tests: `tenant_lifecycle`, `billing_*`, `platform_*`, `audit_compliance`, `health_monitoring`, `tenant_domains`.

## Manual scripts

| Script | Purpose |
|--------|---------|
| `scripts/platform-saas-payment-e2e.sh` | Asaas sandbox payment + dunning flow |
| `scripts/platform-saas-domain-e2e.sh` | Custom domain + portal on host |
| `scripts/platform-saas-acceptance.sh` | Runs contract suite + prints PO checklist |

## Optional Asaas sandbox CI

```bash
ASAAS_SANDBOX=1 ASAAS_API_KEY=... cargo test -p api-http platform_saas_sandbox -- --ignored
```

GitHub Actions: optional `platform-saas-sandbox` job (manual / secrets).

## PO acceptance

Checklist in `.local/phases/13-integration-e2e/TASKS.md`. Record sign-off when manual steps complete.

## Known limitations

- Payment method list/set-default not in tenant billing API v1 (attach-only)
- LGPD export job is async — poll until `completed`
- Live Asaas sandbox requires credentials; CI uses mocks
- Custom domain E2E in production requires real DNS + Caddy — see [deployment/caddy-custom-domains.md](../deployment/caddy-custom-domains.md)

**Implemented:** Phase 13.

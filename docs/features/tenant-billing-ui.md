# Tenant billing UI (Admin)

> Extends `apps/admin` — Phase 12.

## Routes

| Path | Purpose |
|------|---------|
| `/settings` | General site settings (existing) |
| `/settings/billing` | Platform subscription, invoices, attach card token |
| `/settings/payments` | Tenant Asaas connect, checkout toggles, balance, fraud alerts |
| `/settings/domains` | Custom domains (Pro+) |

Settings layout includes section nav (General, Billing, Payments, Domains).

## Billing banner

`BillingStatusBanner` in `AdminShell` shows when subscription `tenantStatus` or `status` is `PastDue`, or tenant is `Suspended`. Links to `/settings/billing`.

## Plan gating

Starter plan (`plan.code === 'starter'`) hides online payment settings on `/settings/payments` with upgrade mailto (ADR-015 / 0-OD-006).

## APIs

- `/v1/billing/*` — subscription, invoices, payment-methods
- `/v1/settings/payments/*` — Asaas connect, balance, transactions
- `/v1/settings/domains/*` — domain CRUD
- `/v1/fraud/alerts` — tenant fraud flags

## Tests

`apps/admin/tests/unit/billing-*.test.ts`, `payments-api.test.ts`, `settings-billing-routes.test.ts`

**Implemented:** Phase 12.

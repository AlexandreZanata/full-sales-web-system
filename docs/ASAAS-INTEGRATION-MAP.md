# Asaas integration map

> Phase 0E reference. Implementation: Phases 3–5.  
> Official docs: [https://docs.asaas.com](https://docs.asaas.com)

---

## Environments

| Environment | Base URL | API key source |
|-------------|----------|----------------|
| Sandbox | `https://api-sandbox.asaas.com/v3` | Asaas sandbox dashboard → Integrações |
| Production | `https://api.asaas.com/v3` | Production account (not in repo) |

**Auth header:** `access_token: <ASAAS_API_KEY>` (Asaas convention — not Bearer).

**User-Agent:** `FullSales/<version>` (required by Asaas).

---

## Platform billing — outbound endpoints (v1)

| Operation | Method | Asaas path | When |
|-----------|--------|------------|------|
| Create customer | `POST` | `/v3/customers` | Tenant provision — `externalReference` = `TenantId` |
| Update customer | `PUT` | `/v3/customers/{id}` | Tenant legal name / CNPJ change |
| Create subscription | `POST` | `/v3/subscriptions` | Attach plan after customer |
| Update subscription | `PUT` | `/v3/subscriptions/{id}` | Plan change |
| Cancel subscription | `DELETE` | `/v3/subscriptions/{id}` | Offboard / downgrade |
| List payments | `GET` | `/v3/payments?subscription={id}` | Invoice sync / support |
| Tokenize card | `POST` | `/v3/creditCard/tokenize` | Tenant Admin adds payment method |
| Create webhook | `POST` | `/v3/webhooks` | One-time platform setup |

### Customer create (minimal)

```json
{
  "name": "Acme Distribuidora LTDA",
  "cpfCnpj": "12345678000199",
  "email": "billing@acme.example",
  "externalReference": "0192a1b2-c3d4-7890-abcd-ef1234567890"
}
```

### Subscription create (minimal)

```json
{
  "customer": "cus_000005401844",
  "billingType": "PIX",
  "value": 199.9,
  "cycle": "MONTHLY",
  "description": "Full Sales Pro",
  "externalReference": "0192a1b2-c3d4-7890-abcd-ef1234567890"
}
```

---

## Tenant payment collection — outbound (Pro+)

Uses **tenant's own** `ASAAS_API_KEY` (decrypted server-side). Same endpoints; scoped to tenant account.

| Operation | Method | Asaas path | When |
|-----------|--------|------------|------|
| Validate key | `GET` | `/v3/myAccount` | Tenant Admin connects Asaas (Phase 5) |
| Create payment | `POST` | `/v3/payments` | Portal checkout for order total |
| Balance | `GET` | `/v3/finance/balance` | Tenant Admin settlement view (read-only) |
| Transactions | `GET` | `/v3/financialTransactions` | Tenant Admin settlement history |
| Refund | `POST` | `/v3/payments/{id}/refund` | Admin-initiated refund v1 |

---

## Inbound webhooks

**Our endpoint:** `POST /v1/billing/webhooks/asaas` (public; token-gated).

### Authentication

Asaas does **not** sign the body with HMAC. Configure `authToken` on webhook creation; Asaas sends:

```http
asaas-access-token: <ASAAS_WEBHOOK_TOKEN>
```

Validate with **constant-time** string compare against `ASAAS_WEBHOOK_TOKEN` env var. Reject missing or wrong token with `401`.

Optional hardening: IP allowlist for Asaas egress ranges (document in deployment runbook).

### Payload shape (representative)

```json
{
  "id": "evt_6b2c8f2a-9e4d-4a1b-8c3d-1f2e3d4c5b6a",
  "event": "PAYMENT_CONFIRMED",
  "dateCreated": "2026-07-10T14:30:00Z",
  "payment": {
    "id": "pay_123",
    "customer": "cus_000005401844",
    "subscription": "sub_456",
    "status": "CONFIRMED",
    "value": 199.9,
    "externalReference": "0192a1b2-c3d4-7890-abcd-ef1234567890"
  }
}
```

### Events handled (v1)

| Event group | Events | Action |
|-------------|--------|--------|
| `PAYMENT_*` | `CREATED`, `CONFIRMED`, `RECEIVED`, `OVERDUE`, `DELETED`, `REFUNDED` | Update invoice + tenant/subscription status |
| `SUBSCRIPTION_*` | `CREATED`, `UPDATED`, `DELETED` | Sync `billing.subscriptions` |
| `INVOICE_*` | `CREATED`, `UPDATED`, `AUTHORIZED`, `CANCELED` | Mirror fiscal invoice rows (if enabled) |

Return **200** quickly; heavy work async if needed. Duplicate `id` → 200 no-op (BR-BI-001).

---

## Idempotency

| Layer | Key | Storage |
|-------|-----|---------|
| Webhook | `payment_events.asaas_event_id` | `UNIQUE` constraint — insert-or-skip |
| Outbound create | `Idempotency-Key` header (our UUID) | Optional Redis 24h for customer/subscription create |

---

## Sandbox credentials (dev / CI)

1. Create free account at [https://sandbox.asaas.com](https://sandbox.asaas.com).
2. Copy API key → `ASAAS_API_KEY` in `backend/.env`.
3. Register webhook via API or UI pointing to dev tunnel URL.
4. Store returned `authToken` → `ASAAS_WEBHOOK_TOKEN`.
5. CI: use GitHub Actions secrets; integration tests mock Asaas or use recorded fixtures.

**Never commit** real keys. `backend/.env.example` has placeholders only.

---

## Rate limits and retries

| Direction | Policy |
|-----------|--------|
| Outbound | Exponential backoff: 1s, 2s, 4s — max 3 retries on 429/5xx |
| Outbound concurrency | Max 5 parallel requests per process |
| Webhook inbound | Process in < 5s p99; queue if DB slow |
| Polling (invoice sync) | Not used v1 — webhook-driven only |

Asaas may pause webhook queue on repeated 4xx/5xx — monitor `interrupted` flag via `GET /v3/webhooks`.

---

## Error mapping

| Asaas error | Our code |
|-------------|----------|
| `invalid_access_token` | Internal — rotate `ASAAS_API_KEY` |
| Customer not found | Re-provision customer from `TenantId` |
| Subscription inactive | `SUBSCRIPTION_PAST_DUE` on tenant APIs |

---

**Version:** 0.1 — 2026-07-10

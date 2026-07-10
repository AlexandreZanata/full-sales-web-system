# Asaas integration

> Phase 3 — platform billing via single Asaas account ([ADR-014](../adr/ADR-014-asaas-platform-billing.md)).

## Sandbox setup

1. Create account at [https://sandbox.asaas.com](https://sandbox.asaas.com).
2. Copy API key → `ASAAS_API_KEY` in `backend/.env`.
3. Set `ASAAS_BASE_URL=https://api-sandbox.asaas.com/v3`.
4. Register webhook pointing to your dev URL:
   - `POST https://<host>/v1/billing/webhooks/asaas`
   - Use the `authToken` returned by Asaas → `ASAAS_WEBHOOK_TOKEN`.
5. Asaas sends `asaas-access-token: <token>` on each webhook (not HMAC).

## Environment variables

| Variable | Required | Description |
|----------|----------|-------------|
| `ASAAS_API_KEY` | Prod / sandbox integration | Outbound API key (`access_token` header) |
| `ASAAS_BASE_URL` | No | Default `https://api-sandbox.asaas.com/v3` |
| `ASAAS_WEBHOOK_TOKEN` | Webhook endpoint | Constant-time validated inbound token |
| `ASAAS_TIMEOUT_SECS` | No | HTTP timeout (default 15) |
| `ASAAS_MAX_RETRIES` | No | Retries on 429/5xx (default 3) |
| `ASAAS_CIRCUIT_THRESHOLD` | No | Failures before circuit opens (default 5) |
| `BILLING_CREDENTIALS_MASTER_KEY` | Tenant payment connect (Phase 5) | Base64-encoded 32-byte AES key for encrypting per-tenant Asaas API keys at rest (ADR-018) |

When `ASAAS_API_KEY` is unset, local dev uses `MockPaymentGateway` (customer ids `cus_mock_{tenantId}`).

## Manual webhook test

```bash
export ASAAS_WEBHOOK_TOKEN=whsec_dev_test_token_min_32_chars_xx
curl -s -X POST http://127.0.0.1:8080/v1/billing/webhooks/asaas \
  -H "asaas-access-token: $ASAAS_WEBHOOK_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"id":"evt_manual_1","event":"PAYMENT_CONFIRMED","payment":{"id":"pay_1","externalReference":"'"$TENANT_UUID"'"}}'
```

## CI

Contract tests use **wiremock** (`infra-asaas/tests`) and in-process webhook tests (`api-http/tests/billing_webhook.rs`).  
Optional nightly sandbox job: set `ASAAS_API_KEY` secret and run `cargo test -p infra-asaas -- --ignored` when sandbox tests are added.

## Security

- API keys and card tokens are masked in logs (`infra-asaas::sanitize`).
- Webhook auth uses constant-time compare (`subtle`).
- Never commit real keys — see `backend/.env.example`.

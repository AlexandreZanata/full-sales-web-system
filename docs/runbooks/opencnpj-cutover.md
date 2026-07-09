# OpenCNPJ cutover runbook (Phase 70E)

Staging and production steps to switch `GET /v1/commerces/cnpj-lookup` upstream to OpenCNPJ.

---

## Prerequisites

- API key from https://admin.comerc.app.br (`ocnpj_live_<32 hex>`)
- `REDIS_URL` configured (negative cache for `CNPJ_NOT_FOUND`, TTL 24h)
- Full Sales API deployed with Phase 70 adapter code

---

## 1. Create and store API key

1. Log in to https://admin.comerc.app.br
2. Create key with lookup scope
3. Store in secret manager / `backend/.env` (never commit):

```bash
CNPJ_LOOKUP_PROVIDER=opencnpj
CNPJ_LOOKUP_URL=https://api.comerc.app.br
CNPJ_LOOKUP_API_KEY=ocnpj_live_<from admin>
```

---

## 2. Verify upstream health

```bash
curl -sS -o /dev/null -w "HTTP %{http_code}\n" https://api.comerc.app.br/readyz
# Expected: HTTP 200
```

Optional authenticated probe (local only):

```bash
./.local/phases/70-opencnpj-integration/_reference/probe-opencnpj.sh
```

---

## 3. Staging smoke

1. Set env vars on staging VPS
2. Restart API
3. Seller app → Register commerce → lookup CNPJ `00000000000191` → form prefilled
4. Unknown CNPJ → `CNPJ_NOT_FOUND` (manual entry still works)
5. Invalid check digits → `INVALID_CNPJ` (no upstream call)

---

## 4. Rate limits and quota

| Layer | Key / policy |
|-------|----------------|
| Full Sales | `cnpj-lookup:{tenant}:{user}` — 30 req/min |
| OpenCNPJ | 60 req/min per API key |
| Negative cache | `cnpj-lookup:miss:{cnpj}` — 24h TTL in Redis |

Monitor OpenCNPJ `429` (`rate_limit_exceeded`, `quota_exceeded`) in logs. Burst tests should return Full Sales `RATE_LIMITED` before upstream quota is exhausted.

---

## 5. Production flip

1. Apply same env as staging during low-traffic window
2. Restart API
3. PO sign-off after seller registration smoke

---

## Rollback

```bash
CNPJ_LOOKUP_PROVIDER=brasilapi
# restart API — no code deploy required
```

BrasilAPI adapter remains in the binary.

---

## Manual validation checklist

| Scenario | Expected | Automated contract test |
|----------|----------|-------------------------|
| Valid known CNPJ | 200 + prefill | `contract_cnpj_lookup_when_valid_known_cnpj_then_200` |
| Unknown CNPJ | `CNPJ_NOT_FOUND` | `contract_cnpj_lookup_when_unknown_cnpj_then_not_found` |
| Invalid check digits | `INVALID_CNPJ` | `contract_cnpj_lookup_when_invalid_check_digits_then_bad_request` |
| Upstream/auth failure | `CNPJ_LOOKUP_UNAVAILABLE` | `contract_cnpj_lookup_when_upstream_unavailable_then_502` |
| Burst lookups | `RATE_LIMITED` | `contract_cnpj_lookup_when_burst_then_rate_limited` |
| Repeat unknown (cache) | No extra upstream call | `contract_cnpj_lookup_when_cached_miss_then_skips_upstream` |

Live OpenCNPJ smoke on staging/production requires a real API key (70E ops).

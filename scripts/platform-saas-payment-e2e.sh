#!/usr/bin/env bash
# Phase 13C — manual payment E2E against Asaas sandbox.
# Requires: API on :8080, migrations, platform admin seeded, ASAAS_* env vars.
set -euo pipefail

API="${API_BASE:-http://127.0.0.1:8080/v1}"
WEBHOOK_TOKEN="${ASAAS_WEBHOOK_TOKEN:-test-webhook-token-phase3}"

echo "== Phase 13C: Platform billing payment E2E =="
echo "API: $API"

if [[ "${ASAAS_SANDBOX:-0}" != "1" ]]; then
  echo "Set ASAAS_SANDBOX=1 and configure ASAAS_API_KEY for live sandbox calls."
  echo "This script documents curl steps; mock path is covered by: cargo test -p api-http platform_saas"
fi

echo ""
echo "1. PlatformAdmin login + MFA (see DEV-COMMANDS.md platform@test.com)"
echo "2. POST $API/platform/tenants — provision tenant with trial"
echo "3. Complete payment in Asaas sandbox dashboard for the customer"
echo "4. POST $API/billing/webhooks/asaas — or wait for Asaas webhook delivery"
echo "   Header: asaas-access-token: \$ASAAS_WEBHOOK_TOKEN"
echo "5. GET $API/platform/tenants/{id} — expect status Active"
echo ""
echo "Past due → suspend:"
echo "6. Trigger PAYMENT_OVERDUE webhook for tenant externalReference"
echo "7. POST $API/platform/jobs/dunning — after grace period (or backdate in dev DB)"
echo "8. Verify tenant status Suspended; tenant sale POST returns TENANT_SUSPENDED"
echo ""
echo "Tenant Asaas (Pro+):"
echo "9. Tenant Admin PUT $API/settings/payments/asaas/connect with sandbox API key"
echo "10. Portal checkout — POST portal order submit with online payment enabled"
echo ""
echo "Webhook token in use: (masked) ${WEBHOOK_TOKEN:0:4}****"

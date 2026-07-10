#!/usr/bin/env bash
# Phase 13D — manual custom domain E2E.
# Requires: API on :8080, Pro+ tenant, DNS or mock verification job.
set -euo pipefail

API="${API_BASE:-http://127.0.0.1:8080/v1}"
HOSTNAME="${CUSTOM_DOMAIN:-shop.example.com}"

echo "== Phase 13D: Custom domain E2E =="
echo "API: $API"
echo "Hostname: $HOSTNAME"
echo ""
echo "1. Tenant Admin (Pro plan): POST $API/settings/domains"
echo "   Body: {\"hostname\": \"$HOSTNAME\"}"
echo "2. Add DNS TXT record from response txtValue (see docs/deployment/caddy-custom-domains.md)"
echo "3. PlatformAdmin: POST $API/platform/jobs/domain-verification"
echo "   Or force-verify in staging: POST $API/platform/domains/{id}/force-verify"
echo "4. GET $API/public/settings — Header: Host: $HOSTNAME"
echo "5. Configure Caddy reverse proxy for $HOSTNAME → portal upstream"
echo "6. Open https://$HOSTNAME in browser — portal loads with tenant branding"
echo ""
echo "Mock path (CI): cargo test -p api-http platform_saas::webhook_fraud_domain"

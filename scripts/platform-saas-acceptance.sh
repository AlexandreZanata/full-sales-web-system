#!/usr/bin/env bash
# Phase 13F — PO acceptance checklist runner (automated contract subset + manual prompts).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT/backend"

echo "== Phase 13 — PO acceptance (automated contract suite) =="
cargo test -p api-http platform_saas -- --test-threads=1

echo ""
echo "== Related contract modules =="
for suite in tenant_lifecycle billing_webhook billing_subscription \
  platform_auth audit_compliance health_monitoring \
  platform_fraud tenant_domains platform_operations; do
  cargo test -p api-http --test "$suite" -- --test-threads=1
done

echo ""
echo "== Manual PO checklist (record sign-off in .local/phases/13-integration-e2e/TASKS.md) =="
cat <<'EOF'
[ ] PlatformAdmin creates tenant with trial; first Admin receives access
[ ] Subscription payment in Asaas sandbox activates tenant
[ ] Failed payment → PastDue → Suspended after grace
[ ] PlatformAdmin suspends tenant; field apps blocked
[ ] PlatformAdmin lists and disables any user across tenants
[ ] Impersonation works with audit trail
[ ] Tenant Admin (Pro) connects Asaas; portal checkout works
[ ] Custom domain verified and serves portal
[ ] Health matrix shows dependency failures
[ ] Fraud event on velocity breach; PlatformAdmin reviews
[ ] Audit log captures PlatformAdmin mutations
[ ] LGPD export produces complete tenant ZIP

Run manual scripts:
  ./scripts/platform-saas-payment-e2e.sh
  ./scripts/platform-saas-domain-e2e.sh
EOF

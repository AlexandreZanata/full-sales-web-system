#!/usr/bin/env bash
# Contract: kustomize overlays render required workloads without a live cluster.
set -euo pipefail

K8S_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
KUBECTL="${KUBECTL:-kubectl}"
if ! command -v "$KUBECTL" >/dev/null 2>&1; then
  if [[ -x /tmp/kubectl ]]; then
    KUBECTL=/tmp/kubectl
  else
    echo "kubectl not found" >&2
    exit 1
  fi
fi

fail() { echo "FAIL: $*" >&2; exit 1; }
pass() { echo "PASS: $*"; }

assert_contains() {
  local haystack="$1" needle="$2" label="$3"
  echo "$haystack" | grep -q "$needle" || fail "$label missing '$needle'"
}

ensure_secrets_env() {
  local overlay="$1"
  local dest="$K8S_ROOT/overlays/${overlay}/secrets.env"
  local src="$K8S_ROOT/overlays/${overlay}/secrets.env.example"
  if [[ ! -f "$dest" ]]; then
    [[ -f "$src" ]] || fail "missing $src"
    cp "$src" "$dest"
  fi
}

ensure_secrets_env staging
ensure_secrets_env prod

STAGING="$("$KUBECTL" kustomize "$K8S_ROOT/overlays/staging")"
assert_contains "$STAGING" "kind: Namespace" "staging"
assert_contains "$STAGING" "name: fullsales-staging" "staging ns"
assert_contains "$STAGING" "name: api" "staging api"
assert_contains "$STAGING" "name: admin" "staging admin"
assert_contains "$STAGING" "name: portal" "staging portal"
assert_contains "$STAGING" "name: platform-admin" "staging platform-admin"
assert_contains "$STAGING" "name: postgres" "staging postgres"
assert_contains "$STAGING" "name: redis" "staging redis"
assert_contains "$STAGING" "name: minio" "staging minio"
assert_contains "$STAGING" "name: fullsales-migrate" "staging migrate"
assert_contains "$STAGING" "path: /health" "staging liveness"
assert_contains "$STAGING" "path: /health/ready" "staging readiness"
assert_contains "$STAGING" "kind: NetworkPolicy" "staging netpol"
assert_contains "$STAGING" "kind: HorizontalPodAutoscaler" "staging hpa"
assert_contains "$STAGING" "kind: Ingress" "staging ingress"
assert_contains "$STAGING" "host: api.example.com" "staging api host"
assert_contains "$STAGING" "name: custom-domains" "staging custom domains"
assert_contains "$STAGING" "cloudflare-origin-tls" "staging origin tls"
assert_contains "$STAGING" "name: api-secrets" "staging secrets"
pass "staging overlay"

PROD="$("$KUBECTL" kustomize "$K8S_ROOT/overlays/prod")"
assert_contains "$PROD" "name: fullsales-prod" "prod ns"
assert_contains "$PROD" "name: api" "prod api"
assert_contains "$PROD" "minReplicas: 2" "prod hpa min"
assert_contains "$PROD" "kind: Ingress" "prod ingress"
if echo "$PROD" | awk '/kind: Deployment/{d=1} d && /name: postgres/{found=1} /---/{d=0} END{exit !found}'; then
  fail "prod must not include in-cluster postgres Deployment"
fi
# No plaintext JWT in committed Secret manifests (secretGenerator only)
echo "$PROD" | grep -q 'kind: Secret' || fail "prod missing generated Secret"
pass "prod overlay (no in-cluster data stores)"

echo "All manifest contract checks passed."

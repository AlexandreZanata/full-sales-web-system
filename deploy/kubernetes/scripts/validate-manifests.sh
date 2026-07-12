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
pass "staging overlay"

PROD="$("$KUBECTL" kustomize "$K8S_ROOT/overlays/prod")"
assert_contains "$PROD" "name: fullsales-prod" "prod ns"
assert_contains "$PROD" "name: api" "prod api"
assert_contains "$PROD" "minReplicas: 2" "prod hpa min"
if echo "$PROD" | awk '/kind: Deployment/{d=1} d && /name: postgres/{found=1} /---/{d=0} END{exit !found}'; then
  fail "prod must not include in-cluster postgres Deployment"
fi
pass "prod overlay (no in-cluster data stores)"

echo "All manifest contract checks passed."

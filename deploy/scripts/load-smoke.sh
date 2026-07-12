#!/usr/bin/env bash
# Light concurrent smoke against local or port-forwarded endpoints (not a full perf suite).
set -euo pipefail

API_URL="${API_URL:-http://127.0.0.1:18080}"
PORTAL_URL="${PORTAL_URL:-http://127.0.0.1:18083}"
N="${N:-20}"

fail() { echo "FAIL: $*" >&2; exit 1; }
pass() { echo "PASS: $*"; }

curl -fsS "$API_URL/health" | grep -q '"status":"ok"' || fail "api health"
pass "api health"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

for i in $(seq 1 "$N"); do
  curl -fsS -o /dev/null "$API_URL/health" && echo ok >"$tmpdir/api-$i" &
done
wait
api_ok=$(find "$tmpdir" -name 'api-*' | wc -l)
test "$api_ok" -eq "$N" || fail "api concurrent health ($api_ok/$N)"
pass "api concurrent x$N"

if curl -fsS -o /dev/null "$PORTAL_URL/" 2>/dev/null; then
  for i in $(seq 1 "$N"); do
    curl -fsS -o /dev/null "$PORTAL_URL/" && echo ok >"$tmpdir/portal-$i" &
  done
  wait
  portal_ok=$(find "$tmpdir" -name 'portal-*' | wc -l)
  test "$portal_ok" -eq "$N" || fail "portal concurrent ($portal_ok/$N)"
  pass "portal concurrent x$N"
else
  echo "SKIP: portal not reachable at $PORTAL_URL"
fi

echo "Load smoke finished."

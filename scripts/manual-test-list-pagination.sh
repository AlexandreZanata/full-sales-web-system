#!/usr/bin/env bash
# Manual smoke test for Phase 68 cursor/offset list routes.
set -euo pipefail

BASE="${API_BASE:-http://127.0.0.1:8080/v1}"
PASS=0
FAIL=0
SKIP=0

red() { printf '\033[31m%s\033[0m\n' "$*"; }
green() { printf '\033[32m%s\033[0m\n' "$*"; }
yellow() { printf '\033[33m%s\033[0m\n' "$*"; }

login() {
  local email=$1 pass=$2
  curl -sf -X POST "$BASE/auth/login" \
    -H 'Content-Type: application/json' \
    -d "{\"email\":\"$email\",\"password\":\"$pass\"}" | jq -r '.accessToken'
}

assert_cursor_envelope() {
  local name=$1 body=$2
  local ok=1
  echo "$body" | jq -e '.data | type == "array"' >/dev/null 2>&1 || ok=0
  echo "$body" | jq -e '.pagination.limit | type == "number"' >/dev/null 2>&1 || ok=0
  echo "$body" | jq -e '.pagination.has_more | type == "boolean"' >/dev/null 2>&1 || ok=0
  if [[ $ok -eq 1 ]]; then
    green "  PASS $name — cursor envelope"
    local count limit has_more next
    count=$(echo "$body" | jq '.data | length')
    limit=$(echo "$body" | jq '.pagination.limit')
    has_more=$(echo "$body" | jq '.pagination.has_more')
    next=$(echo "$body" | jq -r '.pagination.next_cursor // "null"')
    echo "       data=$count limit=$limit has_more=$has_more next_cursor=$next"
    PASS=$((PASS + 1))
    return 0
  fi
  red "  FAIL $name — expected { data, pagination: { limit, has_more } }"
  echo "$body" | jq . 2>/dev/null || echo "$body"
  FAIL=$((FAIL + 1))
  return 1
}

assert_offset_envelope() {
  local name=$1 body=$2
  local ok=1
  echo "$body" | jq -e '.items | type == "array"' >/dev/null 2>&1 || ok=0
  echo "$body" | jq -e '.page | type == "number"' >/dev/null 2>&1 || ok=0
  echo "$body" | jq -e '.pageSize | type == "number"' >/dev/null 2>&1 || ok=0
  echo "$body" | jq -e '.total | type == "number"' >/dev/null 2>&1 || ok=0
  if [[ $ok -eq 1 ]]; then
    green "  PASS $name — offset envelope"
    PASS=$((PASS + 1))
    return 0
  fi
  red "  FAIL $name — expected { items, page, pageSize, total }"
  echo "$body" | jq . 2>/dev/null || echo "$body"
  FAIL=$((FAIL + 1))
  return 1
}

assert_invalid_filter() {
  local name=$1 status=$2 body=$3
  local code
  code=$(echo "$body" | jq -r '.error.code // empty')
  if [[ "$status" == "400" && "$code" == "invalid_filter_field" ]]; then
    green "  PASS $name — invalid_filter_field"
    PASS=$((PASS + 1))
  else
    red "  FAIL $name — expected 400 invalid_filter_field got status=$status code=$code"
    FAIL=$((FAIL + 1))
  fi
}

get() {
  local token=${1:-}; shift
  local path=$1
  if [[ -n $token ]]; then
    curl -s -w '\n%{http_code}' -H "Authorization: Bearer $token" "$BASE$path"
  else
    curl -s -w '\n%{http_code}' "$BASE$path"
  fi
}

split_status() {
  local raw=$1
  BODY=$(echo "$raw" | sed '$d')
  STATUS=$(echo "$raw" | tail -n1)
}

echo "=== Phase 68 list pagination manual smoke ==="
echo "API: $BASE"
echo

if ! curl -sf "$BASE/../health" >/dev/null 2>&1 && ! curl -sf "${BASE%/v1}/health" >/dev/null 2>&1; then
  red "API not reachable at $BASE"
  exit 1
fi
green "API health OK"

ADMIN_TOKEN=$(login admin@test.com secret123) || { red "Admin login failed — run pnpm seed:dev"; exit 1; }
DRIVER_TOKEN=$(login driver-a@test.com secret123) || true
SELLER_TOKEN=$(login seller@test.com secret123) || true
PORTAL_TOKEN=$(login portal@seed-store.com secret123) || true

COMMERCE_ID="01900001-0010-7000-8000-000000000001"
PRODUCT_ID="01900001-0020-7000-8000-000000000001"

yellow "--- 68C: users, commerces, addresses ---"
split_status "$(get "$ADMIN_TOKEN" "/users?limit=5")"
[[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /users" "$BODY" || { red "FAIL GET /users status=$STATUS"; FAIL=$((FAIL+1)); }
split_status "$(get "$ADMIN_TOKEN" "/users?filter[unknown]=x")"
assert_invalid_filter "GET /users invalid filter" "$STATUS" "$BODY"

split_status "$(get "$ADMIN_TOKEN" "/commerces?limit=5")"
[[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /commerces" "$BODY"
split_status "$(get "$ADMIN_TOKEN" "/commerces?limit=5&filter[active]=true")"
[[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /commerces?filter[active]=true" "$BODY"

split_status "$(get "$ADMIN_TOKEN" "/commerces/$COMMERCE_ID/addresses?limit=10")"
[[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /commerces/{id}/addresses" "$BODY"

yellow "--- 68B: catalog + inventory ---"
split_status "$(get "$ADMIN_TOKEN" "/products?limit=5")"
[[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /products" "$BODY"
split_status "$(get "$ADMIN_TOKEN" "/products?limit=5&filter[active]=true")"
[[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /products?filter[active]" "$BODY"

split_status "$(get "$ADMIN_TOKEN" "/products/top-selling?limit=5")"
[[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /products/top-selling" "$BODY"

split_status "$(get "$ADMIN_TOKEN" "/categories?limit=10")"
[[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /categories" "$BODY"

split_status "$(get "$ADMIN_TOKEN" "/products/$PRODUCT_ID/images?limit=10")"
[[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /products/{id}/images" "$BODY"

split_status "$(get "$ADMIN_TOKEN" "/inventory/balances?limit=5")"
[[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /inventory/balances" "$BODY"
split_status "$(get "$ADMIN_TOKEN" "/inventory/balances?limit=5&filter[name][like]=a")"
[[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /inventory/balances?filter[name][like]" "$BODY"

split_status "$(get "$ADMIN_TOKEN" "/inventory/products/$PRODUCT_ID/movements?limit=5")"
[[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /inventory/products/{id}/movements" "$BODY"

yellow "--- 68D: sales, orders, deliveries, portal orders ---"
split_status "$(get "$ADMIN_TOKEN" "/sales?limit=5")"
[[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /sales (admin)" "$BODY"
if [[ -n ${DRIVER_TOKEN:-} ]]; then
  split_status "$(get "$DRIVER_TOKEN" "/sales?limit=5")"
  [[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /sales (driver)" "$BODY"
fi
split_status "$(get "$ADMIN_TOKEN" "/sales?filter[unknown]=x")"
assert_invalid_filter "GET /sales invalid filter" "$STATUS" "$BODY"

split_status "$(get "$ADMIN_TOKEN" "/orders?limit=5")"
[[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /orders" "$BODY"
split_status "$(get "$ADMIN_TOKEN" "/orders?limit=5&filter[status]=PendingApproval")"
[[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /orders?filter[status]" "$BODY"

split_status "$(get "$ADMIN_TOKEN" "/deliveries?limit=5")"
[[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /deliveries" "$BODY"

if [[ -n ${PORTAL_TOKEN:-} ]]; then
  split_status "$(get "$PORTAL_TOKEN" "/portal/orders?limit=5")"
  [[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /portal/orders" "$BODY"
else
  yellow "  SKIP portal/orders — no portal token"
  SKIP=$((SKIP+1))
fi

yellow "--- 68E: portal + public catalog ---"
split_status "$(get "" "/public/products?limit=5")"
[[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /public/products" "$BODY"
split_status "$(get "" "/public/categories?limit=10")"
[[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /public/categories" "$BODY"

if [[ -n ${PORTAL_TOKEN:-} ]]; then
  split_status "$(get "$PORTAL_TOKEN" "/portal/products?limit=5")"
  [[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /portal/products" "$BODY"
  split_status "$(get "$PORTAL_TOKEN" "/portal/categories?limit=10")"
  [[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /portal/categories" "$BODY"
fi

yellow "--- 68F: reports (offset) + audit (cursor) ---"
split_status "$(get "$ADMIN_TOKEN" "/reports?page=1&pageSize=10")"
[[ "$STATUS" == "200" ]] && assert_offset_envelope "GET /reports" "$BODY"

split_status "$(get "$ADMIN_TOKEN" "/audit/events?limit=10")"
[[ "$STATUS" == "200" ]] && assert_cursor_envelope "GET /audit/events" "$BODY"
split_status "$(get "$ADMIN_TOKEN" "/audit/events?filter[unknown]=x")"
assert_invalid_filter "GET /audit/events invalid filter" "$STATUS" "$BODY"

yellow "--- cursor page 2 (when has_more) ---"
split_status "$(get "$ADMIN_TOKEN" "/products?limit=2")"
if [[ "$STATUS" == "200" ]]; then
  next=$(echo "$BODY" | jq -r '.pagination.next_cursor // empty')
  if [[ -n $next && $next != "null" ]]; then
    split_status "$(get "$ADMIN_TOKEN" "/products?limit=2&cursor=$next")"
    if [[ "$STATUS" == "200" ]]; then
      assert_cursor_envelope "GET /products page 2 (cursor)" "$BODY"
    else
      red "  FAIL cursor page 2 status=$STATUS"; FAIL=$((FAIL+1))
    fi
  else
    yellow "  SKIP cursor page 2 — not enough products for has_more"
    SKIP=$((SKIP+1))
  fi
fi

echo
echo "=== Summary: PASS=$PASS FAIL=$FAIL SKIP=$SKIP ==="
[[ $FAIL -eq 0 ]] && exit 0 || exit 1

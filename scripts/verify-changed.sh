#!/usr/bin/env bash
# Verify modified code before commit (staged) or push (range vs upstream).
#
# commit  → fast gate only (secrets, fmt, lint/test touched packages, cargo check)
# push    → full gate (monorepo lint/test, clippy, cargo test, E2E, mobile)
# GitHub CI runs the full suite on push (see .github/workflows/ci.yml).
#
# Usage: verify-changed.sh commit | push
# Skip:  SKIP_VERIFY=1 git commit|push ...
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT}"

MODE="${1:-commit}"
case "$MODE" in
  commit|push) ;;
  *)
    echo "Usage: $0 commit|push" >&2
    exit 2
    ;;
esac

list_changed() {
  if [[ "$MODE" == "commit" ]]; then
    git diff --cached --name-only --diff-filter=ACMR
    return
  fi
  local remote base
  remote="$(git rev-parse --abbrev-ref --symbolic-full-name @{u} 2>/dev/null || true)"
  if [[ -n "$remote" ]]; then
    base="$(git merge-base HEAD "$remote")"
    git diff --name-only --diff-filter=ACMR "${base}..HEAD"
  else
    git diff --name-only --diff-filter=ACMR HEAD
    git ls-files --others --exclude-standard
  fi
}

mapfile -t CHANGED < <(list_changed | sort -u)
if [[ ${#CHANGED[@]} -eq 0 || -z "${CHANGED[0]:-}" ]]; then
  echo "verify-changed ($MODE): no changed files — skip"
  exit 0
fi

echo "verify-changed ($MODE): ${#CHANGED[@]} file(s)"

has_prefix() {
  local p="$1" f
  for f in "${CHANGED[@]}"; do
    [[ "$f" == "$p"* ]] && return 0
  done
  return 1
}

has_glob() {
  local f
  for f in "${CHANGED[@]}"; do
    case "$f" in
      $1) return 0 ;;
    esac
  done
  return 1
}

# Unique pnpm workspace package dirs touched under apps/ or packages/.
list_touched_node_packages() {
  local f pkg
  declare -A seen=()
  for f in "${CHANGED[@]}"; do
    case "$f" in
      apps/*/*|packages/*/*)
        pkg="$(echo "$f" | cut -d/ -f1-2)"
        if [[ -f "${pkg}/package.json" && -z "${seen[$pkg]:-}" ]]; then
          seen[$pkg]=1
          echo "$pkg"
        fi
        ;;
    esac
  done
}

fail=0
run() {
  echo "+ $*"
  if ! "$@"; then
    fail=1
  fi
}

# --- always: secret hygiene (fast) ---
run bash scripts/verify-no-production-secrets.sh
if has_prefix "apps-mobile/" || has_glob "*.keystore" || has_glob "*google-services*"; then
  run bash scripts/verify-no-android-secrets.sh
fi

NODE_ROOT_TOUCHED=0
if has_glob "package.json" || has_glob "pnpm-lock.yaml" || has_glob "eslint*" || has_glob "tsconfig*" || has_glob "pnpm-workspace.yaml"; then
  NODE_ROOT_TOUCHED=1
fi
NODE_PKG_TOUCHED=0
if has_prefix "apps/" || has_prefix "packages/"; then
  NODE_PKG_TOUCHED=1
fi

# --- node ---
if [[ "$MODE" == "commit" ]]; then
  # Fast: only lint+test packages that appear in the staged set.
  if [[ "$NODE_PKG_TOUCHED" -eq 1 ]]; then
    mapfile -t PKGS < <(list_touched_node_packages)
    if [[ ${#PKGS[@]} -gt 0 ]]; then
      FILTER_ARGS=()
      for pkg in "${PKGS[@]}"; do
        FILTER_ARGS+=(--filter "./${pkg}")
      done
      run pnpm "${FILTER_ARGS[@]}" lint
      run pnpm "${FILTER_ARGS[@]}" test
    fi
  fi
  # Root tooling churn: lint only (full monorepo test waits for push/CI).
  if [[ "$NODE_ROOT_TOUCHED" -eq 1 && "$NODE_PKG_TOUCHED" -eq 0 ]]; then
    run pnpm lint
  fi
else
  # Push: full monorepo lint + unit tests when any JS/TS surface changed.
  if [[ "$NODE_PKG_TOUCHED" -eq 1 || "$NODE_ROOT_TOUCHED" -eq 1 ]]; then
    run pnpm lint
    run pnpm test
  fi
fi

# --- E2E: push only (Playwright is slow; GitHub CI also covers this) ---
if [[ "$MODE" == "push" ]]; then
  NEED_PW=0
  RUN_ADMIN=0
  RUN_PORTAL=0
  RUN_FIELD=0
  if has_prefix "e2e/" || has_glob "playwright*.config.ts" || has_prefix "apps/admin/" || has_prefix "apps/portal/" || has_prefix "apps/field/"; then
    NEED_PW=1
  fi
  if has_prefix "e2e/admin" || has_prefix "apps/admin/" || has_glob "playwright.config.ts" || has_prefix "e2e/fixtures/"; then
    RUN_ADMIN=1
  fi
  if has_prefix "e2e/portal" || has_prefix "apps/portal/" || has_glob "playwright.portal.config.ts" || has_prefix "e2e/fixtures/"; then
    RUN_PORTAL=1
  fi
  if has_prefix "e2e/field" || has_prefix "apps/field/" || has_glob "playwright.field.config.ts" || has_prefix "e2e/fixtures/"; then
    RUN_FIELD=1
  fi
  if [[ "$NEED_PW" -eq 1 ]]; then
    run pnpm exec playwright install chromium
  fi
  [[ "$RUN_ADMIN" -eq 1 ]] && run pnpm test:e2e:admin
  [[ "$RUN_PORTAL" -eq 1 ]] && run pnpm test:e2e:portal
  [[ "$RUN_FIELD" -eq 1 ]] && run pnpm test:e2e:field
fi

# --- backend ---
if has_prefix "backend/"; then
  echo "+ (backend) cargo fmt --check"
  if ! (cd backend && cargo fmt --check); then fail=1; fi
  if [[ "$MODE" == "push" ]]; then
    echo "+ (backend) cargo clippy --workspace -- -D warnings"
    if ! (cd backend && cargo clippy --workspace -- -D warnings); then fail=1; fi
    echo "+ (backend) cargo test --workspace"
    if ! (cd backend && cargo test --workspace); then fail=1; fi
  else
    # Commit: typecheck only — full clippy/tests on push + GitHub CI.
    echo "+ (backend) cargo check --workspace"
    if ! (cd backend && cargo check --workspace); then fail=1; fi
  fi
fi

# Contract scripts: push always when touched; commit only when inventory sources change.
if has_prefix "backend/" || has_prefix "docs/openapi" || has_glob "scripts/verify-api-route*" || has_glob "scripts/verify-route-contract*" || has_glob "scripts/api_route_inventory*" || has_glob "scripts/api-route-inventory*"; then
  if [[ "$MODE" == "push" ]] || has_glob "scripts/verify-api-route*" || has_glob "scripts/verify-route-contract*" || has_glob "docs/API-CONTRACT.md" || has_prefix "docs/openapi"; then
    run pnpm verify:api-route-inventory
    run pnpm verify:route-contract-manifest
  fi
fi
if has_glob "docs/openapi.yaml" || has_glob "docs/openapi.yml"; then
  run pnpm validate:openapi
fi

if has_prefix "infra/" || has_prefix "production/" || has_prefix "deploy/"; then
  run pnpm validate:vps-infra
fi

# Mobile Gradle: push only.
if [[ "$MODE" == "push" ]] && has_prefix "apps-mobile/seller/"; then
  run pnpm mobile:seller:check
fi

if [[ "$fail" -ne 0 ]]; then
  echo "verify-changed ($MODE): FAILED" >&2
  exit 1
fi
echo "verify-changed ($MODE): OK"

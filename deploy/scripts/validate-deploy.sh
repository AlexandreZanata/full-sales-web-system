#!/usr/bin/env bash
# Deploy contract checks (no live cluster required).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
KUBECTL="${KUBECTL:-kubectl}"
export KUBECTL
"$ROOT/deploy/kubernetes/scripts/validate-manifests.sh"

MIG="$ROOT/backend/target/debug/fullsales-migrate"
if [[ ! -x "$MIG" ]]; then
  (cd "$ROOT/backend" && cargo build -p infra-postgres --bin fullsales-migrate)
fi
set +e
"$MIG" >/dev/null 2>"$ROOT/deploy/.migrate-err"
ec=$?
set -e
test "$ec" = "2"
grep -q DATABASE_ADMIN_URL "$ROOT/deploy/.migrate-err"
rm -f "$ROOT/deploy/.migrate-err"
echo "PASS: fullsales-migrate missing-env contract"
echo "All deploy contract checks passed."

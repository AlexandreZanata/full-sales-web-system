#!/usr/bin/env bash
# Phase 0 documentation acceptance checks — run from repo root.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

failures=0

check() {
  if "$@"; then
    echo "  OK: $*"
  else
    echo "  FAIL: $*"
    failures=$((failures + 1))
  fi
}

echo "=== Phase 0 doc validation ==="

echo "--- Glossary: no placeholders ---"
if grep -n '_(' docs/GLOSSARY.md; then
  echo "  FAIL: placeholder text in GLOSSARY.md"
  failures=$((failures + 1))
else
  echo "  OK: no _(placeholder)_ in GLOSSARY.md"
fi

echo "--- Open decisions resolved ---"
check test ! -s <(grep -E '^\| OD-' docs/OPEN-DECISIONS.md || true)

echo "--- ADRs present (001..007) ---"
for n in 001 002 003 004 005 006 007; do
  check test -f "docs/adr/ADR-${n}"*.md
done

echo "--- Use cases exist ---"
check test -f docs/use-cases/UC-001-register-and-confirm-sale.md
check test -f docs/use-cases/UC-002-generate-signed-report.md

echo "--- Business rules count ---"
br_count=$(grep -c '^### BR-' docs/BUSINESS-RULES.md || true)
if [ "$br_count" -ge 3 ]; then
  echo "  OK: $br_count business rules found"
else
  echo "  FAIL: expected >= 3 business rules, found $br_count"
  failures=$((failures + 1))
fi

echo "--- UC-001 glossary terms ---"
for term in Sale SaleItem Commerce Product PaymentMethod StockMovement; do
  check grep -q "## ${term}" docs/GLOSSARY.md
done

echo "--- Harness resolve ---"
check ./agent-harness/resolve-rules.sh api order contract >/dev/null

echo "--- Sign-off row filled ---"
check grep -q 'Product Owner' docs/NEW-PROJECT-CHECKLIST.md

if [ "$failures" -eq 0 ]; then
  echo "=== All Phase 0 checks passed ==="
  exit 0
else
  echo "=== $failures check(s) failed ==="
  exit 1
fi

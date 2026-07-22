#!/usr/bin/env bash
# Point this clone at repo-managed hooks (.githooks/). Safe to re-run.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT}"
chmod +x scripts/verify-changed.sh .githooks/pre-commit .githooks/pre-push
git config core.hooksPath .githooks
echo "Git hooks installed: core.hooksPath=.githooks (pre-commit, pre-push)"
echo "Emergency skip: SKIP_VERIFY=1 git commit|push ..."

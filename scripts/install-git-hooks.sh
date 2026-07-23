#!/usr/bin/env bash
# Point this clone at repo-managed hooks (.githooks/). Safe to re-run.
# No-op in Docker/CI partial trees (no .git / .githooks).
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT}"

if [[ ! -d .git ]] || [[ ! -d .githooks ]]; then
  echo "install-git-hooks: skip (no .git or .githooks)"
  exit 0
fi

chmod +x scripts/verify-changed.sh .githooks/pre-commit .githooks/pre-push
git config core.hooksPath .githooks
echo "Git hooks installed: core.hooksPath=.githooks (pre-commit, pre-push)"
echo "Emergency skip: SKIP_VERIFY=1 git commit|push ..."

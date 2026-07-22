#!/usr/bin/env bash
# Ensure Node 22 + pnpm for SPA builds on VPS.
set -euo pipefail

if command -v node >/dev/null 2>&1; then
  major="$(node -v | sed 's/^v//' | cut -d. -f1)"
  if [[ "${major}" -ge 22 ]]; then
    corepack enable >/dev/null 2>&1 || true
    corepack prepare pnpm@9.15.9 --activate >/dev/null 2>&1 || true
    echo "ensure-node.sh: node $(node -v) pnpm $(pnpm -v 2>/dev/null || echo missing)"
    exit 0
  fi
fi

echo "ensure-node.sh: installing Node 22 via NodeSource" >&2
curl -fsSL https://deb.nodesource.com/setup_22.x | bash -
apt-get install -y nodejs
corepack enable
corepack prepare pnpm@9.15.9 --activate
echo "ensure-node.sh: node $(node -v) pnpm $(pnpm -v)"

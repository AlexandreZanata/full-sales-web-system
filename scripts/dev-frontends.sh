#!/usr/bin/env bash
# Start all web frontends (Vite HMR) — no Docker. API/Postgres/Redis must already be running.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

API_ORIGIN="${VITE_DEV_API_ORIGIN:-http://127.0.0.1:8080}"

bold() { printf '\033[1m%s\033[0m\n' "$*"; }
warn() { printf '\033[33m%s\033[0m\n' "$*"; }

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "error: '$1' not found in PATH" >&2
    exit 1
  fi
}

require_cmd pnpm

if [[ ! -d node_modules ]]; then
  echo "error: node_modules missing — run: pnpm install" >&2
  exit 1
fi

if ! curl -sf --connect-timeout 2 "${API_ORIGIN}/health" >/dev/null 2>&1; then
  warn "warning: API not reachable at ${API_ORIGIN}/health"
  warn "  Frontends proxy /v1 to the API. Start it separately (no Docker):"
  warn "    pnpm dev:api"
  warn "  Ensure Postgres + Redis match backend/.env (see backend/.env.example)."
  echo
fi

bold "Full Sales — frontend dev servers (Vite HMR, no Docker)"
echo "  Web    → http://localhost:5173"
echo "  Admin  → http://127.0.0.1:5174/login"
echo "  Portal → http://127.0.0.1:5175/login"
echo "  Field  → http://127.0.0.1:5176/login"
echo "  API    → ${API_ORIGIN} (proxy target for admin/portal/field)"
echo
echo "  Dev logins (after pnpm seed:dev): admin@test.com / secret123"
echo "  Press Ctrl+C to stop all frontends."
echo

exec pnpm dev:frontends

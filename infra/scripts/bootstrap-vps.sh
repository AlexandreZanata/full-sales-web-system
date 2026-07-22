#!/usr/bin/env bash
# Idempotent VPS packages for Full Sales deploy (does not alter other sites).
set -euo pipefail

need_cmd() {
  command -v "$1" >/dev/null 2>&1
}

if ! need_cmd docker; then
  echo "bootstrap-vps.sh: install Docker Engine first" >&2
  exit 1
fi
if ! need_cmd nginx; then
  echo "bootstrap-vps.sh: install nginx first" >&2
  exit 1
fi

need_cmd certbot || echo "bootstrap-vps.sh: warning — certbot missing (install for TLS)"
need_cmd node || echo "bootstrap-vps.sh: warning — Node 22+ needed for SPA builds"
need_cmd pnpm || echo "bootstrap-vps.sh: warning — pnpm needed (corepack enable)"

docker compose version >/dev/null
nginx -t
echo "bootstrap-vps.sh: prerequisites OK"

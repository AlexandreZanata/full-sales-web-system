#!/usr/bin/env bash
# Read-only nginx inventory — safe on shared VPS.
set -euo pipefail

echo "=== sites-enabled ==="
ls -la /etc/nginx/sites-enabled/ 2>/dev/null || true
echo "=== server_name / default_server ==="
grep -REn "server_name|default_server|listen " /etc/nginx/sites-enabled/ 2>/dev/null | head -80 || true
echo "=== docker published ==="
docker ps --format 'table {{.Names}}\t{{.Ports}}' 2>/dev/null | head -40 || true
echo "diagnose-nginx-vhosts.sh: done"

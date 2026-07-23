#!/usr/bin/env bash
# Install SPA Vite env files for production builds.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SRC="${ROOT}/production/env"

install_file() {
  local from="$1" to="$2"
  [[ -f "${from}" ]] || { echo "install-env.sh: missing ${from}" >&2; exit 1; }
  install -m 600 "${from}" "${to}"
  echo "install-env.sh: ${to}"
}

install_file "${SRC}/portal.env" "${ROOT}/apps/portal/.env.production"
install_file "${SRC}/admin.env" "${ROOT}/apps/admin/.env.production"
install_file "${SRC}/platform-admin.env" "${ROOT}/apps/platform-admin/.env.production"
install_file "${SRC}/field.env" "${ROOT}/apps/field/.env.production"
echo "install-env.sh: done"

#!/usr/bin/env bash
# Shared SSH/rsync — ControlMaster reuses one auth for deploy.
# shellcheck disable=SC2034
vps_ssh_begin() {
  local root="${1:?project root}"
  VPS_SSH_KEY="${VPS_SSH_KEY:-${root}/production/ssh/id_ed25519_fullsales}"
  if [[ ! -f "${VPS_SSH_KEY}" ]]; then
    VPS_SSH_KEY="${root}/production/ssh/id_ed25519_print3d"
  fi
  VPS_SSH_CONTROL_DIR="${TMPDIR:-/tmp}/fullsales-ssh-${$}"
  mkdir -p "${VPS_SSH_CONTROL_DIR}"
  VPS_SSH_CONTROL_PATH="${VPS_SSH_CONTROL_DIR}/cm-%r@%h:%p"

  VPS_SSH_OPTS=(
    -p "${VPS_PORT:-22}"
    -o StrictHostKeyChecking=accept-new
    -o ControlMaster=auto
    -o "ControlPath=${VPS_SSH_CONTROL_PATH}"
    -o ControlPersist=600
  )
  if [[ -f "${VPS_SSH_KEY}" ]]; then
    VPS_SSH_OPTS+=(-i "${VPS_SSH_KEY}")
  fi

  VPS_RSYNC_SSH="ssh ${VPS_SSH_OPTS[*]}"
  VPS_REMOTE="${VPS_USER}@${VPS_HOST}"

  vps_ssh_end() {
    ssh "${VPS_SSH_OPTS[@]}" -O exit "${VPS_REMOTE}" 2>/dev/null || true
    rm -rf "${VPS_SSH_CONTROL_DIR}" 2>/dev/null || true
  }
  trap vps_ssh_end EXIT INT TERM
}

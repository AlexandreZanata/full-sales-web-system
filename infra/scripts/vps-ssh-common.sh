#!/usr/bin/env bash
# SSH/rsync helpers — one password prompt, ControlMaster reuse (sorrimobi pattern).
# shellcheck disable=SC2034
# Usage (after sourcing production/vps.env):
#   source infra/scripts/vps-ssh-common.sh
#   vps_ssh_begin "${ROOT}"
#   vps_run_ssh "${VPS_REMOTE}" "mkdir -p …"
#   rsync -e "${VPS_RSYNC_SSH}" …

vps_ssh_begin() {
  local root="${1:?project root}"
  VPS_SSH_CONTROL_DIR="$(mktemp -d "${TMPDIR:-/tmp}/fullsales-ssh.XXXXXX")"
  VPS_SSH_CONTROL_PATH="${VPS_SSH_CONTROL_DIR}/cm-%r@%h:%p"
  VPS_ASKPASS_SCRIPT=""
  VPS_PASS_FILE=""
  VPS_USE_SSHPASS=0

  VPS_SSH_OPTS=(
    -p "${VPS_PORT:-22}"
    -o StrictHostKeyChecking=accept-new
    -o ConnectTimeout=15
    -o ServerAliveInterval=15
    -o ServerAliveCountMax=3
    -o ControlMaster=auto
    -o "ControlPath=${VPS_SSH_CONTROL_PATH}"
    -o ControlPersist=600
  )

  local key="${VPS_SSH_KEY:-${root}/production/ssh/id_ed25519_fullsales}"
  if [[ ! -f "${key}" ]]; then
    key="${root}/production/ssh/id_ed25519_print3d"
  fi

  # Password mode when forced, or when no key file (manual one-shot deploy).
  if [[ "${VPS_USE_PASSWORD:-1}" == "1" ]] || [[ ! -f "${key}" ]]; then
    if [[ -z "${VPS_SSH_PASSWORD:-}" ]]; then
      read -r -s -p "SSH password for ${VPS_USER}@${VPS_HOST}: " VPS_SSH_PASSWORD
      echo
    fi
    if [[ -z "${VPS_SSH_PASSWORD:-}" ]]; then
      echo "vps-ssh-common: empty password" >&2
      exit 1
    fi
    export VPS_SSH_PASSWORD
    VPS_SSH_OPTS+=(
      -o PubkeyAuthentication=no
      -o PreferredAuthentications=password
      -o NumberOfPasswordPrompts=1
    )
    if command -v sshpass >/dev/null 2>&1; then
      VPS_USE_SSHPASS=1
      export SSHPASS="${VPS_SSH_PASSWORD}"
    else
      VPS_PASS_FILE="$(mktemp)"
      VPS_ASKPASS_SCRIPT="$(mktemp)"
      printf '%s' "${VPS_SSH_PASSWORD}" >"${VPS_PASS_FILE}"
      chmod 600 "${VPS_PASS_FILE}"
      printf '#!/bin/sh\ncat "%s"\n' "${VPS_PASS_FILE}" >"${VPS_ASKPASS_SCRIPT}"
      chmod 700 "${VPS_ASKPASS_SCRIPT}"
      export DISPLAY="${DISPLAY:-:0}"
      export SSH_ASKPASS="${VPS_ASKPASS_SCRIPT}"
      export SSH_ASKPASS_REQUIRE=force
    fi
  elif [[ -f "${key}" ]]; then
    VPS_SSH_OPTS+=(-i "${key}" -o IdentitiesOnly=yes)
  fi

  if [[ "${VPS_USE_SSHPASS}" == "1" ]]; then
    VPS_RSYNC_SSH="sshpass -e ssh ${VPS_SSH_OPTS[*]}"
  elif [[ -n "${VPS_SSH_PASSWORD:-}" ]]; then
    VPS_RSYNC_SSH="setsid -w ssh ${VPS_SSH_OPTS[*]}"
  else
    VPS_RSYNC_SSH="ssh ${VPS_SSH_OPTS[*]}"
  fi

  VPS_REMOTE="${VPS_USER}@${VPS_HOST}"

  vps_run_ssh() {
    if [[ "${VPS_USE_SSHPASS}" == "1" ]]; then
      sshpass -e ssh "${VPS_SSH_OPTS[@]}" "$@"
    elif [[ -n "${VPS_SSH_PASSWORD:-}" ]]; then
      setsid -w ssh "${VPS_SSH_OPTS[@]}" "$@"
    else
      ssh "${VPS_SSH_OPTS[@]}" "$@"
    fi
  }

  vps_ssh_end() {
    ssh -O exit -o "ControlPath=${VPS_SSH_CONTROL_PATH}" -p "${VPS_PORT:-22}" \
      "${VPS_REMOTE}" 2>/dev/null || true
    [[ -n "${VPS_ASKPASS_SCRIPT}" ]] && rm -f "${VPS_ASKPASS_SCRIPT}"
    [[ -n "${VPS_PASS_FILE}" ]] && rm -f "${VPS_PASS_FILE}"
    rm -rf "${VPS_SSH_CONTROL_DIR}" 2>/dev/null || true
    unset VPS_SSH_PASSWORD SSHPASS SSH_ASKPASS SSH_ASKPASS_REQUIRE 2>/dev/null || true
  }
  trap vps_ssh_end EXIT INT TERM
}

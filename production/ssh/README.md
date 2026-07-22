# Production SSH keys (gitignored private keys)

## Setup

```bash
ssh-keygen -t ed25519 -f production/ssh/id_ed25519_fullsales -C "fullsales-deploy" -N ""
# Install pubkey on VPS:
ssh-copy-id -i production/ssh/id_ed25519_fullsales.pub root@YOUR_VPS_IP
```

Or reuse an existing deploy key already authorized on the VPS (set `VPS_SSH_KEY` path in `vps-ssh-common.sh` / env).

Never commit `id_*` private keys.

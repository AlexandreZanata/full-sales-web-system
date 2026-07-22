# Shared VPS host deploy (Phase 20)

> Complement to [kubernetes.md](kubernetes.md) / Phase 15. **Go-live path:** Docker Compose data + API on the Hostinger VPS, host nginx, Cloudflare in front.

## Targets

| Surface | Hostname |
|---------|----------|
| Portal | `https://vendas.comerc.app.br` |
| Admin | `https://admin.vendas.comerc.app.br` |
| API | `https://api.vendas.comerc.app.br` |
| Platform admin | `https://platform.vendas.comerc.app.br` |

App directory on VPS: `/var/www/fullsales`. Origin IP: `YOUR_VPS_IP` (shared with other sites — do not remove their nginx vhosts).

## Data residency (v1)

All durable state stays **on the VPS**:

| Store | How |
|-------|-----|
| PostgreSQL | Compose volume `postgres_prod_data` (host port `5435` → container `5432`, localhost only) |
| Redis | Compose (host port `6381`) |
| MinIO (media) | Compose volume + host ports `9010`/`9011` |

No managed cloud database for v1. Phase 15 K8s manifests remain for a later cluster cutover.

## Operator entrypoints

| Action | Command |
|--------|---------|
| Manual deploy | `./production/deploy-to-vps.sh` |
| Env only | `./production/deploy-to-vps.sh --env-only` |
| Local infra contract | `./infra/scripts/validate-infra.sh` |

Secrets live under `production/` (gitignored). Templates: `production/*.example`, `production/env/*.example`.

## CI secrets (auto-deploy)

| GitHub secret | Purpose |
|---------------|---------|
| `VPS_HOST` | `YOUR_VPS_IP` |
| `VPS_USER` | `root` |
| `VPS_SSH_KEY` | Deploy private key |

Workflow: `.github/workflows/vps-deploy.yml` — after CI success on `main`, rsync + remote `vps-full-deploy.sh`, then smoke `GET /health`.

## Related

- [cloudflare.md](cloudflare.md)
- [../runbooks/deploy-vps.md](../runbooks/deploy-vps.md)
- [../runbooks/rollback-vps.md](../runbooks/rollback-vps.md)

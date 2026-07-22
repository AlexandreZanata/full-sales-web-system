# Production deploy (local secrets + VPS)

## Live targets

| Surface | URL |
|---------|-----|
| Portal | https://vendas.comerc.app.br |
| Admin | https://admin.vendas.comerc.app.br |
| API | https://api.vendas.comerc.app.br |
| Platform | https://platform.vendas.comerc.app.br |

Data (Postgres / Redis / MinIO) stays **on the VPS** Docker volumes.

## Quick start

```bash
cp production/vps.env.domain.example production/vps.env
# optional: create production/ssh/id_ed25519_fullsales (see ssh/README.md)
chmod +x production/deploy-to-vps.sh infra/scripts/*.sh
./production/deploy-to-vps.sh --env-only   # generate secrets + URLs
./production/deploy-to-vps.sh             # rsync + remote build
```

On VPS after first domain deploy (TLS):

```bash
ssh root@YOUR_VPS_IP
cd /var/www/fullsales
certbot --nginx \
  -d vendas.comerc.app.br \
  -d admin.vendas.comerc.app.br \
  -d api.vendas.comerc.app.br \
  -d platform.vendas.comerc.app.br
./infra/scripts/install-nginx-domain.sh
```

Cloudflare: SSL **Full (strict)** after certs exist.

## Layout

| Path | In git? |
|------|---------|
| `deploy-to-vps.sh` | yes |
| `vps.env*.example` | yes |
| `env/*.example` | yes |
| `vps.env`, `env/*.env`, `ssh/id_*` | **no** |

Docs: [docs/deployment/vps-shared-host.md](../docs/deployment/vps-shared-host.md)

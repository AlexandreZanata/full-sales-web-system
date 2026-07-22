# Production deploy (local secrets + VPS)

## Live targets

| Surface | URL |
|---------|-----|
| API | https://vendas.comerc.app.br/v1 |
| Admin | https://vendas.comerc.app.br/admin/ |
| Platform | https://vendas.comerc.app.br/platform/ |
| Catalog | https://catalogo.comerc.app.br |

Data (Postgres / Redis / MinIO) stays **on the VPS** Docker volumes.

## Quick start

```bash
# One password prompt (ControlMaster reuses it for ssh + rsync) — sorrimobi-style
./production/deploy-to-vps.sh
```

Requires TCP `${VPS_PORT:-22}` open to `VPS_HOST`. If probe fails, fix Hostinger firewall first.

```bash
cp production/vps.env.domain.example production/vps.env
# set VPS_HOST=… in production/vps.env
./production/deploy-to-vps.sh --env-only   # secrets + URLs only
./production/deploy-to-vps.sh             # ask password once → sync → remote build
```

Key-based deploy (optional):

```bash
VPS_USE_PASSWORD=0 ./production/deploy-to-vps.sh
```

On VPS after first domain deploy (TLS):

```bash
ssh root@YOUR_VPS_IP
cd /var/www/fullsales
certbot --nginx \
  -d vendas.comerc.app.br \
  -d catalogo.comerc.app.br
./infra/scripts/install-nginx-domain.sh
```

Smoke: `curl -fsS https://vendas.comerc.app.br/health`

Cloudflare: SSL **Full (strict)** after certs exist; DNS `vendas` + `catalogo` → VPS IP (proxied).

## Layout

| Path | In git? |
|------|---------|
| `deploy-to-vps.sh` | yes |
| `vps.env*.example` | yes |
| `env/*.example` | yes |
| `vps.env`, `env/*.env`, `ssh/id_*` | **no** |

Docs: [docs/deployment/vps-shared-host.md](../docs/deployment/vps-shared-host.md)

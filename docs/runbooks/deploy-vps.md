# Deploy to VPS (Full Sales)

## Prerequisites

- Cloudflare DNS for `vendas*` → `YOUR_VPS_IP` (Proxied)
- SSH key authorized on the VPS
- Local `production/vps.env` (from `vps.env.domain.example`)

## Deploy

```bash
./production/deploy-to-vps.sh --env-only   # first time / URL change
./production/deploy-to-vps.sh
```

Remote path runs `infra/scripts/vps-full-deploy.sh`:

1. `prepare-env.sh`
2. `pnpm` SPA builds
3. Compose up Postgres + Redis + MinIO + API (all volumes on VPS)
4. nginx vhost install

## After code lands on `main`

If GitHub secrets `VPS_HOST`, `VPS_USER`, `VPS_SSH_KEY` are set, `.github/workflows/vps-deploy.yml` rsyncs and runs the same remote script.

## Verify

```bash
curl -fsS https://api.vendas.comerc.app.br/health
curl -fsSI https://vendas.comerc.app.br/
```

On VPS: `./infra/scripts/diagnose-nginx-vhosts.sh`

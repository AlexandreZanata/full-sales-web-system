# Deploy staging

> Phase 15F · ADR-019

## Prerequisites

- Images in GHCR (`ghcr.io/<org>/fullsales-*: <sha>`) — workflow **Deploy**
- `kubectl` context for staging
- `deploy/kubernetes/overlays/staging/secrets.env` from `secrets.env.example` (gitignored)
- Nginx Ingress Controller installed ([nginx-ingress.md](../deployment/nginx-ingress.md))
- Origin TLS Secret `cloudflare-origin-tls` (replace placeholder)

## GitHub Actions

1. Push to `main` (paths under `backend/`, SPAs, `deploy/`) → validate + build/push images
2. **Actions → Deploy → Run workflow**
   - `environment`: staging
   - `deploy_cluster`: true
3. Repository secrets:
   - `KUBE_CONFIG` — base64 kubeconfig
   - `STAGING_SECRETS_ENV` — full contents of `secrets.env`
4. Optional variable: `VITE_API_BASE_URL` for SPA builds

Never commit kubeconfig or `secrets.env`.

## Manual apply

```bash
cp deploy/kubernetes/overlays/staging/secrets.env.example \
   deploy/kubernetes/overlays/staging/secrets.env
# edit secrets.env

kubectl apply -k deploy/kubernetes/overlays/staging
kubectl -n fullsales-staging rollout status deploy/api
kubectl -n fullsales-staging port-forward svc/api 18080:80
curl -fsS http://127.0.0.1:18080/health
```

## Smoke

1. `GET /health` → `{"status":"ok"}`
2. `GET /health/ready` with data stores up
3. `GET /v1/status` (public) through Ingress / Cloudflare — no CDN cache
4. Ingress Hosts: `api.` / `admin.` / `portal.` / `platform.`
5. Authenticated `/v1` read (login) once secrets and seed users exist

## Migrations

Run Job `fullsales-migrate` before or with rollout (embedded in overlay). API also migrates on boot via `DATABASE_ADMIN_URL` — prefer Job-first for controlled rollouts.

## Related

- [rollback-deploy.md](rollback-deploy.md)
- [kubernetes.md](../deployment/kubernetes.md)

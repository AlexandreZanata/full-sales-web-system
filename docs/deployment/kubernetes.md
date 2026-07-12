# Kubernetes deployment

> Phase 15 · ADR-019

## Layout

```text
deploy/kubernetes/
  base/                 # Workloads, Ingress, NetworkPolicy, migrate Job, TLS placeholder
  overlays/staging/     # In-cluster Postgres/Redis/MinIO + secretGenerator
  overlays/prod/        # Managed DB URLs + HPA minReplicas 2
```

Apply staging (after `secrets.env` exists):

```bash
cp deploy/kubernetes/overlays/staging/secrets.env.example \
   deploy/kubernetes/overlays/staging/secrets.env
kubectl apply -k deploy/kubernetes/overlays/staging
```

Validate manifests without a cluster:

```bash
pnpm validate:deploy
# or: ./deploy/kubernetes/scripts/validate-manifests.sh
```

## Images

| Service | Image | Dockerfile |
|---------|-------|------------|
| API | `ghcr.io/<org>/fullsales-api:<git-sha>` | `backend/Dockerfile` |
| Migrate Job | same API image, command `fullsales-migrate` | built in API image |
| admin / portal / platform-admin | `ghcr.io/<org>/fullsales-<app>:<git-sha>` | `deploy/docker/spa.Dockerfile` |

### Build commands

```bash
docker build -f backend/Dockerfile -t ghcr.io/example/fullsales-api:local backend

docker build -f deploy/docker/spa.Dockerfile \
  --build-arg APP_NAME=admin \
  --build-arg VITE_API_BASE_URL=https://api.example.com/v1 \
  -t ghcr.io/example/fullsales-admin:local .
# repeat for portal, platform-admin
```

CI: `.github/workflows/deploy.yml` builds/pushes on `main` and can apply staging.

Tagging: immutable short SHA; overlays may use `staging` / `prod` until pinned.

## Health probes

| Path | Use |
|------|-----|
| `GET /health` | Liveness |
| `GET /health/ready` | Readiness |

See [health-monitoring.md](../runbooks/health-monitoring.md). From Ingress: probe `https://api.<host>/health`.

## Migrations

Prefer Job `fullsales-migrate` before API rollout. API also migrates on boot via `DATABASE_ADMIN_URL`.

## Secrets (OD-15-7)

- Overlays use `secretGenerator` from **gitignored** `secrets.env`
- Commit only `secrets.env.example`
- CI injects `STAGING_SECRETS_ENV` — never commit real JWT / Ed25519 / Asaas keys
- Origin TLS: Secret `cloudflare-origin-tls` (replace placeholder)

## Ingress

See [nginx-ingress.md](nginx-ingress.md) and [cloudflare.md](cloudflare.md).

## Hardening

- API: non-root UID 10001, read-only root FS, `emptyDir` for `/tmp`, drop all caps
- NetworkPolicy on API + SPA ingress
- Logs: container stdout → `kubectl -n <ns> logs deploy/api -f`

## Related runbooks

- [deploy-staging.md](../runbooks/deploy-staging.md)
- [rollback-deploy.md](../runbooks/rollback-deploy.md)

# Kubernetes deployment

> Phase 15 · ADR-019

## Layout

```text
deploy/kubernetes/
  base/                 # Deployments, Services, ConfigMaps, NetworkPolicy, migrate Job
  overlays/staging/     # In-cluster Postgres/Redis/MinIO + staging replicas
  overlays/prod/        # External data stores; min 2 API replicas
```

Apply staging:

```bash
kubectl apply -k deploy/kubernetes/overlays/staging
```

Validate manifests without a cluster:

```bash
./deploy/kubernetes/scripts/validate-manifests.sh
```

## Images

| Service | Image | Dockerfile |
|---------|-------|------------|
| API | `ghcr.io/<org>/fullsales-api:<git-sha>` | `backend/Dockerfile` |
| Migrate Job | same API image, command `fullsales-migrate` | built in API image |
| admin / portal / platform-admin | `ghcr.io/<org>/fullsales-<app>:<git-sha>` | `deploy/docker/spa.Dockerfile` |

### Build commands

```bash
# API (+ fullsales-migrate binary)
docker build -f backend/Dockerfile -t ghcr.io/example/fullsales-api:local backend

# SPAs (build context = repo root)
docker build -f deploy/docker/spa.Dockerfile \
  --build-arg APP_NAME=admin \
  --build-arg VITE_API_BASE_URL=https://api.example.com/v1 \
  -t ghcr.io/example/fullsales-admin:local .

docker build -f deploy/docker/spa.Dockerfile \
  --build-arg APP_NAME=portal \
  --build-arg VITE_API_BASE_URL=https://api.example.com/v1 \
  -t ghcr.io/example/fullsales-portal:local .

docker build -f deploy/docker/spa.Dockerfile \
  --build-arg APP_NAME=platform-admin \
  --build-arg VITE_API_BASE_URL=https://api.example.com/v1 \
  -t ghcr.io/example/fullsales-platform-admin:local .
```

Tagging: prefer immutable `<git-sha>`; optional mutable `staging` / `prod` tags for overlays.

## Health probes

| Path | Use |
|------|-----|
| `GET /health` | Liveness — process up |
| `GET /health/ready` | Readiness — dependencies OK |

API Deployment uses both. SPA pods probe `GET /`.

## Migrations

API startup still runs embedded sqlx migrations via `DATABASE_ADMIN_URL`.  
Prefer the one-shot Job `fullsales-migrate` (same image) **before** rolling the API Deployment so schema is ready before new pods become Ready. See `base/migrate-job.yaml`.

## Secrets

Do not commit real secrets. Copy `deploy/kubernetes/overlays/staging/secrets.env.example` → `secrets.env` (gitignored) or inject via CI (OD-15-7).

## Next

Ingress + origin TLS: [nginx-ingress.md](nginx-ingress.md) (Phase 15D).  
Cloudflare: [cloudflare.md](cloudflare.md) (Phase 15E).  
Runbooks: [deploy-staging.md](../runbooks/deploy-staging.md), [rollback-deploy.md](../runbooks/rollback-deploy.md).

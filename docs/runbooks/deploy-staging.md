# Deploy staging

> Phase 15F — stub until CI deploy lands.

## Prerequisites

- Images built and pushed (`docs/deployment/kubernetes.md`)
- `kubectl` context pointed at staging cluster
- Secrets injected (`secrets.env` or CI)

## Apply

```bash
kubectl apply -k deploy/kubernetes/overlays/staging
kubectl -n fullsales-staging rollout status deploy/api
curl -fsS http://<api-service>/health
```

## Smoke

1. `GET /health` → `{"status":"ok"}`
2. `GET /health/ready` → ready (with Postgres/Redis/MinIO up)
3. Open admin/portal/platform-admin Services (or Ingress after 15D)

## Related

- [rollback-deploy.md](rollback-deploy.md)
- [kubernetes.md](../deployment/kubernetes.md)

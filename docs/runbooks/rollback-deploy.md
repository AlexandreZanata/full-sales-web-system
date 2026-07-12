# Rollback deploy

> Phase 15F · rollback-readiness

## Image rollback (< 15 minutes)

```bash
NS=fullsales-staging   # or fullsales-prod
PREV=<previous-git-sha-short>

kubectl -n "$NS" set image deploy/api api=ghcr.io/<org>/fullsales-api:$PREV
kubectl -n "$NS" set image deploy/admin admin=ghcr.io/<org>/fullsales-admin:$PREV
kubectl -n "$NS" set image deploy/portal portal=ghcr.io/<org>/fullsales-portal:$PREV
kubectl -n "$NS" set image deploy/platform-admin platform-admin=ghcr.io/<org>/fullsales-platform-admin:$PREV

kubectl -n "$NS" rollout status deploy/api --timeout=180s
kubectl -n "$NS" port-forward svc/api 18080:80 &
curl -fsS http://127.0.0.1:18080/health
```

Or: `kubectl -n "$NS" rollout undo deploy/api` (and SPA deploys).

## Migrations

- Prefer backward-compatible migrations.
- Destructive changes: two-phase deploy; document compensating SQL before apply.
- Rolling back images does **not** reverse schema — restore from backup if required.

## DB backup before risky migrate

```bash
# Staging example (in-cluster)
kubectl -n fullsales-staging exec deploy/postgres -- \
  pg_dump -U admin full_sales > "backup-staging-$(date -u +%Y%m%dT%H%M%SZ).sql"
```

Verify restore on a scratch database before production migrate.

## Feature flags

No global kill-switch required for image rollback. For risky product features, disable via config/Secret env and restart API without waiting for a bad build.

## Related

- [deploy-staging.md](deploy-staging.md)
- [health-monitoring.md](health-monitoring.md)

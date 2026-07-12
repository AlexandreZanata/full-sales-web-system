# Rollback deploy

> Phase 15F · rollback-readiness — stub until CI tags digests.

## Image rollback

```bash
# Revert API to previous digest (example)
kubectl -n fullsales-staging set image deploy/api api=ghcr.io/<org>/fullsales-api:<previous-sha>
kubectl -n fullsales-staging rollout undo deploy/api
kubectl -n fullsales-staging rollout status deploy/api
curl -fsS http://<api>/health
```

## Migrations

- Prefer backward-compatible migrations.
- Destructive migrations: two-phase deploy; document compensating SQL before apply.
- Rolling back the image does **not** automatically reverse schema — restore from backup if required.

## Related

- [deploy-staging.md](deploy-staging.md)
- [health-monitoring.md](health-monitoring.md)

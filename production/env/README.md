# Production env files

| File | Purpose |
|------|---------|
| `docker.env` | Compose Postgres/Redis/MinIO passwords + host ports |
| `api.env` | API container env (DB/Redis/MinIO URLs on Docker network) |
| `portal.env` / `admin.env` / `platform-admin.env` | Vite `VITE_API_BASE_URL=/v1` |

Generate:

```bash
./infra/scripts/generate-secrets.sh
./infra/scripts/prepare-env.sh
```

Never commit `*.env` (only `*.example`).

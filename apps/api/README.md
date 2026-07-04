# @full-sales/api

HTTP API is implemented in **Rust/Axum**: `backend/crates/api-http`.

| Endpoint | Handler |
|----------|---------|
| `GET /health` | Liveness probe |
| `GET /v1/` | API version stub |

Run via pnpm (delegates to Cargo):

```bash
pnpm --filter @full-sales/api dev
pnpm --filter @full-sales/api test
```

See [docs/API-CONTRACT.md](../../docs/API-CONTRACT.md).

# @full-sales/api

HTTP API is implemented in **Rust/Axum**: `backend/crates/api-http`.

| Endpoint | Handler |
|----------|---------|
| `GET /health` | Liveness probe |
| `GET /v1/` | API version stub |
| `POST /v1/sales` | Create sale (201 + Location) |
| `GET /v1/sales/{id}` | Get sale |
| `POST /v1/sales/{id}/confirm` | Confirm sale |
| `GET /v1/products` | List products (paginated) |

OpenAPI: [`docs/openapi.yaml`](../../docs/openapi.yaml)

Run via pnpm (delegates to Cargo):

```bash
pnpm --filter @full-sales/api dev
pnpm --filter @full-sales/api test
```

See [docs/API-CONTRACT.md](../../docs/API-CONTRACT.md).

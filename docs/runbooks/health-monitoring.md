# Health monitoring runbook

## Overview

Phase 9 probes run every 60s (configurable via `HEALTH_PROBE_INTERVAL_SECS`) and store results in `ops.health_probe_results` (30-day retention).

| Probe | Critical for readiness | Public status component |
|-------|------------------------|-------------------------|
| `postgres` | Yes | API, Portal |
| `redis` | When `REDIS_URL` set | API, Portal |
| `minio` | When `STORAGE_*` set | Storage |
| `asaas` | No | Payments |
| `dns` | No | API, Portal (degraded only) |
| `webhook_queue` | No | API, Portal (degraded only) |

## Endpoints

- **Liveness:** `GET /health` — process up
- **Readiness:** `GET /health/ready` — critical dependencies (503 when not ready)
- **Platform matrix:** `GET /v1/platform/health/matrix` — PlatformAdmin
- **History:** `GET /v1/platform/health/history?probe=postgres&since=<RFC3339>`
- **Public status:** `GET /v1/status`

## Alerts

When a probe fails `HEALTH_ALERT_THRESHOLD` times consecutively (default 3):

1. Row inserted into `ops.ops_alerts`
2. If `OPS_ALERT_WEBHOOK` is set, POST JSON `{ "text", "content" }` (Slack/Discord compatible)

## Public path via Ingress / Cloudflare

| URL | Expect |
|-----|--------|
| `https://api.<platform>/health` | liveness OK through edge |
| `https://api.<platform>/health/ready` | 200 when Postgres/Redis/MinIO OK |
| `https://api.<platform>/v1/status` | public status (no cache at Cloudflare) |

Do not cache `/health*` or `/v1/*` at the CDN — see [cloudflare.md](../deployment/cloudflare.md).

## Manual verification (dev)

```bash
# Readiness — all up
curl -s http://127.0.0.1:8080/health/ready | jq

# Stop Redis → readiness should 503 with redis down
docker compose stop redis
curl -s -o /dev/null -w '%{http_code}\n' http://127.0.0.1:8080/health/ready

# Platform matrix (requires PlatformAdmin token)
curl -s -H "Authorization: Bearer $PLATFORM_TOKEN" \
  http://127.0.0.1:8080/v1/platform/health/matrix | jq
```

## Recovery

1. Fix underlying dependency (Postgres, Redis, MinIO, Asaas API, DNS).
2. Confirm `GET /health/ready` returns 200.
3. Review `ops.ops_alerts` for recent failures; dismiss or document in incident log.
4. Check webhook delivery (`webhook_sent = true` on alert row).

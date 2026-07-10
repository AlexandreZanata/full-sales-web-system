# LGPD compliance runbook

## Audit trail (BR-AU-001)

- PlatformAdmin mutations append immutable rows to `audit.events`
- Fields: `actorId`, `actorType`, `tenantId`, `action`, `resourceType`, `resourceId`, `ip`, `correlationId`
- Append-only trigger blocks UPDATE/DELETE

### Query APIs

| Endpoint | Scope |
|----------|-------|
| `GET /v1/audit/events` | Tenant Admin — own tenant |
| `GET /v1/platform/audit/events` | PlatformAdmin — cross-tenant |

Max date range: **90 days** per query (`AUDIT_RANGE_TOO_WIDE` if exceeded).

## Data export (LGPD access)

### PlatformAdmin

```bash
POST /v1/platform/tenants/{id}/export        # 202 + job id
GET  /v1/platform/tenants/{id}/export/{jobId} # status + presigned downloadUrl
```

ZIP contains: `users.json`, `commerces.json`, `orders.json`, `sales.json`, `manifest.json` — no secrets or credentials.

### Tenant Admin

```bash
POST /v1/settings/data-export
GET  /v1/settings/data-export/{jobId}
```

Storage bucket: `EXPORT_BUCKET` env (default `exports`).

## Retention and erasure

- Offboarding retention: **90 days** (`application::tenants::RETENTION_DAYS`, aligns with 0-OD-008)
- Job: `POST /v1/platform/jobs/offboarding` anonymizes PII after retention
- **Legal hold:** set `settings.legalHold = true` on tenant — skips anonymization; audit/billing rows retained

### Anonymization mapping

| Field | After anonymization |
|-------|---------------------|
| User email | `deleted-{uuid}@anonymized.local` |
| User name | `Redacted` |
| Tenant display | `Deleted` |

## Checklist

- [ ] Audit emitted for every PlatformAdmin mutation in scope
- [ ] Export ZIP verified — no API keys, password hashes, or webhook tokens
- [ ] Legal hold documented before forced erasure
- [ ] 90-day audit query limit enforced in UI filters

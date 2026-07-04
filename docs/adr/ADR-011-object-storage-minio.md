# ADR-011: Self-hosted MinIO for object storage

**Status:** Accepted  
**Date:** 2026-07-04  
**Deciders:** Phase 0d domain expansion sign-off

## Context

Phases 07–12 store binary assets (product images, profile photos, delivery proof) outside PostgreSQL. DE-009 asked whether to use self-hosted MinIO or a cloud S3-compatible provider.

Requirements: S3-compatible API, presigned URLs (~15 min TTL), private buckets, SHA-256 integrity metadata in `media.files`, local/dev parity with production.

## Decision

**Self-hosted MinIO** as the default object storage backend via a new `infra-storage` crate implementing an `ObjectStorage` port (put, delete, presigned_get).

Configuration via environment (`STORAGE_ENDPOINT`, `STORAGE_ACCESS_KEY`, `STORAGE_SECRET_KEY`, `STORAGE_BUCKET`). Production may point MinIO client at any S3-compatible endpoint without code changes.

## Consequences

### Positive

- No cloud vendor lock-in for MVP; runs in Docker Compose alongside Postgres/Redis
- Same API as AWS S3 — migration to cloud bucket later is config-only
- Aligns with Phase 07 MODULE-MAP and `infra-storage` plan

### Negative

- Ops owns MinIO availability, backups, and disk capacity
- No managed CDN — presigned URLs hit origin unless fronted later

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| Cloud S3 (AWS/GCS) | Higher ops cost and credential management for MVP; defer until scale |
| Postgres bytea | Violates media module design; poor fit for photos |

## Related

- DE-009 in `.local/phases/0d-domain-expansion/documentation/OPEN-DECISIONS-EXPANSION.md`
- Phase 07 — `media.files` metadata only

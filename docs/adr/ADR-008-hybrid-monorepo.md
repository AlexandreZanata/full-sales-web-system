# ADR-008: Hybrid monorepo — Rust backend + TypeScript web client

**Status:** Accepted  
**Date:** 2026-07-04  
**Deciders:** TECH-STACK.md, Phase 1 foundation sign-off

## Context

Phase `01-monorepo-scaffold` originally assumed a full TypeScript stack (Fastify/Nest API, TS domain). Phase `01-foundation` already established **Rust/Axum** for domain, application, and HTTP API per product spec.

## Decision

Use a **hybrid monorepo**:

| Layer | Location | Technology |
|-------|----------|------------|
| Domain | `backend/crates/domain-*` | Rust |
| Application | `backend/crates/application` | Rust |
| HTTP API | `backend/crates/api-http` | Axum (`apps/api` delegates to Cargo) |
| Web client | `apps/web` | Vite + React + TypeScript |
| Client types | `packages/domain`, `packages/application` | TypeScript shells (no HTTP/ORM) |

Root `pnpm` orchestrates the web workspace; `backend/` Cargo workspace owns the backend.

## Consequences

### Positive

- Single repo with clear separation: Rust absorbs business complexity; web stays thin
- No duplicate API servers (TS + Rust)
- CI runs both toolchains

### Negative

- Two package managers (pnpm + Cargo) — mitigated by root `pnpm verify` script
- Client domain types duplicated until OpenAPI codegen (Phase 3+)

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| Full TS monorepo per original TASKS | Conflicts with TECH-STACK and completed Rust foundation |
| Rust-only, no web scaffold | ROADMAP backlog includes simple web UI |

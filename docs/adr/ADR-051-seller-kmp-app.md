# ADR-051: Separate KMP seller app vs extending field app

**Status:** Accepted  
**Date:** 2026-07-05  
**Deciders:** Phase 52–65 seller KMP implementation

## Context

The monorepo already has `apps-mobile/field` — a KMP app for **Driver** and **Seller** roles with delivery routes, offline sales, and shared catalog sync. Product requirements for a dedicated seller experience (Seller-only auth, no deliveries, Compose Multiplatform iOS, Material 3 shell) raised whether to extend `field` or add a new app.

## Decision

Create a **separate** Kotlin Multiplatform app at `apps-mobile/seller`:

| Aspect | `apps-mobile/field` | `apps-mobile/seller` |
|--------|---------------------|----------------------|
| Primary actor | Driver + Seller | Seller only |
| Navigation | Sales, deliveries, proof upload | Sales, create, catalog browse |
| Auth gate | Both roles | JWT `Seller` role only |
| iOS | Not in scope (Phase 39) | Compose Multiplatform + Keychain (Phase 64) |
| UI theme | Compose (evolving) | Material 3 app-wide (Phase 60+) |
| Package id | `com.fullsales.field` | `com.fullsales.seller` |

Shared patterns (not shared Gradle modules yet): `SellerApiClient` mirrors field HTTP contracts; Room + outbox + `SyncEngine` follow the same offline-first model as field Phase 39F.

### Connectivity + push-first + cache-first (Phase 14)

- Prefer **validated** reachability (`NET_CAPABILITY_VALIDATED` / `NWPathMonitor`) over “has interface”.
- Debounce flaps: Offline immediate; Online after a stable window (2s).
- Outbox **push must not wait** on catalog pull success; auto-push once on stable Online.
- Detail/read paths are **cache-first** (Room catalog + stock/address snapshots); online enrich is optional.
- **Phase 16A:** Room v5 adds commerce CNPJ, product UOM/description, sale origin/driverId, sale-line prices; explicit `MIGRATION_4_5` (no wipe from v4).
- **Phase 16B:** Sales list is LocalStore-first; `PullSalesSync` mirrors remote sales; online create upserts Room.

## Consequences

### Positive

- Smaller APK/IPA surface — no delivery code paths in seller builds
- Seller-only RBAC at login without runtime role branching in navigation
- Independent release cadence and CI jobs (`seller-kmp`, `seller-ios`)
- Compose Multiplatform iOS without blocking field Android maintenance

### Negative

- Duplicated KMP scaffolding (`shared`, sync, API client) — mitigated by mirroring proven field patterns
- Two mobile apps to maintain until a future shared `mobile-core` library is extracted

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| Extend `field` with seller flavor | Delivery routes and Driver auth would remain in seller builds; harder M3/iOS CMP migration |
| PWA only for seller | Offline outbox + native sync required for field parity |
| Single shared `mobile-ui` module now | YAGNI — extract after seller ships and duplication cost is measured |

## References

- [seller-mobile-app.md](../features/seller-mobile-app.md)
- [client-apps.md](../features/client-apps.md)
- [ADR-008](ADR-008-hybrid-monorepo.md) — hybrid monorepo layout

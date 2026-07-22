# Seller mobile app — KMP (Phase 52–65)

> Kotlin Multiplatform seller shell at `apps-mobile/seller`. Android (Room + WorkManager) and iOS (Compose Multiplatform + Keychain). **Seller role only** — no delivery routes.

## Packages

| Module | Purpose |
|--------|---------|
| `shared` | API client, sync engine, repositories, i18n, JWT role gate |
| `composeApp` | Compose Multiplatform UI — M3 theme, navigation, ViewModels |
| `androidApp` | Activity shell, Room, WorkManager, EncryptedSharedPreferences |
| `iosApp/` | SwiftUI host for `MainViewController()` — see [iosApp/README.md](../../apps-mobile/seller/iosApp/README.md) |

## Dev login

| Field | Value |
|-------|-------|
| Email | `seller@test.com` |
| Password | `secret123` |

Requires dev seed (`pnpm seed:dev`) and API on `:8080`. **Driver accounts are rejected** at login (`JwtRoleGate` → `NotSeller`).

## API base URL

| Platform | URL |
|----------|-----|
| Android emulator (debug) | `http://10.0.2.2:8080/v1` |
| iOS simulator | `http://127.0.0.1:8080/v1` |
| Physical device (LAN debug) | `http://<host-lan-ip>:8080/v1` |
| **Android release (Play)** | `https://api.vendas.comerc.app.br/v1` |

Override at build time:

```bash
# Env (preferred for install scripts)
SELLER_API_BASE_URL=http://172.19.2.162:8080/v1 ./gradlew :androidApp:installDebug

# Or local.properties
# seller.api.base.url=http://172.19.2.162:8080/v1

# Release / staging override
SELLER_RELEASE_API_BASE_URL=https://staging.example/v1 ./gradlew :androidApp:bundleRelease
```

USB without LAN: `adb reverse tcp:8080 tcp:8080` then keep emulator default `10.0.2.2` (or `127.0.0.1` on device with reverse).

Wrong/unreachable host → sticky Offline banner (server reason) via periodic `GET /health` probe — not a silent empty UI.

## Play Store readiness (Phase 21)

| Item | Value |
|------|-------|
| `applicationId` | `com.fullsales.seller` |
| Version | `1` / `1.0.0` |
| `allowBackup` | `false` |
| Cleartext | Debug only |
| Privacy | `https://vendas.comerc.app.br/privacy-seller.html` |
| Preflight | `pnpm mobile:seller:play-preflight` |

Runbook: [docs/mobile/seller-play-store.md](../mobile/seller-play-store.md).


## Commands

```bash
# From repo root
pnpm mobile:seller:check
pnpm mobile:seller:play-preflight

# Full quality gate (Phase 65E)
cd apps-mobile/seller
./gradlew :shared:check :androidApp:lint :androidApp:assembleDebug
./gradlew :composeApp:compileDebugKotlinAndroid
./gradlew :shared:compileKotlinIosSimulatorArm64 :composeApp:compileKotlinIosSimulatorArm64  # macOS only
pnpm verify:backend   # from repo root

# Instrumented tests (emulator required) — includes LaunchSmokeInstrumentedTest
./gradlew :androidApp:connectedDebugAndroidTest
```

Open `apps-mobile/seller` in Android Studio → run **androidApp**. iOS: see [iosApp/README.md](../../apps-mobile/seller/iosApp/README.md).

## Navigation routes

| Route | Screen | API (when online) |
|-------|--------|-------------------|
| `login` | Login + locale switcher | `POST /v1/auth/login` |
| `offline` | Offline hub (status, last sync, pending outbox) | — (local) |
| `sales` | Sales list (merged remote + local) | `GET /v1/sales` |
| `sales/new` | Create sale form | `POST /v1/sales` or offline outbox |
| `sales/{saleId}` | Sale detail, confirm/cancel | `GET /v1/sales/{id}`, `POST …/confirm`, `POST …/cancel` |
| — | Top sellers (create-sale picker) | `GET /v1/products/top-selling?limit=5` |
| `commerces` | Commerce catalog | Cached + `GET /v1/commerces` |
| `commerces/pick` | Commerce picker (create sale) | Same as commerces |
| `commerces/{commerceId}` | Commerce detail + addresses | `GET /v1/commerces/{id}`, `GET …/addresses` |
| `commerces/registrations` | Registration mode picker | — |
| `commerces/registrations/lookup` | CNPJ lookup + prefill | `GET /v1/commerces/cnpj-lookup` |
| `commerces/registrations/form` | Submit registration | `POST /v1/commerces/registrations` |
| `commerces/registrations/mine` | My submissions | `GET /v1/commerces/registrations` |
| `products` | Product catalog | Cached + `GET /v1/products` |
| `products/{productId}` | Product detail + stock | `GET /v1/products/{id}`, `GET /v1/inventory/products/{id}/balance` |

Bottom nav: **Sales** and **New sale** only. No delivery routes.

## API client coverage (SELLER-ROUTE-MATRIX)

All seller-facing HTTP routes have `SellerApiClient` methods and MockEngine unit tests in `shared/src/commonTest/`.

| Method | Path | Client |
|--------|------|--------|
| POST | `/auth/login`, `/auth/refresh`, `/auth/logout` | ✅ |
| GET | `/settings` | ✅ |
| GET | `/commerces`, `/commerces/{id}`, `/commerces/{id}/addresses` | ✅ |
| GET | `/products`, `/products/{id}` | ✅ |
| GET | `/inventory/products/{id}/balance` | ✅ |
| GET/POST | `/sales`, `/sales/{id}`, `/sales/{id}/confirm`, `/sales/{id}/cancel` | ✅ |
| GET/POST | `/media/{id}/url`, `/media/upload` | ✅ client only — **upload UI deferred** |

## Offline sync

Phase 14–16 — validated connectivity, push-first sync, **local-first SQLite** for seller field reads/writes (except internet-only), durable iOS LocalStore.

1. **Connectivity:** `NetworkMonitor.connectivity: StateFlow<ConnectivityState>` (`Offline` | `Connecting` | `Online`). Offline is immediate; Online after **2s** continuous validated reachability (`DebouncedConnectivity`). Android uses `NetworkCallback` + `NET_CAPABILITY_INTERNET` **and** `NET_CAPABILITY_VALIDATED`; iOS uses `NWPathMonitor`.
2. **Create:** offline **or** online transport failure → `OfflineSaleWriter` → LocalStore (`PendingSync`) + outbox (same Pending sync UX). Business `ApiException` still fails hard.
3. **Confirm/cancel (OD-16-2):** with `remoteId` → optimistic local `Confirmed`/`Cancelled` + outbox. Without `remoteId` but pending create outbox → **chained** outbox (`dependsOnOutboxId`); SyncEngine skips dependents until parent completes, then rewrites confirm/cancel path to the synced remote id. Without create outbox → blocked (`NO_REMOTE_ID`).
4. **Sync:** `SellerSyncCoordinator.pushOutbox()` then best-effort `pullCatalog()` + **`pullSales()`** + **`pullRegistrations()`** — pull failures never block push. Stable Offline→Online auto-drains outbox once (`OnlineSyncTrigger`). WorkManager (Android) / become-active (iOS) remain secondary. Exhausted retries (`attempts >= max`) → `SyncFailed` (dead-letter UX).
5. **Internet-only:** CNPJ lookup CTAs disabled when not Online (“Disponível com internet”). Registration **submit** queues offline (OD-16-1): LocalStore `PendingSync` + outbox `POST /commerces/registrations`. Draft form stays editable.
6. **Cache-first reads (14D / 16A):** Product/commerce detail load from LocalStore (and address/stock snapshots) first; API enrich when Online. **Room schema v8** (Android) / **SQLDelight** (iOS) store commerce `cnpj`, product `unitOfMeasure`/`description`, sale `driverId`/`origin`, sale-line unit prices, registrations, media/settings. Detail enrich upserts UOM/description; catalog list pull **preserves** those detail fields when the list payload omits them. Stock balances persist until next successful fetch (no hard TTL). Browse/create-sale uses cached stock for backorder warnings; unknown stock does not hide products.
7. **Offline chrome (14E + 18B/C):** Shell chip shows Offline / Syncing / Online / Sync failed (TalkBack live region); pull-to-refresh offline shows “Sem conexão” and does not spin. Sticky Offline banner under the app bar when `ConnectivityState.Offline` **or** Online + failed `GET /health` probe; pending outbox count chip; tap → `/offline` hub (last sync stamps, queue, try sync / continue offline). i18n en + pt-BR under `SellerMessages.offline`.
8. **Idempotency:** UUID v7 key on `POST /v1/sales` and `POST /v1/commerces/registrations`; server dedupes retries.
9. **Migrations (16A / OD-16-4):** `SellerMigrations.MIGRATION_4_5` is explicit; installs from v4 keep sales. Destructive fallback only for versions 1–3. **v5→v6 (16C):** creates `registrations` + `sync_outbox.entityType`. **v6→v7 (16D):** renames outbox `saleLocalId` → `aggregateId`, adds `dependsOnOutboxId`. **v7→v8 (16E):** `media_url_cache` + `site_settings`.
10. **Sales local-first (16B):** `PullSalesSync` pages `GET /sales` into LocalStore (OD-16-3: mirrors use `localId = remoteId`). Sales list observes LocalStore only; online refresh runs sync pulls (never RAM-only remote list). Online create success upserts LocalStore as `Synced`. Metadata key `lastSalesSync`.
11. **Registrations local-first (16C):** `PullRegistrationsSync` pages `GET /commerces/registrations` into LocalStore. My registrations observes LocalStore; offline refresh shows snackbar and keeps list. Offline/online-transport-failure submit → `OfflineRegistrationWriter` outbox. Pending/sync-failed show as chips via `registrationStatus` mapping.
12. **Generic outbox (16D):** Outbox keyed by `entityType` + `aggregateId`; SyncEngine FIFO honors `dependsOnOutboxId` (skip until parent completed; network stop-early unchanged).
13. **Settings + media (16E / OD-16-8=A):** `site_settings` + `media_url_cache` (fileId, url, expiresAt). Offline image resolve uses non-expired cache; expired/missing → null/placeholder (no crash). Sync best-effort `pullSettings()`. **OD-16-6=B:** top-seller chips hide when offline.
14. **Empty / error UX (16F):** Shared `ListEmptyReason` (`NeverSynced` | `SyncedEmpty` | `OfflineUnavailable` | `RefreshFailedKeepCache`) drives sales/products/commerces/registrations empty copy. Virgin offline install shows bootstrap (“connect once”). Online refresh failure with LocalStore rows → snackbar keep-cache; lists never wipe to empty.
15. **iOS durable LocalStore (16G / OD-16-5=A):** SQLDelight shared schema (`SellerLocalDatabase`) matches Room field inventory. `IosAppContainer` uses `SqlDelight*` repositories (not memory). Outbox + sales survive process kill. Sync drains via `OnlineSyncTrigger` (Offline→Online) and `UIApplicationDidBecomeActive` → `IosForegroundSync` / `onAppResume`. **No WorkManager equivalent** on iOS — background push while suspended is not required. CI `seller-ios` compiles `:shared` + `:composeApp` iosSimulatorArm64.

**Manual device script (PO):** sync catalog online → open product with image → airplane → image still loads if URL unexpired → create sale offline (no top-seller chips) → confirm offline (chained) → submit registration offline → go Online → outbox drains. Virgin airplane install → bootstrap empty (not silent blank). Online with cache + API down → list kept + refresh snackbar. **iOS:** create sale offline → force-quit → reopen → sale + pending outbox still present → go Online → drains. Rapid airplane flaps → no blank lists; outbox drains after stable Online.

Tests: `DebouncedConnectivityTest`, `SellerSyncCoordinatorTest`, `CacheFirstDetailLoaderTest`, `ProductDetailEnrichPersistTest`, `PullSalesSyncTest`, `SalesListLocalFirstTest`, `ListEmptyReasonTest` (T-16-13), `SqlDelightLocalStorePersistenceTest` (T-16G reopen), `SyncEngineTest`, `SyncEngineDependencyTest`, `CreateSaleSubmitterTest`, `RegistrationOutboxTest`, `RegistrationRepositoryTest`, `SaleActionSubmitterTest`, `MediaUrlCacheTest`, `OfflineSalePersistenceTest`, `CatalogFieldPersistenceTest`, `SellerDatabaseMigrationTest`, `SyncOutboxMigrationTest`, `MediaSettingsMigrationTest`, `RemoteSalePersistenceTest`, `RegistrationPersistenceTest` (Robolectric), `OfflineBannerStateTest`, `OfflineFieldVisibilityTest`, `SellerApiProbeTest` (Phase 18), instrumented outbox/create/`OfflineSalesLocalFirstInstrumentedTest`/`OfflineRegistrationInstrumentedTest` (device with install allowed).

## Accessibility (Phase 66)

| Feature | Implementation |
|---------|----------------|
| System font scale | `SellerTheme` composes `LocalDensity` with OS `fontScale` × in-app preset |
| In-app text size | `TextSizePreset` (`Normal` 1.0 / `Large` 1.15 / `ExtraLarge` 1.3) via `AccessibilityStore` + `TextSizeSwitcher` on login and shell |
| TalkBack labels | `SellerMessages.A11y` strings; list rows use `contentDescription` summaries |
| Touch targets | Primary actions use `defaultMinSize(48.dp)` minimum height |
| Commerce picker | Auto-sync on first open (`COMMERCE_PICK` / `COMMERCES`); offline empty mentions pull-to-sync |

**Manual validation (Android):** Settings → Display → Font size **Largest**; enable TalkBack; walk login → sales list → create sale → commerce pick. Verify preset switcher on login and shell top bar.

## i18n

Default locale: **pt-BR**. Switch EN/PT via M3 `SegmentedButton` on login and shell top bar — no app restart. Messages in `shared/i18n/`.

## Material Design 3

**Library:** `androidx.compose.material3` only for UI components.  
**Icons:** `androidx.compose.material.icons` (extended icons pack — not M2 Material components).

| Item | Implementation |
|------|----------------|
| Entry | `SellerTheme { … }` in `SellerNavHost` — root `Surface` uses `colorScheme.background` |
| Colors | `lightColorScheme` / `darkColorScheme`; dynamic color on Android 12+ |
| Typography / shapes | `MaterialTheme.typography`, `MaterialTheme.shapes` |
| Components | `TopAppBar`, `NavigationBar`, `Card`, `AssistChip`, `OutlinedTextField`, `Snackbar`, `PullToRefreshBox`, etc. |

**Do not use** `androidx.compose.material` (Material 2) for screens or theme.

### System UI (status bar) — do not regress

Android status-bar icons (clock, battery, signal) **must** contrast with the app background:

| System mode | Icon color | Configuration |
|-------------|------------|---------------|
| Light | **Dark** icons | `values/themes.xml` → `android:windowLightStatusBar=true` |
| Dark | **Light** icons | `values-night/themes.xml` → `android:windowLightStatusBar=false` |

Also required:

1. **`values-night/themes.xml`** — without it, `isSystemInDarkTheme()` stays false and Compose keeps the light palette in dark mode.
2. **`MainActivity.enableEdgeToEdge()`** — edge-to-edge with transparent status bar.
3. **`SellerSystemBarsEffect(darkTheme)`** — runtime sync in `SellerTheme.android.kt` when configuration changes.

**Checklist after theme edits:** toggle system light/dark → status-bar icons readable; Compose background matches `SellerDarkColors` / `SellerLightColors`.

### Create-sale form — product list and draft

- **Product lines:** thumbnail (remote URL or inventory icon placeholder), name, SKU badge, quantity stepper; picker hidden after selection — remove line/product with **×** only.
- **Draft persistence:** commerce, payment method, and line items auto-save to platform storage (`CreateSaleDraftStore`); restored on return. **Limpar / Clear** resets the form; draft cleared on successful submit.
- **Manual check:** fill form → force-stop app → reopen **New sale** → fields restored; **Clear** empties all fields.

### Create-sale bottom bar — button conventions

| Action | Component | Label key |
|--------|-----------|-----------|
| Back | `OutlinedButton` (outline border, 48 dp min height) | `common.back` |
| Submit | `Button` (filled primary) | `sales.confirmShort` (`Confirmar` / `Confirm`) — **not** `sales.confirm` |

`SaleDetailScreen` already uses `confirmShort` for the same reason (short label fits narrow layouts).

### Shell overflow menu

Accessibility presets (text size + logout) live in `ShellOverflowMenu` — not inline chips in the top bar. Login screen keeps `LoginAccessibilityPanel` with `FilterChip` / `SegmentedButton` for first-run discovery.

iOS hosts the same Compose UI and M3 theme via `MainViewController()`.

## Manual acceptance script

Run with API + seed + Android emulator:

1. Login as `seller@test.com` → shell opens.
2. Login as `driver-a@test.com` → error (not Seller).
3. **Sales** tab → list loads; pull-to-refresh works.
4. **New sale** → pick commerce, add line item, submit online → appears in list.
5. Airplane mode → create sale → `Pending sync` badge; disable airplane mode → sync completes.
6. Open sale detail → **Confirm** (pending sale with remote id).
7. Locale switcher: PT default labels; switch EN → labels update without restart.
8. Verify no crash on cold start with stored session.

## CI

GitHub Actions jobs `seller-kmp` (Ubuntu) and `seller-ios` (macOS) — see [.github/workflows/ci.yml](../../.github/workflows/ci.yml).

## Related docs

- [client-apps.md](client-apps.md) — all client packages
- [DEV-COMMANDS.md](../DEV-COMMANDS.md) — monorepo commands
- [ADR-051](../adr/ADR-051-seller-kmp-app.md) — separate seller app vs field
- Module README: [apps-mobile/seller/README.md](../../apps-mobile/seller/README.md)

**Updated:** 2026-07-14

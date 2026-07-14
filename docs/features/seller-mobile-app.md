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
| Android emulator | `http://10.0.2.2:8080/v1` |
| iOS simulator | `http://127.0.0.1:8080/v1` |
| iOS physical device | `http://<host-lan-ip>:8080/v1` |

Override via `SELLER_API_BASE_URL` in root `.env` (see `.env.example`).

## Commands

```bash
# From repo root
pnpm mobile:seller:check

# Full quality gate (Phase 65E)
cd apps-mobile/seller
./gradlew :shared:check :androidApp:lint :androidApp:assembleDebug
./gradlew :composeApp:compileDebugKotlinAndroid
./gradlew :shared:compileKotlinIosSimulatorArm64 :composeApp:compileKotlinIosSimulatorArm64  # macOS only
pnpm verify:backend   # from repo root

# Instrumented tests (emulator required)
./gradlew :androidApp:connectedDebugAndroidTest
```

Open `apps-mobile/seller` in Android Studio → run **androidApp**. iOS: see [iosApp/README.md](../../apps-mobile/seller/iosApp/README.md).

## Navigation routes

| Route | Screen | API (when online) |
|-------|--------|-------------------|
| `login` | Login + locale switcher | `POST /v1/auth/login` |
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

Phase 14 — validated connectivity, push-first sync, offline-first mutations, cache-first reads.

1. **Connectivity:** `NetworkMonitor.connectivity: StateFlow<ConnectivityState>` (`Offline` | `Connecting` | `Online`). Offline is immediate; Online after **2s** continuous validated reachability (`DebouncedConnectivity`). Android uses `NetworkCallback` + `NET_CAPABILITY_INTERNET` **and** `NET_CAPABILITY_VALIDATED`; iOS uses `NWPathMonitor`.
2. **Create:** offline **or** online transport failure → `OfflineSaleWriter` → Room (`PendingSync`) + outbox (same Pending sync UX). Business `ApiException` still fails hard.
3. **Confirm/cancel:** with `remoteId` → optimistic local `Confirmed`/`Cancelled` + pending sync chip while outbox drains. Without `remoteId` → blocked (`NO_REMOTE_ID` / “Aguardando sincronização”).
4. **Sync:** `SellerSyncCoordinator.pushOutbox()` then best-effort `pullCatalog()` + **`pullSales()`** — pull failures never block push. Stable Offline→Online auto-drains outbox once (`OnlineSyncTrigger`). WorkManager / onResume remain secondary. Exhausted retries (`attempts >= max`) → `SyncFailed` (dead-letter UX).
5. **Internet-only:** CNPJ lookup + registration submit CTAs disabled when not Online (“Disponível com internet”). Draft form stays editable; my-registrations keeps last in-memory list and skips refresh offline.
6. **Cache-first reads (14D / 16A):** Product/commerce detail load from Room (and address/stock snapshots) first; API enrich when Online. **Room schema v5** stores commerce `cnpj`, product `unitOfMeasure`/`description`, sale `driverId`/`origin`, and sale-line unit prices. Detail enrich upserts UOM/description into LocalStore; catalog list pull **preserves** those detail fields when the list payload omits them. Stock balances persist in Room (`stock_snapshots`) until next successful fetch (no hard TTL). Browse/create-sale uses cached stock for backorder warnings; unknown stock does not hide products.
7. **Offline chrome (14E):** Shell chip shows Offline / Syncing / Online / Sync failed (TalkBack live region); pull-to-refresh offline shows “Sem conexão” and does not spin.
8. **Idempotency:** UUID v7 key on `POST /v1/sales`; server dedupes retries.
9. **Migrations (16A / OD-16-4):** `SellerMigrations.MIGRATION_4_5` is explicit; installs from v4 keep sales. Destructive fallback only for versions 1–3.
10. **Sales local-first (16B):** `PullSalesSync` pages `GET /sales` into Room (OD-16-3: mirrors use `localId = remoteId`). Sales list observes LocalStore only; online refresh runs sync pulls (never RAM-only remote list). Online create success upserts LocalStore as `Synced`. Metadata key `lastSalesSync`.

**Manual device script:** sync catalog online → airplane → open product/commerce/sale detail from cache → create/confirm with remoteId → CNPJ/registration disabled → toggle airplane rapidly → outbox drains after stable Online.

Tests: `DebouncedConnectivityTest`, `SellerSyncCoordinatorTest`, `CacheFirstDetailLoaderTest`, `ProductDetailEnrichPersistTest`, `PullSalesSyncTest`, `SalesListLocalFirstTest`, `SyncEngineTest`, `CreateSaleSubmitterTest`, `SaleActionSubmitterTest`, `OfflineSalePersistenceTest`, `CatalogFieldPersistenceTest`, `SellerDatabaseMigrationTest`, `RemoteSalePersistenceTest` (Robolectric), instrumented outbox/create/`OfflineSalesLocalFirstInstrumentedTest`.

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

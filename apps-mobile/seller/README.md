# Full Sales Seller — Kotlin Multiplatform (Phase 52+)

Android shell + `shared` KMP module. Phases 54–56 add HTTP client, Room offline storage, and WorkManager sync.

## Prerequisites

- JDK 17
- Android Studio Ladybug+ (or command-line SDK)
- Android SDK API 35, min SDK 26

## Local API URL (emulator)

```text
http://10.0.2.2:8080/v1
```

(`10.0.2.2` is the host loopback from the Android emulator.)

Override at Gradle build time (see root `.env.example`):

```bash
SELLER_API_BASE_URL=http://<lan-ip>:8080/v1 ./gradlew :androidApp:installDebug
# or local.properties: seller.api.base.url=http://<lan-ip>:8080/v1
adb reverse tcp:8080 tcp:8080   # USB alternative
```

### Release / Play Store (Phase 21)

| Item | Value |
|------|-------|
| `applicationId` | `com.fullsales.seller` |
| Release API | `https://api.vendas.comerc.app.br/v1` |
| Version | `versionCode=1` / `versionName=1.0.0` |
| `allowBackup` | `false` |
| Privacy | `https://vendas.comerc.app.br/privacy-seller.html` |

```bash
# From repo root — mandatory before Play upload
pnpm mobile:seller:play-preflight
pnpm verify:no-android-secrets
```

Full runbook: [docs/mobile/seller-play-store.md](../../docs/mobile/seller-play-store.md).  
Release signing / AAB: [docs/mobile/seller-release-build.md](../../docs/mobile/seller-release-build.md).

### Catalog share link

Share URLs come from the API: `GET /v1/me/seller-share` → `shareUrl` (built from backend `PORTAL_PUBLIC_ORIGIN` + `/s/{code}`).

Configure on the API (not in the Android app):

```bash
# backend/.env
PORTAL_PUBLIC_ORIGIN=http://192.168.15.15:5175
```

Create `local.properties` (gitignored):

```properties
sdk.dir=/path/to/Android/sdk
```

### Offline UX (Phase 18)

- Sticky Offline banner when airplane / no route **or** API `GET /health` fails while Wi‑Fi is up
- Tap banner → Offline hub (`offline` route): last sync, pending outbox, try sync / continue
- Cached create-sale / detail fields stay visible from LocalStore (see phase `FIELD-VISIBILITY-MATRIX.md`)

## Build

```bash
cd apps-mobile/seller
./gradlew :shared:check :composeApp:compileDebugKotlinAndroid :androidApp:lint :androidApp:assembleDebug
./gradlew :shared:compileKotlinIosSimulatorArm64 :composeApp:compileKotlinIosSimulatorArm64  # macOS only
./scripts/play-preflight.sh   # release AAB gate (Play)
```

From repo root:

```bash
pnpm mobile:seller:check
pnpm mobile:seller:play-preflight
```

## Run

Open `apps-mobile/seller` in Android Studio → run **androidApp** on an emulator or device.

For iOS simulator (macOS + Xcode): see [iosApp/README.md](iosApp/README.md).

## Structure

| Module | Purpose |
|--------|---------|
| `shared` | KMP common code — API client, sync engine, repositories |
| `composeApp` | Compose Multiplatform UI — `SellerNavHost`, M3 theme (Android + iOS) |
| `androidApp` | Android Activity, Room, WorkManager, EncryptedSharedPreferences |
| `iosApp/` | SwiftUI shell hosting `MainViewController()` — see `iosApp/README.md` |

### Shared API layer (Phase 54)

| Package | Contents |
|---------|----------|
| `shared/api/` | `SellerApiClient`, `HttpClientFactory`, `ApiError`, Bearer auth + 401 refresh interceptor |
| `shared/model/` | DTOs mirroring `apps/field/src/lib/api/types.ts` (camelCase JSON) |
| `shared/auth/` | JWT role gate (`Seller` only), `SecureTokenStore` expect/actual (iOS stub) |

All Seller routes from `SELLER-ROUTE-MATRIX` have client methods. Unit tests use Ktor `MockEngine`.

### Seller auth (Phase 53)

| Component | Purpose |
|-----------|---------|
| `AuthViewModel` | Login, logout, session restore with JWT role gate |
| `TokenStore` | Android `EncryptedSharedPreferences` for access/refresh tokens |
| `SellerTokenRefresher` | Shared refresh for Ktor 401 retry + sync engine |
| `LoginScreen` | M3 form — `OutlinedTextField`, `Button`, error `Card`; debug prefill |
| `AuthRefreshPlugin` | On 401 (non-auth paths): refresh once, retry with new Bearer token |

Login contract matches field PWA (`apps/field/src/lib/api/client.ts`). Only `Seller` role enters the shell.

### Offline persistence (Phase 55)

| Package | Contents |
|---------|----------|
| `shared/repository/` | `CatalogRepository`, `SaleRepository`, `SyncOutboxRepository` (interfaces) |
| `shared/sync/` | `OfflineSaleWriter` — local sale + outbox enqueue |
| `shared/db/` (androidMain) | Room `SellerDatabase`, entities, DAOs, `Room*Repository` |

Local sales use UUID v7 idempotency keys. Catalog sync uses atomic replace-all writes.

### Sync engine (Phase 56)

| Package | Contents |
|---------|----------|
| `shared/sync/` | `CatalogPullSync`, `SyncEngine`, `SellerSyncCoordinator`, `OfflineSaleWriter` |
| `shared/api/SellerSyncTransport.kt` | Outbox transport + catalog pull via `SellerApiClient` |
| `androidApp/sync/SyncWorker.kt` | Periodic + one-time WorkManager jobs (network required) |

Foreground sync runs on `MainActivity.onResume`. After login (Phase 53), offline sales replay with the stored idempotency key.

### App shell (Phase 57)

| Component | Purpose |
|-----------|---------|
| `SellerNavHost` | Auth-gated navigation — sales, new sale, detail placeholders |
| `SellerShellScaffold` | Top bar (tenant branding + sync badge) + 2-tab bottom nav |
| `LoginScreen` | Seller-only login with JWT role gate |
| `SettingsViewModel` | `GET /v1/settings` with 5 min cache |

No delivery routes — Seller nav matches field PWA (Sales + New sale only).

### Commerces UI (Phase 58)

| Component | Purpose |
|-----------|---------|
| `CommerceListScreen` | Cached list + pull-to-refresh sync; client-side search; active filter |
| `CommerceDetailScreen` | `GET /commerces/{id}` + addresses; masked CNPJ |
| `CommerceViewModel` | Room `observeCommerces()` + `SellerSyncCoordinator` refresh |
| `CreateSaleScreen` | Commerce picker reuses list (`commerces/pick` route) |

Catalog preload runs on login via `container.requestSync()`.

### Products UI (Phase 59)

| Component | Purpose |
|-----------|---------|
| `ProductListScreen` | Cached active products + search; BRL price from minor units |
| `ProductDetailScreen` | Detail + live stock badge; optional Coil thumbnail |
| `ProductViewModel` | Room catalog flow + sync refresh |
| `ProductDetailViewModel` | `GET /products/{id}` + inventory balance on open |
| `MediaUrlCache` | Presigned URL cache via `GET /media/{id}/url` |

### Material Design 3 + Sales list (Phase 60)

| Component | Purpose |
|-----------|---------|
| `ui/theme/SellerTheme.kt` | M3 color scheme (dynamic on Android 12+), typography, shapes, `SellerSystemBarsEffect` |
| `SalesListScreen` | Merged remote + local sales, FAB, pull-to-refresh, empty states |
| `SalesListViewModel` | `GET /v1/sales` page 1 + Room merge via `mergeSalesList` |
| `SaleStatusChip` | M3 `AssistChip` for domain + sync statuses |
| `SellerEmptyState` | Reusable empty/offline state with `FilledTonalButton` CTA |

Merge logic lives in `shared/sales/SalesListMerger.kt` — remote status wins when deduped by remote id; pending sync rows sort to top.

### Create sale (Phase 61)

| Component | Purpose |
|-----------|---------|
| `CreateSaleScreen` | M3 form — commerce picker, payment chips, multi-line items, sticky total bar |
| `ProductSearchPicker` | Search by name/SKU; top 5 sellers from `GET /v1/products/top-selling` when search empty |
| `CreateSaleViewModel` | Catalog flows, stock lookup, top-selling fetch, draft persistence, validation, online/offline submit |
| `CreateSaleSubmitter` | `POST /v1/sales` with idempotency key, or `OfflineSaleWriter` + outbox |
| `shared/sales/CreateSaleForm.kt` | Total calculation (minor units), validation, ADR-006 payment methods |

Parity with field PWA `/sales/new`. Errors: `INSUFFICIENT_STOCK`, `VALIDATION_ERROR`, `COMMERCE_NOT_FOUND` via Snackbar.

### Sale detail (Phase 62)

| Component | Purpose |
|-----------|---------|
| `SaleDetailScreen` | Status chip, total, line items, commerce name, sync badge |
| `SaleDetailViewModel` | Load from API or Room; confirm/cancel with Snackbar errors |
| `SaleActionSubmitter` | Online `POST …/confirm|cancel`; offline outbox enqueue |
| `SaleDetailLoader` | Resolve local/remote id; prefer API when online |

Confirm/cancel only when status is `Pending` and sale has a remote id. Maps `INSUFFICIENT_STOCK`, `INVALID_SALE_TRANSITION`, `SALE_NOT_FOUND`.

### i18n (Phase 63)

| Component | Purpose |
|-----------|---------|
| `shared/i18n/SellerMessages.kt` | Typed message catalog (en + pt-BR), aligned with field PWA |
| `shared/i18n/SellerStrings.kt` | Locale resolver, status/payment/error formatters |
| `LocaleStore` | Android `SharedPreferences` persistence; default `pt-BR` |
| `LocaleViewModel` + `LocalSellerStrings` | Live locale without app restart |
| `LocaleSwitcher` | M3 `SegmentedButton` (EN / PT) in shell top bar + login screen |

Validation errors and API error codes resolve to localized strings in Compose. Shared submitters return stable error codes only.

### iOS + Compose Multiplatform (Phase 64)

| Component | Purpose |
|-----------|---------|
| `composeApp` | Shared Compose UI for Android and iOS |
| `MainViewController()` | iOS entry — `ComposeUIViewController` + `SellerRoot` |
| `IosAppContainer` | Keychain tokens, in-memory catalog/sales (online-first MVP) |
| `KeychainTokenStore` | iOS secure token storage in `shared/iosMain` |
| Ktor Darwin | `ktor-client-darwin` on `iosMain` (Phase 54 baseline) |

**Media upload UI:** skipped for MVP — `SellerApiClient.uploadMedia` client method + unit test cover the route; no image picker in seller shell.

iOS simulator API: `http://127.0.0.1:8080/v1`. See [iosApp/README.md](iosApp/README.md).

Spec: [docs/features/seller-mobile-app.md](../../../docs/features/seller-mobile-app.md) (Material 3 section).

**Updated:** 2026-07-22

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

Override via `SELLER_API_BASE_URL` in root `.env` when wiring CI or custom backends (see `.env.example`).

Create `local.properties` (gitignored):

```properties
sdk.dir=/path/to/Android/sdk
```

## Build

```bash
cd apps-mobile/seller
./gradlew :shared:check :androidApp:assembleDebug
./gradlew :shared:compileKotlinIosSimulatorArm64
```

From repo root:

```bash
pnpm mobile:seller:check
```

## Run

Open `apps-mobile/seller` in Android Studio → run **androidApp** on an emulator or device.

Placeholder screen: app name, platform greeting, and configured API base URL.

## Structure

| Module | Purpose |
|--------|---------|
| `shared` | KMP common code — API client, sync engine, repositories |
| `androidApp` | Jetpack Compose Activity + WorkManager sync |
| `iosApp/` | Stub — see `iosApp/README.md` |

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
| `ui/theme/SellerTheme.kt` | M3 color scheme (dynamic on Android 12+), typography, shapes |
| `SalesListScreen` | Merged remote + local sales, FAB, pull-to-refresh, empty states |
| `SalesListViewModel` | `GET /v1/sales` page 1 + Room merge via `mergeSalesList` |
| `SaleStatusChip` | M3 `AssistChip` for domain + sync statuses |
| `SellerEmptyState` | Reusable empty/offline state with `FilledTonalButton` CTA |

Merge logic lives in `shared/sales/SalesListMerger.kt` — remote status wins when deduped by remote id; pending sync rows sort to top.

Spec: `.local/phases/_reference/MATERIAL-3-UI.md`.

**Updated:** 2026-07-05

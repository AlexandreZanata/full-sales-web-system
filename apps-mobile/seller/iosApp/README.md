# iOS app — Full Sales Seller

Compose Multiplatform shell consuming the `ComposeApp` framework from `:composeApp`.

## Prerequisites

- macOS with Xcode 15+
- JDK 17
- Android Studio or IntelliJ with KMP plugin (optional)

## Build shared + Compose framework

From `apps-mobile/seller`:

```bash
./gradlew :shared:compileKotlinIosSimulatorArm64
./gradlew :composeApp:embedAndSignAppleFrameworkForXcode
```

On Linux CI, iOS targets are skipped (`kotlin.native.ignoreDisabledTargets=true`); use the macOS CI job (`seller-ios`).

## Run on simulator (macOS)

1. Start the API on the host: `pnpm dev:api` (listens on `127.0.0.1:8080`).
2. Open `iosApp/iosApp.xcodeproj` in Xcode.
3. Select an iPhone simulator → Run.
4. Login: `seller@test.com` / `secret123` (seed via `pnpm seed:dev`).

### API base URL

iOS simulator uses `http://127.0.0.1:8080/v1` from `shared/src/iosMain/.../ApiConfig.ios.kt`.

Physical device: replace with your machine LAN IP, e.g. `http://192.168.1.10:8080/v1`.

## Architecture

| Piece | Role |
|-------|------|
| `ComposeApp` framework | Shared Compose UI (`SellerNavHost`, M3 theme) |
| `IosAppContainer` | Keychain tokens + **SQLDelight** LocalStore (`seller.db`) |
| `MainViewController()` | Compose host + `UIApplicationDidBecomeActive` → sync resume |

### Offline durability (Phase 16G / OD-16-5=A)

- Catalog, sales (+ lines), registrations, outbox, stock/addresses, media URL cache, site settings — all in SQLDelight (schema matches Android Room inventory).
- Pending sale + outbox survive force-quit / process kill.
- Sync drain: `OnlineSyncTrigger` when Online is stable; `IosForegroundSync` on app become-active.
- **Limit:** no WorkManager / reliable background push while suspended — foreground + resume required.

## Media upload UI

**Skipped for MVP** — `SellerApiClient.uploadMedia` exists (Phase 54); product photo picker not required for seller shell.

**Updated:** 2026-07-14

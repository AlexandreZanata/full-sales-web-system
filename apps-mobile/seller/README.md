# Full Sales Seller — Kotlin Multiplatform (Phase 52)

Android shell + `shared` KMP module. Field sales UI and offline sync arrive in later phases.

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
| `shared` | KMP common code (`Greeting`, `expect/actual` platform + API config) |
| `androidApp` | Jetpack Compose Activity |
| `iosApp/` | Stub — see `iosApp/README.md` |

**Updated:** 2026-07-05

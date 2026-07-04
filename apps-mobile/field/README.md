# Full Sales Field — Kotlin Multiplatform (Phase 39E)

Android shell + `shared` KMP module. Offline sync and sales UI arrive in **Phase 39F**.

## Prerequisites

- JDK 17
- Android Studio Ladybug+ (or command-line SDK)
- Android SDK API 35, min SDK 26

## Local API URL (emulator)

Point the future Ktor client at:

```text
http://10.0.2.2:8080
```

(`10.0.2.2` is the host loopback from the Android emulator.)

Create `local.properties` (gitignored):

```properties
sdk.dir=/path/to/Android/sdk
```

## Build

```bash
cd apps-mobile/field
./gradlew :shared:check :androidApp:lint :androidApp:assembleDebug
```

## Run

Open `apps-mobile/field` in Android Studio → run **androidApp** on an emulator or device.

Placeholder screen: app name + “Field app — offline sync in Phase 39F”.

## Structure

| Module | Purpose |
|--------|---------|
| `shared` | KMP common code (`Greeting`, `expect/actual` platform hook) |
| `shared/.../api/` | Ktor client placeholder (39F) |
| `shared/.../sync/` | Sync engine placeholder (39F) |
| `androidApp` | Jetpack Compose Activity |
| `iosApp/` | Stub only — not built in CI v1 |

**Updated:** 2026-07-04

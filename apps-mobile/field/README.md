# Full Sales Field — Kotlin Multiplatform

Android shell + `shared` KMP module. Offline sync, sales list, and create-sale UI.

## Prerequisites

- JDK 17
- Android Studio Ladybug+ (or command-line SDK)
- Android SDK API 35, min SDK 26

## Local API URL

| Target | Base URL |
|--------|----------|
| Emulator (default) | `http://10.0.2.2:8080/v1` |
| Physical device (LAN) | `http://<host-lan-ip>:8080/v1` |

```bash
FIELD_API_BASE_URL=http://172.19.2.162:8080/v1 ./gradlew :androidApp:installDebug
# or local.properties: field.api.base.url=http://172.19.2.162:8080/v1
adb reverse tcp:8080 tcp:8080
```

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

```bash
adb shell am start -n com.fullsales.field/com.fullsales.field.android.MainActivity
```

Offline: banner on sales list; empty copy vs offline-empty; new sale uses Room catalog or empty-cache warning.

## Structure

| Module | Purpose |
|--------|---------|
| `shared` | API client, Room repos, sync, offline copy helpers |
| `androidApp` | Compose UI, WorkManager, EncryptedSharedPreferences |

**Updated:** 2026-07-16 (Phase 18)

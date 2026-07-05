# iOS app stub — Phase 52

Full Xcode project is not checked in yet. The `:shared` module compiles for iOS targets in CI.

## Prerequisites

- macOS with Xcode 15+
- JDK 17
- CocoaPods (when the native shell is added)

## Compile shared (CI / local)

From `apps-mobile/seller`:

```bash
./gradlew :shared:compileKotlinIosSimulatorArm64
./gradlew :shared:compileKotlinIosArm64
./gradlew :shared:compileKotlinIosX64
```

## Future Xcode shell

When iOS shipping is planned:

1. In Android Studio or IntelliJ, use **File → New → Kotlin Multiplatform App** or the [KMP wizard](https://kotlinlang.org/docs/multiplatform-create-first-app.html) as reference.
2. Point the iOS target at `:shared` (package `com.fullsales.seller`).
3. Add `iosApp/` Xcode project with a `ContentView` hosting shared Compose UI (when CMP iOS is adopted) or a SwiftUI placeholder.
4. Set API base URL via build configuration (simulator: `http://127.0.0.1:8080/v1`).
5. Wire `IosForegroundSync` on app resume (Phase 56 stub; full BGTask in Phase 66).

**Updated:** 2026-07-05

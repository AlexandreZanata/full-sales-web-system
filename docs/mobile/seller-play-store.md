# Seller Android — Play Store readiness

> English runbook. **No secrets** (keystore passwords, Play service-account JSON) belong in git.

## Locked decisions (Phase 21 defaults)

| ID | Decision |
|----|----------|
| OD-21-1 | Seller app only |
| OD-21-2 | `applicationId` = `com.fullsales.seller` |
| OD-21-3 | Release API = `https://api.vendas.comerc.app.br/v1` |
| OD-21-4 | First Internal: `versionCode=1`, `versionName=1.0.0` |
| OD-21-5 | R8 minify + shrink **on** for release |
| OD-21-6 | `android:allowBackup=false` (tokens on device) |
| OD-21-7 | Instrumented smoke **local** for v1 (not required on every CI job) |
| OD-21-8 | Privacy: `https://vendas.comerc.app.br/privacy-seller.html` |
| OD-21-9 | Store default language **pt-BR** |
| OD-21-10 | This phase targets **Internal testing** only |

## Inventory (current)

| Item | Value |
|------|-------|
| `applicationId` | `com.fullsales.seller` |
| Namespace | `com.fullsales.seller.android` |
| Store / label | Full Sales Seller (`android:label`) |
| Permissions (manifest) | `INTERNET`, `ACCESS_NETWORK_STATE` |
| Permissions (merged) | Plus WorkManager merges: `WAKE_LOCK`, `RECEIVE_BOOT_COMPLETED`, `FOREGROUND_SERVICE` (no ads ID) |
| `allowBackup` | `false` |
| Cleartext | Debug only (`src/debug`); release HTTPS-only |
| Icons | Adaptive + round mipmaps (Phase 18) |
| Min / target SDK | 26 / 35 |
| First store versions | `versionCode=1`, `versionName=1.0.0` |
| Preflight validated | 2026-07-22 — `pnpm mobile:seller:play-preflight` green (ephemeral upload key) |

## Versioning policy

| Field | Rule |
|-------|------|
| `versionCode` | Monotonic integer; bump every Play upload (CI build number or manual +1) |
| `versionName` | Semver (`MAJOR.MINOR.PATCH`) for humans |

First Internal upload: `1` / `1.0.0`.

## Release API

- Default release BuildConfig: `https://api.vendas.comerc.app.br/v1`
- Staging override (internal QA only):

```bash
SELLER_RELEASE_API_BASE_URL=https://staging-api.example/v1 ./gradlew :androidApp:bundleRelease
# or local.properties: seller.release.api.base.url=https://…
```

Debug keeps emulator/LAN defaults (`SELLER_API_BASE_URL` / `seller.api.base.url`).

## Signing (local)

See [seller-release-build.md](seller-release-build.md) and [apps-mobile/seller/README.md](../../apps-mobile/seller/README.md).

**Play upload:** use a durable upload keystore (backed up offline), never the preflight ephemeral `.preflight-upload.jks`.

```bash
cd apps-mobile/seller
# keystore.properties + upload-keystore.jks are gitignored
./gradlew :androidApp:bundleRelease
```

## Preflight gate (before any Play upload)

```bash
pnpm mobile:seller:play-preflight
# or: apps-mobile/seller/scripts/play-preflight.sh
```

Runs: secret refuse → `:shared:check` → lint → release compile → `bundleRelease`.

## Privacy / Data safety (summary)

| Topic | Answer for Console |
|-------|--------------------|
| App type | B2B seller field tool (not consumer social) |
| Data collected | Account email (login); auth tokens on device; commerce/product/sale cache |
| In transit | TLS (HTTPS) to API |
| At rest on device | EncryptedSharedPreferences for tokens; local SQLite cache |
| Backup | Disabled (`allowBackup=false`) |
| Deletion | Contact tenant admin / `privacy@comerc.app.br` (LGPD) |
| Privacy URL | `https://vendas.comerc.app.br/privacy-seller.html` |
| Terms URL | `https://vendas.comerc.app.br/terms-seller.html` |

## Play Console (operator)

1. Create app `com.fullsales.seller`, default language pt-BR.
2. Enroll Play App Signing; upload AAB from `bundleRelease`.
3. Complete listing, Data safety, content rating.
4. Internal testing → add license testers → install via opt-in link.
5. Production promotion is a **separate** PO gate (OD-21-10).

## CI release (optional)

Tag `seller-android-v*` → workflow builds AAB (needs GitHub Environment `play-internal` secrets) and uploads artifact. Never print signing passwords in logs.

## Secret hygiene

```bash
pnpm verify:no-android-secrets
# or: scripts/verify-no-android-secrets.sh
```

Refuses `*.jks`, `*.keystore`, `keystore.properties`, `play-service-account.json` if tracked/staged.

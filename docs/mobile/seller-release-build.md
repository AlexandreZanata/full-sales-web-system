# Seller Android — release build notes

> Secrets stay local / CI. Companion runbook: [seller-play-store.md](seller-play-store.md).

## Prerequisites

- JDK 17
- Android SDK (compile/target per `libs.versions`)
- Upload keystore (create once; backup offline)

## Create upload keystore (local only)

```bash
cd apps-mobile/seller
keytool -genkey -v \
  -keystore upload-keystore.jks \
  -keyalg RSA -keysize 2048 -validity 10000 \
  -alias upload
```

`keystore.properties` (gitignored):

```properties
storeFile=upload-keystore.jks
storePassword=***
keyAlias=upload
keyPassword=***
```

## Build AAB

```bash
cd apps-mobile/seller
./gradlew :androidApp:bundleRelease
# output: androidApp/build/outputs/bundle/release/*.aab
```

Release API defaults to `https://api.vendas.comerc.app.br/v1`. Override:

```bash
SELLER_RELEASE_API_BASE_URL=https://staging.example/v1 ./gradlew :androidApp:bundleRelease
```

Never ship `http://10.0.2.2:8080/v1` in release.

## Preflight (mandatory before Play)

```bash
cd apps-mobile/seller
./scripts/play-preflight.sh
# or from repo root:
pnpm mobile:seller:play-preflight
```

If `keystore.properties` is missing, preflight generates an **ephemeral** keystore (build proof only — not for Play upload).

## Open-repo ignores

```gitignore
*.jks
*.keystore
keystore.properties
play-service-account.json
.preflight-upload.jks
```

```bash
pnpm verify:no-android-secrets
```

## Mapping / crashes

Release enables R8 minify + shrink. Keep `androidApp/build/outputs/mapping/release/mapping.txt` when investigating Play crashes.

## CI tag flow

Tag `seller-android-v*` → workflow `.github/workflows/seller-play-internal.yml` runs preflight and uploads the AAB artifact (GitHub Environment `play-internal` secrets). Never print signing passwords in logs.

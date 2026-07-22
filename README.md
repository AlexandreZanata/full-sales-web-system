# Full Sales Web System

> Rust backend for driver/seller control — inventory, commerces, sales, and cryptographically signed reports.

**Repository:** [AlexandreZanata/full-sales-web-system](https://github.com/AlexandreZanata/full-sales-web-system)

---

## Golden rule

> **A simple interface is not a poor interface.** Perceived simplicity comes from a backend that absorbs complexity — strong validation, defined states, few dumb client endpoints.

---

## Documentation (construction base)

| Document | Description |
|----------|-------------|
| [PROJECT-OVERVIEW.md](docs/PROJECT-OVERVIEW.md) | Vision, bounded contexts, profiles |
| [TECH-STACK.md](docs/TECH-STACK.md) | Rust, Axum, Postgres, Redis, Ed25519 |
| [ARCHITECTURE.md](docs/ARCHITECTURE.md) | DDD + hexagonal, RLS multi-tenancy |
| [DOMAIN-MODEL.md](docs/DOMAIN-MODEL.md) | Aggregates per context |
| [GLOSSARY.md](docs/GLOSSARY.md) | Ubiquitous language |
| [BUSINESS-RULES.md](docs/BUSINESS-RULES.md) | GIVEN/WHEN/THEN rules |
| [STATE-MACHINES.md](docs/STATE-MACHINES.md) | Sale and lifecycle transitions |
| [API-CONTRACT.md](docs/API-CONTRACT.md) | HTTP `/v1/` endpoints |
| [openapi.yaml](docs/openapi.yaml) | OpenAPI 3.1 schema (Phase 3) |
| [TESTING-STRATEGY.md](docs/TESTING-STRATEGY.md) | TDD pyramid, CI gates |
| [SECURITY.md](docs/SECURITY.md) | Auth, RBAC, RLS |
| [ROADMAP.md](docs/ROADMAP.md) | Build phases 1–6 |
| [adr/](docs/adr/) | Architecture decisions (ADR-001..008) |
| [OPEN-DECISIONS.md](docs/OPEN-DECISIONS.md) | Decision log (all Phase 0 items resolved) |
| [NEW-PROJECT-CHECKLIST.md](docs/NEW-PROJECT-CHECKLIST.md) | Pre-code checklist — signed |
| [use-cases/](docs/use-cases/) | UC-001, UC-002 |

---

## Agent harness

| Path | Purpose |
|------|---------|
| [AGENTS.md](AGENTS.md) | Agent session entry point |
| `agent-rules/` | OWASP, TDD, architecture rules |
| `agent-harness/` | Resolve rules by task keywords |
| `.cursor/rules/` | Cursor always-on rules |
| `.local/phases/` | Local implementation tasks (gitignored) |

```bash
pip install -r agent-harness/requirements.txt
./agent-harness/resolve-rules.sh api auth domain rust sale
```

---

## Quick start

```bash
pnpm install
cp .env.example .env

# Full verify (TypeScript + Rust)
pnpm verify

# Git hooks (auto via pnpm install / prepare): verify changed code on commit + push
pnpm hooks:install
# Emergency skip: SKIP_VERIFY=1 git commit|push ...

# Dev: API (Rust) + web (Vite) in parallel
pnpm dev

# All web frontends only (Vite HMR, no Docker — API must be running separately)
./scripts/dev-frontends.sh

# API only
pnpm dev:api          # GET http://127.0.0.1:8080/health

# Web only
pnpm dev:web          # http://127.0.0.1:5173
```

Backend-only commands: [DEV-COMMANDS.md](docs/DEV-COMMANDS.md).

---

## CNPJ lookup (OpenCNPJ)

Full Sales proxies CNPJ lookups via `GET /v1/commerces/cnpj-lookup`. Upstream in production is **OpenCNPJ**.

| Item | Value |
|------|-------|
| OpenCNPJ base URL | `https://api.comerc.app.br` |
| Lookup path | `GET /api/v1/cnpj/{cnpj}` |
| Auth header | `X-API-Key: ocnpj_live_<your-key>` |
| Admin (create key) | https://admin.comerc.app.br |
| Cutover runbook | [docs/runbooks/opencnpj-cutover.md](docs/runbooks/opencnpj-cutover.md) |

**Never commit the live API key.** Store it only in `backend/.env` or the secret manager.

### Configure the Full Sales API

```bash
# backend/.env
CNPJ_LOOKUP_PROVIDER=opencnpj
CNPJ_LOOKUP_URL=https://api.comerc.app.br
CNPJ_LOOKUP_API_KEY=ocnpj_live_<your-32-hex-key>
```

### Call OpenCNPJ directly

```bash
export CNPJ_LOOKUP_API_KEY='ocnpj_live_<your-32-hex-key>'

curl -sS \
  -H "X-API-Key: ${CNPJ_LOOKUP_API_KEY}" \
  -H "X-Request-ID: $(uuidgen)" \
  "https://api.comerc.app.br/api/v1/cnpj/00000000000191"
```

### Call through Full Sales (seller/admin JWT)

```bash
# Local API: http://127.0.0.1:8080
curl -sS \
  -H "Authorization: Bearer ${ACCESS_TOKEN}" \
  "http://127.0.0.1:8080/v1/commerces/cnpj-lookup?cnpj=00000000000191"
```

Contract: [docs/API-CONTRACT.md](docs/API-CONTRACT.md) — `GET /v1/commerces/cnpj-lookup`.

---

## License

Third-party harness: [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

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
| [TESTING-STRATEGY.md](docs/TESTING-STRATEGY.md) | TDD pyramid, CI gates |
| [SECURITY.md](docs/SECURITY.md) | Auth, RBAC, RLS |
| [ROADMAP.md](docs/ROADMAP.md) | Build phases 1–6 |
| [adr/](docs/adr/) | Architecture decisions (ADR-001..007) |
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

**Phase 0 (current):** validate documentation before scaffold:

```bash
./scripts/validate-phase0-docs.sh
```

**After Phase 1 scaffold:** see [DEV-COMMANDS.md](docs/DEV-COMMANDS.md).

---

## License

Third-party harness: [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

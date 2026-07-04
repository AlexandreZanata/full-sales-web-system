# New Project Checklist

> Complete **before writing the first line of application code**.
> Mirrors `agent-rules/AGENT-CORE-PRINCIPLES.md` checklist.
> If any item is blank, the agent **must ask** — never assume.

**Spec base:** product README (driver/seller control system) → see [PROJECT-OVERVIEW.md](PROJECT-OVERVIEW.md).

---

## Architecture and domain

- [x] **Layers defined** — [ARCHITECTURE.md](ARCHITECTURE.md)
- [x] **Database modules** — [database/README.md](database/README.md) (Phase 1b entity + migration specs)
- [x] **Entities and aggregates** — [DOMAIN-MODEL.md](DOMAIN-MODEL.md)
- [x] **Value Objects** — Cnpj, Money, FullName, etc. in domain model
- [x] **Business rules** — [BUSINESS-RULES.md](BUSINESS-RULES.md)
- [x] **State machines** — [STATE-MACHINES.md](STATE-MACHINES.md)
- [x] **Access roles** — RBAC matrix in BUSINESS-RULES + use cases
- [x] **Domain events** — catalog in [DOMAIN-MODEL.md](DOMAIN-MODEL.md#domain-events-catalog)
- [x] **Use cases** — [use-cases/UC-001](use-cases/UC-001-register-and-confirm-sale.md), [UC-002](use-cases/UC-002-generate-signed-report.md)
- [x] **API contract** — [API-CONTRACT.md](API-CONTRACT.md)
- [x] **Glossary** — [GLOSSARY.md](GLOSSARY.md)

---

## Open decisions (must resolve before Phase 2+)

- [x] [OPEN-DECISIONS.md](OPEN-DECISIONS.md) — OD-001..OD-007 resolved in [adr/](adr/)

---

## Security (OWASP)

- [x] **OWASP Top 10:2025** — initial map in [SECURITY.md](SECURITY.md)
- [x] **Agentic 2026 (ASI01–ASI10)** — N/A unless AI tools in product flow

---

## Agent harness

- [x] **Harness installed** — `agent-rules/`, `agent-harness/`, `.cursor/rules/`
- [x] **AGENTS.md** — present
- [x] **Ponytail (static)** — `.cursor/rules/ponytail.mdc`

---

## Testing (contract-first)

- [x] **Policy documented** — [TESTING-STRATEGY.md](TESTING-STRATEGY.md)
- [x] **Unit tests** — `domain-shared` 100% coverage (Phase 1)
- [x] **Integration tests** — RLS isolation per module (`infra-postgres`, Phase 1b)
- [x] **E2E scenarios** — defined in TESTING-STRATEGY (E2E-001..004)
- [x] **CI** — `.github/workflows/ci.yml` (Phase 1)
- [x] **No mirror tests** — policy in TESTING-STRATEGY
- [x] **Phase 0 doc validation** — `scripts/validate-phase0-docs.sh`

---

## Sign-off

| Role | Name | Date |
|------|------|------|
| Product / domain | Product Owner *(placeholder — update before production)* | 2026-07-04 |
| Tech lead | Tech Lead *(placeholder — update before production)* | 2026-07-04 |

Phase 1 implementation may begin. Replace placeholder names with real signatories before production release.

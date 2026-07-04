# Project Overview — Driver/Seller Control System

> Rust backend for inventory, commerce registration, sales, and cryptographically signed reports — focused on **ease of use**, **security**, and **reliability**.

**Source spec:** internal product README (driver/seller control system).

---

## Golden rule

> **A simple interface is not a poor interface.** Perceived simplicity for the end user comes from a backend that absorbs complexity internally — strong validation, well-defined states, and few "dumb" endpoints on the client.

---

## End-user profiles

| Profile | Primary actions |
|---------|-----------------|
| **Admin** | Manage users, register commerces, view reports, verify signatures |
| **Driver / Seller** | Record sales, check stock, operate in the field |

Business complexity (inventory, billing, traceability) lives entirely in the backend.

---

## Bounded contexts

| Context | Responsibility |
|---------|----------------|
| **Identity & Access** | Users, roles (admin, driver, seller), authentication, sessions |
| **Commerces (Tenants)** | Business client registration (CNPJ, legal name, address, contacts) |
| **Inventory** | Products, batches, inbound/outbound movements, balance per driver/vehicle |
| **Sales** | Orders, sold items, payment method, driver → commerce → sale linkage |
| **Reports & Signature** | Report generation (daily/period), Ed25519 signature, verification |

Each context is a **crate** in the Rust Cargo workspace — clear boundaries, no "domain soup".

---

## Documentation map

| Document | Purpose |
|----------|---------|
| [TECH-STACK.md](TECH-STACK.md) | Language, frameworks, infrastructure |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Layers, hexagonal ports, multi-tenancy |
| [DOMAIN-MODEL.md](DOMAIN-MODEL.md) | Aggregates and entities per context |
| [GLOSSARY.md](GLOSSARY.md) | Ubiquitous language |
| [BUSINESS-RULES.md](BUSINESS-RULES.md) | GIVEN/WHEN/THEN rules |
| [STATE-MACHINES.md](STATE-MACHINES.md) | Valid transitions |
| [API-CONTRACT.md](API-CONTRACT.md) | HTTP `/v1/` contract |
| [TESTING-STRATEGY.md](TESTING-STRATEGY.md) | TDD pyramid and CI gates |
| [SECURITY.md](SECURITY.md) | Auth, RBAC, RLS, PII |
| [CACHING-STRATEGY.md](CACHING-STRATEGY.md) | Redis usage |
| [DIGITAL-SIGNATURE.md](DIGITAL-SIGNATURE.md) | Ed25519 vs ICP-Brasil |
| [ROADMAP.md](ROADMAP.md) | Build phases |
| [OPEN-DECISIONS.md](OPEN-DECISIONS.md) | Pending ADRs |
| [use-cases/](use-cases/) | Actor flows |

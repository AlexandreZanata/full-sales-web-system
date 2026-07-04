# Domain Model

Per bounded context. Code names in **English**; see [GLOSSARY.md](GLOSSARY.md).

---

## Identity & Access

**Aggregate root:** `User`

```rust
pub struct User {
    id: UserId,              // UUIDv7
    name: FullName,          // Value Object
    role: Role,              // Admin | Driver | Seller
    password_hash: PasswordHash,
    active: bool,
    tenant_id: TenantId,
}

pub enum Role {
    Admin,
    Driver,
    Seller,
}
```

- `FullName`, `Email`, `PasswordHash` are Value Objects — validate at construction.
- Login issues **JWT access token** (15 min) + **opaque refresh token** in Redis (revocable logout).

---

## Commerces

**Aggregate root:** `Commerce`

```rust
pub struct Commerce {
    id: CommerceId,
    cnpj: Cnpj,              // VO: check digit at creation
    legal_name: String,
    trade_name: Option<String>,
    address: Address,
    contact: Contact,
    active: bool,
    tenant_id: TenantId,
}
```

- Commerce registration is **admin-only** (application rule, not UI-only).
- `Cnpj` rejects invalid values before persistence.

---

## Inventory

**Aggregate root:** `Inventory` (per driver — see [ADR-005](adr/ADR-005-inventory-driver-scope.md))

```rust
pub struct Product {
    id: ProductId,
    name: String,
    sku: Sku,
    price: Money,            // integer minor units — never f64
    tenant_id: TenantId,
}

pub struct StockMovement {
    id: MovementId,
    product_id: ProductId,
    movement_type: MovementType,  // Inbound | SaleOutbound | Adjustment | Return
    quantity: u32,
    responsible_id: UserId,
    created_at: DateTime<Utc>,
}
```

- Stock balance uses a **materialized projection** in Postgres plus Redis cache (ADR-001). Every confirmed sale creates a `SaleOutbound` movement in the **same transaction** as the sale.

---

## Sales

**Aggregate root:** `Sale`

```rust
pub struct Sale {
    id: SaleId,              // UUIDv7 — time-sortable
    driver_id: UserId,
    commerce_id: CommerceId,
    items: Vec<SaleItem>,
    payment_method: PaymentMethod,
    status: SaleStatus,      // Pending | Confirmed | Cancelled
    total: Money,            // always recalculated from items
    created_at: DateTime<Utc>,
    tenant_id: TenantId,
}
```

**Invariants:**

- Cannot add empty items.
- Cannot confirm sale without items.
- `total` is never set from client input — always computed from line items.

---

## Reports & Signature

**Aggregate root:** `Report`

```rust
pub struct Report {
    id: ReportId,
    report_type: ReportType,       // DailyDriver | CommercePeriod | Consolidated
    period_start: DateTime<Utc>,
    period_end: DateTime<Utc>,
    canonical_payload: String,     // canonical JSON — signed bytes
    signature: Ed25519Signature,
    public_key_id: String,         // key rotation support
    generated_at: DateTime<Utc>,
}
```

**Generation flow:**

1. Application assembles report data (sales, totals, driver, commerce).
2. Serialize to **canonical JSON** (deterministic key order, no superfluous whitespace).
3. Hash payload (SHA-256) and sign with server Ed25519 private key.
4. Persist `canonical_payload` + `signature` + `public_key_id`.
5. Verification endpoint recalculates hash and validates signature — any post-generation tampering fails verification.

See [DIGITAL-SIGNATURE.md](DIGITAL-SIGNATURE.md).

---

## Domain events catalog

Events are past-tense facts raised by aggregates (see use cases for triggers):

| Event | Context | Raised when |
|-------|---------|-------------|
| `UserCreated` | Identity | Admin registers user |
| `UserDeactivated` | Identity | Admin deactivates user |
| `CommerceCreated` | Commerces | Admin registers commerce |
| `CommerceDeactivated` | Commerces | Admin deactivates commerce |
| `ProductCreated` | Inventory | Admin creates product |
| `StockMovementRecorded` | Inventory | Movement persisted (inbound, sale outbound, adjustment, return) |
| `SaleCreated` | Sales | Sale saved in `Pending` status |
| `SaleConfirmed` | Sales | Sale transitions to `Confirmed` |
| `SaleCancelled` | Sales | Pending sale voided |
| `ReportGenerated` | Reports | Report signed and persisted |

Full event payloads and handlers are defined during Phase 2+ implementation.

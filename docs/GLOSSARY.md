# Domain Glossary

> Ubiquitous language. Code, APIs, docs, and agents MUST use these terms exactly.

---

## User

**Definition:** System account with authentication credentials and a assigned role.
**Not the same as:** Commerce (business client record)
**Code name:** `User`

---

## Role

**Definition:** Authorization profile for a User.
**Enum values:** `Admin`, `Driver`, `Seller`
**Code name:** `Role`

| Role | Typical permissions |
|------|---------------------|
| Admin | Manage users, commerces, reports |
| Driver | Record sales, view assigned stock |
| Seller | Record sales (field seller variant) |

---

## TenantId

**Definition:** UUID identifying the **platform owner organization** for multi-tenancy and RLS. All users and commerces belong to one tenant (ADR-002).
**Not the same as:** CommerceId (a business client within the tenant)
**Code name:** `TenantId`

---

## Commerce

**Definition:** Registered business client (store/company) where sales occur — identified by CNPJ.
**Not the same as:** User, Tenant
**Code name:** `Commerce`

---

## Cnpj

**Definition:** Brazilian company tax identifier Value Object with check-digit validation.
**Code name:** `Cnpj`

---

## Product

**Definition:** Sellable SKU with name, identifier, and unit price.
**Code name:** `Product`

---

## Sku

**Definition:** Stock-keeping unit identifier for a Product.
**Code name:** `Sku`

---

## Money

**Definition:** Monetary amount in minor units (e.g. centavos) with currency code — never floating point.
**Code name:** `Money`

---

## StockMovement

**Definition:** Immutable record of inventory change (inbound, sale outbound, adjustment, return).
**Enum values (`MovementType`):** `Inbound`, `SaleOutbound`, `Adjustment`, `Return`
**Code name:** `StockMovement`

---

## Inventory

**Definition:** Aggregate representing stock balance **per driver** per product (ADR-005). Movements are keyed by `responsible_id` (the driver).
**Code name:** `Inventory`

---

## Sale

**Definition:** Commercial transaction linking driver, commerce, line items, payment method, and total.
**Not the same as:** StockMovement (sale triggers outbound movement)
**State values:** `Pending`, `Confirmed`, `Cancelled`
**Code name:** `Sale`

---

## SaleItem

**Definition:** Single product line on a Sale — quantity, unit price, line subtotal.
**Code name:** `SaleItem`

---

## PaymentMethod

**Definition:** How the sale was paid — recorded only, no gateway capture in MVP (ADR-006).
**Enum values:** `Cash`, `Pix`, `Credit`, `Debit`
**API strings:** `"cash"`, `"pix"`, `"credit"`, `"debit"`
**Code name:** `PaymentMethod`

---

## Report

**Definition:** Period summary of sales/activity with canonical payload and Ed25519 signature.
**Enum values (`ReportType`):** `DailyDriver`, `CommercePeriod`, `Consolidated`
**Code name:** `Report`

---

## Ed25519Signature

**Definition:** Cryptographic signature over report canonical payload — integrity and origin proof.
**Not the same as:** ICP-Brasil qualified digital signature (see [DIGITAL-SIGNATURE.md](DIGITAL-SIGNATURE.md))
**Code name:** `Ed25519Signature`

---

## Driver

**Definition:** User with role `Driver` who performs field sales and stock operations.
**Code name:** `Driver` (role context; entity is `User`)

---

## Admin

**Definition:** User with role `Admin` — full tenant management except crypto key access (infra-only).
**Code name:** `Admin` (role context)

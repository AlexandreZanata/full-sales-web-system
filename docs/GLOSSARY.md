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
**Enum values:** `Admin`, `Driver`, `Seller`, `CommerceContact`
**Code name:** `Role`

| Role | Typical permissions |
|------|---------------------|
| Admin | Manage users, commerces, reports; approve orders |
| Driver | Record sales, view assigned stock; confirm deliveries |
| Seller | Record sales (field seller variant); create orders on visit |
| CommerceContact | Commerce portal — create/view orders for own commerce |

---

## TenantId

**Definition:** UUID identifying the **platform owner organization** for multi-tenancy and RLS. All users and commerces belong to one tenant (ADR-002).
**Not the same as:** CommerceId (a business client within the tenant)
**Code name:** `TenantId`

---

## SiteSettings

**Definition:** Tenant-level branding configuration — public display name, logo, and optional sales contact phone (WhatsApp) shown in admin, portal, and field app shells.
**Not the same as:** Commerce logo (`logo_file_id` on a Commerce row — per-store branding)
**Storage:** `shared.tenants.display_name`, `shared.tenants.logo_file_id`, `shared.tenants.sales_contact_phone` → `media.files`
**Code name:** `SiteSettings`

---

## Commerce

**Definition:** Registered business client (store/company) where sales occur — identified by CNPJ.
**Not the same as:** User, Tenant
**Code name:** `Commerce`

---

## CommerceAddress

**Definition:** Normalized billing or delivery address for a Commerce — replaces legacy JSON `address` on the commerce row.
**Enum values (`AddressType`):** `Billing`, `Delivery`
**Invariant:** At most one `is_primary = true` per (commerce, address_type).
**Code name:** `CommerceAddress`

---

## AddressType

**Definition:** Classification of a CommerceAddress — billing (invoicing) or delivery (order fulfillment).
**Enum values:** `Billing`, `Delivery`
**Code name:** `AddressType`

---

## PostalCode

**Definition:** Brazilian postal code (CEP) Value Object — eight digits, normalized without punctuation.
**Code name:** `PostalCode`

---

## BrazilianState

**Definition:** Brazilian federative unit (UF) Value Object — two-letter code from the official 27 states.
**Code name:** `BrazilianState`

---

## Cnpj

**Definition:** Brazilian company tax identifier Value Object with check-digit validation.
**Code name:** `Cnpj`

---

## Product

**Definition:** Sellable SKU with name, identifier, unit price, optional category assignment (`categoryId`), unit of measure, and optional portal description (max 2000 chars).
**Code name:** `Product`

---

## ProductCategory

**Definition:** Tenant-scoped catalog grouping for products — name, URL slug, sort order, optional image, active flag.
**Code name:** `ProductCategory`  
**Table:** `inventory.product_categories`

---

## CatalogViewMode

**Definition:** Portal catalog product layout preference — list (horizontal row) or grid (vertical card).
**Enum values:** `list`, `grid`
**Code name:** `CatalogViewMode`  
**Persistence:** Browser `localStorage` key `portal.catalog.viewMode`

---

## UnitOfMeasure

**Definition:** Catalog unit for a Product — how quantity is expressed on orders and in stock.
**Enum values:** `Unit`, `Kg`, `Box`, `Liter`
**Code name:** `UnitOfMeasure`

---

## ProductImage

**Definition:** Gallery image linked to a Product — references `media.files`; one primary per product.
**Code name:** `ProductImage`

---

## ReservationStatus

**Definition:** Lifecycle state of a `StockReservation`.
**Enum values:** `Active`, `Released`, `Consumed`
**Code name:** `ReservationStatus`

---

## Sku

**Definition:** Stock-keeping unit identifier for a Product.
**Code name:** `Sku`

---

## Money

**Definition:** Monetary amount in minor units (e.g. centavos) with currency code — never floating point.
**Code name:** `Money`, `Currency`

---

## SaleStatus

**Definition:** Lifecycle state of a `Sale` aggregate.
**Enum values:** `Pending`, `Confirmed`, `Cancelled`
**Code name:** `SaleStatus`

---

## FullName

**Definition:** User display name Value Object — minimum two parts, validated at construction.
**Code name:** `FullName`

---

## Email

**Definition:** Validated email address Value Object for user identity fields.
**Code name:** `Email`

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

## Order

**Definition:** Commerce or seller **intent** to buy — approval and fulfillment lifecycle before stock is deducted.
**Not the same as:** Sale (Order is intent; Sale is the delivery fact)
**State values:** `Draft`, `PendingApproval`, `Approved`, `Picking`, `InTransit`, `Delivered`, `PartiallyDelivered`, `Rejected`, `Cancelled`
**Code name:** `Order`

---

## OrderItem

**Definition:** Single product line on an Order — requested quantity, optional delivered quantity, unit price **frozen at order creation** (RN3).
**Code name:** `OrderItem`

---

## Delivery

**Definition:** Fulfillment step for an Order — assigned driver, proof photo, geo, received-by name.
**State values:** `Waiting`, `InTransit`, `Delivered`, `Failed`
**Cardinality:** One Delivery per Order in v1 (DE-004)
**Code name:** `Delivery`

---

## StockReservation

**Definition:** Quantity held against tenant available stock when an Order is approved — not yet deducted from driver balance.
**Enum values (`ReservationStatus`):** `Active`, `Released`, `Consumed`
**Scope:** Tenant pool until driver assigned; consumed on delivery confirm (ADR-010)
**Code name:** `StockReservation`

---

## CommerceContact

**Definition:** User with role `CommerceContact` — portal login scoped to a single `commerce_id` via RLS.
**Not the same as:** Commerce (the business client record)
**Code name:** `CommerceContact` (role context; entity is `User`)

---

## DeclaredPayment

**Definition:** Seller or driver **assertion** of payment received for a Sale — not verified by the platform (RN-PAG1–RN-PAG4).
**Fields on Sale:** `declared_payment_method`, `declared_payment_received`, `declared_payment_at`, `declared_payment_by_user_id`, `declared_payment_notes`
**Not the same as:** `PaymentMethod` (expected method at sale creation, ADR-006)
**Code name:** `DeclaredPayment`

---

## Sale

**Definition:** Commercial transaction linking driver, commerce, line items, payment method, and total — the **fact** of what was delivered.
**Not the same as:** Order (field sales have `order_id = NULL`; portal sales link to Order after delivery)
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

---

## PlatformAdmin

**Definition:** Platform operator with cross-tenant powers — tenant CRUD, billing override, fraud review, impersonation, system health. Distinct from tenant `Admin` (ADR-013).
**Not the same as:** `Admin`, `SuperAdmin`
**Storage:** `identity.platform_users`
**Code name:** `PlatformAdmin`

---

## TenantStatus

**Definition:** Lifecycle state of a tenant organization on the platform.
**Enum values:** `Provisioning`, `Trial`, `Active`, `PastDue`, `Suspended`, `Offboarding`, `Deleted`
**Code name:** `TenantStatus`

---

## SubscriptionPlan

**Definition:** Named SaaS tier with price, limits, and feature flags (`Starter`, `Pro`, `Enterprise`).
**Storage:** `billing.plans`
**Code name:** `SubscriptionPlan`

---

## TenantPaymentSettings

**Definition:** Tenant Admin configuration for collecting portal payments via the tenant's own Asaas account (Pro+ only, ADR-018).
**Fields:** `enabled`, method toggles (PIX, credit, boleto), `autoCapture`
**Storage:** `billing.tenant_payment_settings`
**Code name:** `TenantPaymentSettings`

---

## TenantAsaasCredentials

**Definition:** Encrypted tenant Asaas API key (AES-256-GCM) for portal payment collection — separate from platform `ASAAS_API_KEY`.
**Storage:** `billing.tenant_asaas_credentials`
**Code name:** `TenantAsaasCredentials`

---

## AsaasCustomer

**Definition:** Tenant's customer record on the **platform** Asaas account — used for SaaS subscription billing (`externalReference` = `TenantId`).
**Not the same as:** Tenant's own Asaas account for portal payments (ADR-018)
**Code name:** `AsaasCustomer`

---

## TenantDomain

**Definition:** Custom hostname attached to a tenant for portal/admin UI routing.
**Enum values (`DomainStatus`):** `Pending`, `Verifying`, `Verified`, `Active`, `Failed`, `Detached`
**Code name:** `TenantDomain`

---

## ImpersonationGrant

**Definition:** Short-lived, audited authorization for PlatformAdmin to act as a tenant `Admin` with scoped JWT.
**TTL:** 15 minutes (ADR-013)
**Code name:** `ImpersonationGrant`

---

## FraudEvent

**Definition:** Recorded signal of suspected abuse — velocity breach, blocklist hit, cross-tenant probe, etc.
**Storage:** `fraud.fraud_events`
**Code name:** `FraudEvent`

---

## PaymentEvent

**Definition:** Idempotent log entry for an inbound Asaas webhook — `asaas_event_id` UNIQUE (ADR-014).
**Storage:** `billing.payment_events`
**Code name:** `PaymentEvent`

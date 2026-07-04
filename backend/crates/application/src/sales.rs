use chrono::{DateTime, Utc};
use domain_commerces::Commerce;
use domain_identity::UserId;
use domain_inventory::Product;
use domain_sales::{
    DeclarePaymentInput, DeclaredPaymentMethod, PaymentDeclarationAuditPort, PaymentMethod, Sale, SaleId, SaleStatus,
};
use domain_shared::{Money, TenantId};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum SalesAppError {
    #[error(transparent)]
    Sale(#[from] domain_sales::SaleError),

    #[error(transparent)]
    Inventory(#[from] domain_inventory::InventoryError),

    #[error(transparent)]
    Commerce(#[from] domain_commerces::CommerceError),

    #[error("sale not found")]
    SaleNotFound,

    #[error("commerce not found")]
    CommerceNotFound,

    #[error("product not found")]
    ProductNotFound,

    #[error("insufficient stock")]
    InsufficientStock,
}

pub struct CreateSaleLineInput {
    pub product_id: Uuid,
    pub quantity: i32,
}

pub struct CreateSaleCommand {
    pub sale_id: SaleId,
    pub driver_id: UserId,
    pub commerce_id: Uuid,
    pub payment_method: PaymentMethod,
    pub tenant_id: TenantId,
    pub items: Vec<CreateSaleLineInput>,
}

pub struct SaleDto {
    pub id: SaleId,
    pub driver_id: UserId,
    pub commerce_id: Uuid,
    pub status: SaleStatus,
    pub payment_method: PaymentMethod,
    pub total: Money,
    pub items: Vec<SaleItemDto>,
}

pub struct SaleItemDto {
    pub product_id: Uuid,
    pub quantity: i32,
    pub unit_price_amount: i64,
    pub unit_price_currency: String,
    pub line_total_amount: i64,
}

pub fn create_sale(
    commerce: Commerce,
    products: Vec<Product>,
    command: CreateSaleCommand,
) -> Result<Sale, SalesAppError> {
    let mut sale = Sale::create(domain_sales::SaleCreateInput {
        id: command.sale_id,
        driver_id: command.driver_id,
        commerce,
        payment_method: command.payment_method,
        tenant_id: command.tenant_id,
    })?;

    for line in command.items {
        let product = products
            .iter()
            .find(|p| p.id().as_uuid() == line.product_id)
            .ok_or(SalesAppError::ProductNotFound)?;
        let quantity = domain_inventory::Quantity::of(line.quantity)?;
        sale = sale.add_item(domain_sales::AddSaleItemInput {
            product: product.clone(),
            quantity,
        })?;
    }

    Ok(sale)
}

pub fn sale_to_dto(sale: &Sale) -> Result<SaleDto, domain_shared::DomainError> {
    let total = sale.total()?;
    Ok(SaleDto {
        id: sale.id(),
        driver_id: sale.driver_id(),
        commerce_id: sale.commerce_id().as_uuid(),
        status: sale.status(),
        payment_method: sale.payment_method(),
        total,
        items: sale
            .items()
            .iter()
            .map(|item| SaleItemDto {
                product_id: item.product_id().as_uuid(),
                quantity: item.quantity().value(),
                unit_price_amount: item.unit_price().amount_minor(),
                unit_price_currency: item.unit_price().currency().as_str().to_owned(),
                line_total_amount: item.line_total().amount_minor(),
            })
            .collect(),
    })
}

pub fn confirm_sale(sale: Sale) -> Result<Sale, SalesAppError> {
    sale.confirm().map_err(SalesAppError::from)
}

pub struct DeclareSalePaymentCommand {
    pub method: DeclaredPaymentMethod,
    pub received: bool,
    pub declared_at: DateTime<Utc>,
    pub declaring_user: UserId,
    pub notes: Option<String>,
}

pub fn declare_sale_payment(
    sale: Sale,
    command: DeclareSalePaymentCommand,
    audit: &mut impl PaymentDeclarationAuditPort,
) -> Result<Sale, SalesAppError> {
    sale.declare_payment(
        DeclarePaymentInput {
            method: command.method,
            received: command.received,
            declared_at: command.declared_at,
            declaring_user: command.declaring_user,
            notes: command.notes,
        },
        audit,
    )
    .map_err(SalesAppError::from)
}

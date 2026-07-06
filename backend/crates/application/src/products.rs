use domain_inventory::{Product, ProductId, Sku};
use domain_shared::{Currency, Money, TenantId};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ProductsAppError {
    #[error(transparent)]
    Inventory(#[from] domain_inventory::InventoryError),

    #[error(transparent)]
    Domain(#[from] domain_shared::DomainError),
}

pub struct ProductDto {
    pub id: Uuid,
    pub name: String,
    pub sku: String,
    pub price_amount: i64,
    pub price_currency: String,
    pub active: bool,
}

pub struct PaginatedProducts {
    pub items: Vec<ProductDto>,
    pub page: u32,
    pub page_size: u32,
    pub total: u64,
}

pub fn product_to_dto(product: &Product) -> ProductDto {
    ProductDto {
        id: product.id().as_uuid(),
        name: product.name().to_owned(),
        sku: product.sku().as_str().to_owned(),
        price_amount: product.unit_price().amount_minor(),
        price_currency: product.unit_price().currency().as_str().to_owned(),
        active: product.is_active(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn restore_product(
    id: Uuid,
    name: &str,
    sku: &str,
    price_amount: i64,
    price_currency: &str,
    tenant_id: TenantId,
    active: bool,
    category: Option<&str>,
    unit_of_measure: &str,
) -> Result<Product, ProductsAppError> {
    Ok(Product::create(domain_inventory::ProductCreateInput {
        id: ProductId::from_uuid(id),
        name: name.to_owned(),
        sku: Sku::parse(sku)?,
        unit_price: Money::new(price_amount, Currency::parse(price_currency)?)?,
        tenant_id,
        active,
        category: category.map(str::to_owned),
        unit_of_measure: unit_of_measure.parse()?,
    }))
}

pub fn paginate_products(products: Vec<Product>, page: u32, page_size: u32) -> PaginatedProducts {
    let total = products.len() as u64;
    let start = ((page.saturating_sub(1)) as u64 * page_size as u64) as usize;
    let end = start.saturating_add(page_size as usize).min(products.len());
    let slice = if start < products.len() {
        &products[start..end]
    } else {
        &[]
    };
    PaginatedProducts {
        items: slice.iter().map(product_to_dto).collect(),
        page,
        page_size,
        total,
    }
}

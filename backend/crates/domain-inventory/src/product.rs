use domain_shared::{Money, TenantId};

use crate::product_id::ProductId;
use crate::sku::Sku;
use crate::unit_of_measure::UnitOfMeasure;

pub struct ProductCreateInput {
    pub id: ProductId,
    pub name: String,
    pub sku: Sku,
    pub unit_price: Money,
    pub tenant_id: TenantId,
    pub active: bool,
    pub category: Option<String>,
    pub unit_of_measure: UnitOfMeasure,
}

/// Sellable SKU with name, identifier, and unit price.
#[derive(Debug, Clone)]
pub struct Product {
    id: ProductId,
    name: String,
    sku: Sku,
    unit_price: Money,
    tenant_id: TenantId,
    active: bool,
    category: Option<String>,
    unit_of_measure: UnitOfMeasure,
}

impl Product {
    pub fn create(input: ProductCreateInput) -> Self {
        Self {
            id: input.id,
            name: input.name.trim().to_owned(),
            sku: input.sku,
            unit_price: input.unit_price,
            tenant_id: input.tenant_id,
            active: input.active,
            category: input
                .category
                .map(|c| c.trim().to_owned())
                .filter(|c| !c.is_empty()),
            unit_of_measure: input.unit_of_measure,
        }
    }

    pub fn id(&self) -> ProductId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn sku(&self) -> &Sku {
        &self.sku
    }

    pub fn unit_price(&self) -> &Money {
        &self.unit_price
    }

    pub fn tenant_id(&self) -> TenantId {
        self.tenant_id
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn category(&self) -> Option<&str> {
        self.category.as_deref()
    }

    pub fn unit_of_measure(&self) -> UnitOfMeasure {
        self.unit_of_measure
    }
}

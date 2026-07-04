use domain_shared::TenantId;
use uuid::Uuid;

use crate::error::InventoryError;
use crate::product_id::ProductId;
use crate::product_image_id::ProductImageId;

pub struct CreateProductImageInput {
    pub id: ProductImageId,
    pub tenant_id: TenantId,
    pub product_id: ProductId,
    pub file_id: Uuid,
    pub sort_order: i32,
    pub is_primary: bool,
}

/// Gallery image linked to a Product — one primary per product.
#[derive(Debug, Clone)]
pub struct ProductImage {
    id: ProductImageId,
    tenant_id: TenantId,
    product_id: ProductId,
    file_id: Uuid,
    sort_order: i32,
    is_primary: bool,
}

impl ProductImage {
    pub fn create(
        input: CreateProductImageInput,
        existing: &[ProductImage],
    ) -> Result<Self, InventoryError> {
        if input.is_primary && has_primary(existing, input.product_id) {
            return Err(InventoryError::DuplicatePrimaryImage);
        }

        Ok(Self {
            id: input.id,
            tenant_id: input.tenant_id,
            product_id: input.product_id,
            file_id: input.file_id,
            sort_order: input.sort_order,
            is_primary: input.is_primary,
        })
    }

    pub fn id(&self) -> ProductImageId {
        self.id
    }

    pub fn tenant_id(&self) -> TenantId {
        self.tenant_id
    }

    pub fn product_id(&self) -> ProductId {
        self.product_id
    }

    pub fn file_id(&self) -> Uuid {
        self.file_id
    }

    pub fn sort_order(&self) -> i32 {
        self.sort_order
    }

    pub fn is_primary(&self) -> bool {
        self.is_primary
    }
}

fn has_primary(existing: &[ProductImage], product_id: ProductId) -> bool {
    existing
        .iter()
        .any(|img| img.product_id() == product_id && img.is_primary())
}

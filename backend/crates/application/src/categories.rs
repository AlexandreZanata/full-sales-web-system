use domain_inventory::{ProductCategory, ProductCategoryCreateInput};
use domain_shared::TenantId;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CategoriesAppError {
    #[error(transparent)]
    Inventory(#[from] domain_inventory::InventoryError),

    #[error("category slug already exists")]
    SlugConflict,

    #[error("category not found")]
    NotFound,
}

#[allow(clippy::too_many_arguments)]
pub fn create_category(
    id: Uuid,
    tenant_id: TenantId,
    name: &str,
    slug: Option<&str>,
    description: Option<&str>,
    sort_order: i32,
    active: bool,
    existing_slugs: Vec<String>,
) -> Result<ProductCategory, CategoriesAppError> {
    ProductCategory::create(ProductCategoryCreateInput {
        id,
        tenant_id,
        name: name.to_owned(),
        slug: slug.map(str::to_owned),
        description: description.map(str::to_owned),
        sort_order,
        active,
        existing_slugs,
    })
    .map_err(CategoriesAppError::Inventory)
}

pub fn rename_category(
    category: &mut ProductCategory,
    name: &str,
    regenerate_slug: bool,
    existing_slugs: &[String],
) -> Result<(), CategoriesAppError> {
    category
        .rename(name, regenerate_slug, existing_slugs)
        .map_err(CategoriesAppError::Inventory)
}

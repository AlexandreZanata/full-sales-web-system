use axum::{
    Json,
    extract::{Query, State},
};
use domain_identity::Role;
use serde::{Deserialize, Serialize};

use crate::auth::{AuthUser, require_roles};
use crate::error::ApiError;
use crate::pagination::paginate_offset;
use crate::state::AppState;

pub mod catalog;
pub mod catalog_images;
pub mod top_selling;

#[derive(Serialize)]
pub struct ProductResponse {
    pub id: uuid::Uuid,
    pub name: String,
    pub sku: String,
    #[serde(rename = "priceAmount")]
    pub price_amount: i64,
    #[serde(rename = "priceCurrency")]
    pub price_currency: String,
    pub active: bool,
    #[serde(rename = "categoryId", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<uuid::Uuid>,
    #[serde(rename = "categoryName", skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    #[serde(rename = "categorySlug", skip_serializing_if = "Option::is_none")]
    pub category_slug: Option<String>,
}

#[derive(Serialize)]
pub struct PaginatedProductsResponse {
    pub items: Vec<ProductResponse>,
    pub page: u32,
    #[serde(rename = "pageSize")]
    pub page_size: u32,
    pub total: u64,
}

pub(crate) fn product_response_from_row(
    row: &infra_postgres::inventory::ProductRow,
) -> ProductResponse {
    ProductResponse {
        id: row.id,
        name: row.name.clone(),
        sku: row.sku.clone(),
        price_amount: row.price_amount,
        price_currency: row.price_currency.clone(),
        active: row.active,
        category_id: row.category_id,
        category_name: row.category_name.clone(),
        category_slug: row.category_slug.clone(),
    }
}

#[derive(Deserialize)]
pub struct ListProductsQuery {
    #[serde(default = "crate::pagination::default_page")]
    pub page: u32,
    #[serde(rename = "pageSize", default = "crate::pagination::default_page_size")]
    pub page_size: u32,
    pub active: Option<bool>,
}

pub async fn list_products(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<ListProductsQuery>,
) -> Result<Json<PaginatedProductsResponse>, ApiError> {
    require_can_read_products(&auth)?;
    let (page, page_size, offset) = paginate_offset(query.page, query.page_size);

    let rows = infra_postgres::inventory::list_products(
        &state.app_pool,
        auth.tenant_id,
        query.active,
        page_size as i64,
        offset,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let total =
        infra_postgres::inventory::count_products(&state.app_pool, auth.tenant_id, query.active)
            .await
            .map_err(|_| ApiError::internal())? as u64;

    Ok(Json(PaginatedProductsResponse {
        items: rows.iter().map(product_response_from_row).collect(),
        page,
        page_size,
        total,
    }))
}

pub(crate) fn require_can_read_products(auth: &AuthUser) -> Result<(), ApiError> {
    require_roles(auth, &[Role::Admin, Role::Driver, Role::Seller])
}

pub(crate) fn require_can_write_products(auth: &AuthUser) -> Result<(), ApiError> {
    match auth.role {
        Role::Admin => Ok(()),
        _ => Err(ApiError::forbidden()),
    }
}

pub use catalog::{create_product, get_product, update_product};
pub use catalog_images::{
    attach_product_image, delete_product_image, list_product_images,
};
pub use top_selling::list_top_selling_products;

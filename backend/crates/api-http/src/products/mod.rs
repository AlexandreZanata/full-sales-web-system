use axum::{
    Json,
    extract::{Query, State},
};
use domain_identity::Role;
use serde::Serialize;

use crate::auth::{AuthUser, require_roles};
use crate::error::ApiError;
use crate::pagination::{PaginationQuery, paginate_offset};
use crate::state::AppState;

pub mod catalog;
pub mod catalog_images;

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
    }
}

pub async fn list_products(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<PaginatedProductsResponse>, ApiError> {
    require_can_read_products(&auth)?;
    let (page, page_size, offset) = paginate_offset(query.page, query.page_size);

    let rows = infra_postgres::inventory::list_products(
        &state.app_pool,
        auth.tenant_id,
        page_size as i64,
        offset,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let total = infra_postgres::inventory::count_products(&state.app_pool, auth.tenant_id)
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
pub use catalog_images::{attach_product_image, delete_product_image};

use axum::{
    Json,
    extract::{RawQuery, State},
    response::{IntoResponse, Response},
};
use domain_identity::Role;
use serde::Serialize;

use crate::auth::{AuthUser, require_roles};
use crate::error::ApiError;
use crate::list_query::{
    CursorListResponse, PRODUCTS_LIST_CONFIG, build_cursor_page, decode_query_pairs,
    filter_eq_bool, parse_list_query,
};
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

pub async fn list_products(
    State(state): State<AppState>,
    auth: AuthUser,
    RawQuery(query): RawQuery,
) -> Result<Json<CursorListResponse<ProductResponse>>, Response> {
    require_can_read_products(&auth).map_err(IntoResponse::into_response)?;
    let parsed = parse_list_query(&decode_query_pairs(query.as_deref()), &PRODUCTS_LIST_CONFIG)
        .map_err(IntoResponse::into_response)?;
    let active = filter_eq_bool(&parsed.filters, "active");
    let rows = infra_postgres::inventory::list_products_cursor(
        &state.app_pool,
        auth.tenant_id,
        active,
        parsed.pagination.cursor,
        parsed.pagination.fetch_size() as i64,
    )
    .await
    .map_err(|_| IntoResponse::into_response(ApiError::internal()))?;

    let items: Vec<ProductResponse> = rows.iter().map(product_response_from_row).collect();
    Ok(Json(build_cursor_page(
        items,
        parsed.pagination.limit,
        |product| product.id,
    )))
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
pub use catalog_images::{attach_product_image, delete_product_image, list_product_images};
pub use top_selling::list_top_selling_products;

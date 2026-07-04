use axum::{
    Json,
    extract::{Query, State},
};
use domain_identity::Role;
use serde::{Deserialize, Serialize};

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ListProductsQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(rename = "pageSize", default = "default_page_size")]
    pub page_size: u32,
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    20
}

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

pub async fn list_products(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<ListProductsQuery>,
) -> Result<Json<PaginatedProductsResponse>, ApiError> {
    require_can_list_products(&auth)?;

    let page_size = query.page_size.clamp(1, 50);
    let page = query.page.max(1);
    let offset = ((page - 1) as i64) * (page_size as i64);

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

    let products: Vec<ProductResponse> = rows
        .iter()
        .map(|row| ProductResponse {
            id: row.id,
            name: row.name.clone(),
            sku: row.sku.clone(),
            price_amount: row.price_amount,
            price_currency: row.price_currency.clone(),
            active: row.active,
        })
        .collect();

    Ok(Json(PaginatedProductsResponse {
        items: products,
        page,
        page_size,
        total,
    }))
}

fn require_can_list_products(auth: &AuthUser) -> Result<(), ApiError> {
    match auth.role {
        Role::Admin | Role::Driver | Role::Seller => Ok(()),
        Role::CommerceContact => Err(ApiError::forbidden()),
    }
}

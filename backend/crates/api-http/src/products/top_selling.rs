use axum::{
    Json,
    extract::{Query, State},
};
use serde::{Deserialize, Serialize};

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::products::require_can_read_products;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct TopSellingProductsQuery {
    #[serde(default = "default_top_selling_limit")]
    pub limit: u32,
}

fn default_top_selling_limit() -> u32 {
    5
}

#[derive(Serialize)]
pub struct TopSellingProductResponse {
    #[serde(rename = "productId")]
    pub product_id: uuid::Uuid,
    pub name: String,
    pub sku: String,
    #[serde(rename = "unitsSold")]
    pub units_sold: i64,
}

#[derive(Serialize)]
pub struct TopSellingProductsResponse {
    pub items: Vec<TopSellingProductResponse>,
}

pub async fn list_top_selling_products(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<TopSellingProductsQuery>,
) -> Result<Json<TopSellingProductsResponse>, ApiError> {
    require_can_read_products(&auth)?;
    let limit = query.limit.clamp(1, 20) as i64;
    let rows = infra_postgres::sales::list_top_selling_products(
        &state.app_pool,
        auth.tenant_id,
        limit,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    Ok(Json(TopSellingProductsResponse {
        items: rows
            .into_iter()
            .map(|row| TopSellingProductResponse {
                product_id: row.product_id,
                name: row.name,
                sku: row.sku,
                units_sold: row.units_sold,
            })
            .collect(),
    }))
}

use axum::{
    Json,
    extract::{Query, State},
};
use application::list_query::ListPagination;
use serde::{Deserialize, Serialize};

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::list_query::{CursorListResponse, CursorPaginationMeta};
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

pub async fn list_top_selling_products(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<TopSellingProductsQuery>,
) -> Result<Json<CursorListResponse<TopSellingProductResponse>>, ApiError> {
    require_can_read_products(&auth)?;
    // ponytail: ranked list capped at 20 — not cursor-paginated across pages
    let limit = query.limit.clamp(1, 20);
    let _ = ListPagination::new(Some(limit), None).map_err(|_| {
        ApiError::bad_request("invalid_pagination", "limit must be between 1 and 100")
    })?;
    let rows = infra_postgres::sales::list_top_selling_products(
        &state.app_pool,
        auth.tenant_id,
        limit as i64,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let data = rows
        .into_iter()
        .map(|row| TopSellingProductResponse {
            product_id: row.product_id,
            name: row.name,
            sku: row.sku,
            units_sold: row.units_sold,
        })
        .collect();

    Ok(Json(CursorListResponse {
        data,
        pagination: CursorPaginationMeta {
            next_cursor: None,
            has_more: false,
            limit,
        },
    }))
}

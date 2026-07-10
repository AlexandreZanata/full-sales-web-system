use axum::{
    Json,
    extract::{Query, State},
    response::{IntoResponse, Response},
};

use crate::domains::PublicTenantId;
use crate::error::ApiError;
use crate::list_query::CursorListResponse;
use crate::portal::products::{
    build_portal_product_responses, list_products_cursor,
};
use crate::state::AppState;

#[derive(serde::Deserialize)]
pub struct FeaturedPopularQuery {
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_limit() -> u32 {
    12
}

pub async fn list_public_featured_products(
    State(state): State<AppState>,
    PublicTenantId(tenant_id): PublicTenantId,
    Query(query): Query<FeaturedPopularQuery>,
) -> Result<Json<CursorListResponse<crate::portal::products::PortalProductResponse>>, Response> {
    let limit = query.limit.clamp(1, 50) as i64;
    let rows = infra_postgres::inventory::portal_products::list_portal_featured_products(
        &state.app_pool,
        tenant_id,
        limit,
    )
    .await
    .map_err(|_| IntoResponse::into_response(ApiError::internal()))?;
    let items = build_portal_product_responses(&state, tenant_id, &rows)
        .await
        .map_err(IntoResponse::into_response)?;
    Ok(Json(crate::list_query::build_cursor_page(
        items,
        query.limit,
        |product| product.id,
    )))
}

pub async fn list_public_popular_products(
    State(state): State<AppState>,
    PublicTenantId(tenant_id): PublicTenantId,
    Query(query): Query<FeaturedPopularQuery>,
) -> Result<Json<CursorListResponse<crate::portal::products::PortalProductResponse>>, Response> {
    let limit = query.limit.clamp(1, 50) as i64;
    let rows = infra_postgres::inventory::portal_products::list_portal_popular_products(
        &state.app_pool,
        tenant_id,
        limit,
    )
    .await
    .map_err(|_| IntoResponse::into_response(ApiError::internal()))?;
    if rows.is_empty() {
        let query_string = format!("limit={}", query.limit);
        return list_products_cursor(&state, tenant_id, Some(&query_string))
            .await
            .map(Json)
            .map_err(IntoResponse::into_response);
    }
    let items = build_portal_product_responses(&state, tenant_id, &rows)
        .await
        .map_err(IntoResponse::into_response)?;
    Ok(Json(crate::list_query::build_cursor_page(
        items,
        query.limit,
        |product| product.id,
    )))
}

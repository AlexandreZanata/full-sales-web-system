use axum::{
    Json,
    extract::{Path, RawQuery, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use domain_identity::Role;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::{AuthUser, require_admin, require_roles};
use crate::error::ApiError;
use crate::list_query::{
    CursorListResponse, STOCK_BALANCES_LIST_CONFIG, STOCK_MOVEMENTS_LIST_CONFIG, build_cursor_page,
    decode_query_pairs, filter_gte_datetime, filter_like_pattern, filter_lte_datetime,
    parse_list_query,
};
use crate::state::AppState;

#[derive(Serialize)]
pub struct StockBalanceResponse {
    #[serde(rename = "productId")]
    pub product_id: Uuid,
    pub available: i32,
}

#[derive(Deserialize)]
pub struct RecordMovementRequest {
    #[serde(rename = "productId")]
    pub product_id: Uuid,
    #[serde(rename = "movementType")]
    pub movement_type: String,
    pub quantity: i32,
    pub reason: Option<String>,
}

#[derive(Serialize)]
pub struct StockMovementResponse {
    pub id: Uuid,
    #[serde(rename = "productId")]
    pub product_id: Uuid,
    #[serde(rename = "responsibleId")]
    pub responsible_id: Uuid,
    #[serde(rename = "movementType")]
    pub movement_type: String,
    pub quantity: i32,
    #[serde(rename = "referenceId", skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub struct ProductStockOverviewResponse {
    #[serde(rename = "productId")]
    pub product_id: Uuid,
    pub sku: String,
    pub name: String,
    #[serde(rename = "unitOfMeasure")]
    pub unit_of_measure: String,
    pub active: bool,
    #[serde(rename = "balanceTotal")]
    pub balance_total: i32,
    pub reserved: i32,
    pub available: i32,
}

pub async fn list_stock_balances(
    State(state): State<AppState>,
    auth: AuthUser,
    RawQuery(query): RawQuery,
) -> Result<Json<CursorListResponse<ProductStockOverviewResponse>>, Response> {
    require_admin(&auth).map_err(IntoResponse::into_response)?;
    let parsed = parse_list_query(
        &decode_query_pairs(query.as_deref()),
        &STOCK_BALANCES_LIST_CONFIG,
    )
    .map_err(IntoResponse::into_response)?;
    let name_like = filter_like_pattern(&parsed.filters, "name");
    let sku_like = filter_like_pattern(&parsed.filters, "sku");

    let rows = infra_postgres::inventory::stock_overview::list_product_stock_overview_cursor(
        &state.app_pool,
        auth.tenant_id,
        name_like.as_deref(),
        sku_like.as_deref(),
        parsed.pagination.cursor,
        parsed.pagination.fetch_size() as i64,
    )
    .await
    .map_err(|_| IntoResponse::into_response(ApiError::internal()))?;

    let items: Vec<ProductStockOverviewResponse> =
        rows.iter().map(stock_overview_response).collect();
    Ok(Json(build_cursor_page(
        items,
        parsed.pagination.limit,
        |row| row.product_id,
    )))
}

pub async fn get_stock_balance(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(product_id): Path<Uuid>,
) -> Result<Json<StockBalanceResponse>, ApiError> {
    require_roles(&auth, &[Role::Admin, Role::Driver, Role::Seller])?;
    ensure_product(&state, auth.tenant_id, product_id).await?;
    let available = infra_postgres::inventory::reservations::tenant_available_for_product(
        &state.app_pool,
        auth.tenant_id,
        product_id,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    Ok(Json(StockBalanceResponse {
        product_id,
        available,
    }))
}

pub async fn record_movement(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<RecordMovementRequest>,
) -> Result<(StatusCode, Json<StockMovementResponse>), ApiError> {
    require_admin(&auth)?;
    if body.movement_type != "Adjustment" {
        return Err(ApiError::bad_request(
            "INVALID_MOVEMENT_TYPE",
            "Only Adjustment movements may be recorded via API",
        ));
    }
    let reason = body
        .reason
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| ApiError::bad_request("REASON_REQUIRED", "Adjustment reason is required"))?;
    if body.quantity == 0 {
        return Err(ApiError::bad_request(
            "VALIDATION_ERROR",
            "Quantity must be non-zero",
        ));
    }
    ensure_product(&state, auth.tenant_id, body.product_id).await?;

    infra_postgres::inventory::insert_adjustment_movement(
        &state.app_pool,
        auth.tenant_id,
        auth.user_id,
        body.product_id,
        body.quantity.abs(),
        reason,
        body.quantity,
    )
    .await
    .map_err(map_adjustment_error)?;

    let rows = infra_postgres::inventory::list_stock_movements_by_product(
        &state.app_pool,
        auth.tenant_id,
        body.product_id,
        1,
        0,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    let row = rows.into_iter().next().ok_or_else(ApiError::internal)?;
    Ok((StatusCode::CREATED, Json(movement_response(&row))))
}

pub async fn list_movements(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(product_id): Path<Uuid>,
    RawQuery(query): RawQuery,
) -> Result<Json<CursorListResponse<StockMovementResponse>>, Response> {
    require_admin(&auth).map_err(IntoResponse::into_response)?;
    ensure_product(&state, auth.tenant_id, product_id)
        .await
        .map_err(IntoResponse::into_response)?;
    let parsed = parse_list_query(
        &decode_query_pairs(query.as_deref()),
        &STOCK_MOVEMENTS_LIST_CONFIG,
    )
    .map_err(IntoResponse::into_response)?;
    let from = filter_gte_datetime(&parsed.filters, "created_at");
    let to = filter_lte_datetime(&parsed.filters, "created_at");

    let rows = infra_postgres::inventory::list_stock_movements_by_product_cursor(
        &state.app_pool,
        auth.tenant_id,
        product_id,
        parsed.pagination.cursor,
        from,
        to,
        parsed.pagination.fetch_size() as i64,
    )
    .await
    .map_err(|_| IntoResponse::into_response(ApiError::internal()))?;

    let items: Vec<StockMovementResponse> = rows.iter().map(movement_response).collect();
    Ok(Json(build_cursor_page(
        items,
        parsed.pagination.limit,
        |movement| movement.id,
    )))
}

async fn ensure_product(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
    product_id: Uuid,
) -> Result<(), ApiError> {
    let exists =
        infra_postgres::inventory::find_product_by_id(&state.app_pool, tenant_id, product_id)
            .await
            .map_err(|_| ApiError::internal())?
            .is_some();
    if exists {
        Ok(())
    } else {
        Err(ApiError::product_not_found())
    }
}

fn stock_overview_response(
    row: &infra_postgres::inventory::stock_overview::ProductStockOverviewRow,
) -> ProductStockOverviewResponse {
    ProductStockOverviewResponse {
        product_id: row.product_id,
        sku: row.sku.clone(),
        name: row.name.clone(),
        unit_of_measure: row.unit_of_measure.clone(),
        active: row.active,
        balance_total: row.balance_total,
        reserved: row.reserved,
        available: row.available,
    }
}

fn movement_response(row: &infra_postgres::inventory::StockMovementRow) -> StockMovementResponse {
    StockMovementResponse {
        id: row.id,
        product_id: row.product_id,
        responsible_id: row.responsible_id,
        movement_type: row.movement_type.clone(),
        quantity: row.quantity,
        reference_id: row.reference_id,
        reason: row.reason.clone(),
        created_at: row.created_at,
    }
}

fn map_adjustment_error(err: infra_postgres::PostgresError) -> ApiError {
    match err {
        infra_postgres::PostgresError::Database(_) => ApiError::insufficient_balance(),
        _ => ApiError::internal(),
    }
}

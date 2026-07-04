use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use domain_identity::Role;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::{require_admin, require_roles, AuthUser};
use crate::error::ApiError;
use crate::pagination::{paginate_offset, PaginationQuery};
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
pub struct PaginatedMovementsResponse {
    pub items: Vec<StockMovementResponse>,
    pub page: u32,
    #[serde(rename = "pageSize")]
    pub page_size: u32,
    pub total: u64,
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
        .ok_or_else(|| {
            ApiError::bad_request("REASON_REQUIRED", "Adjustment reason is required")
        })?;
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
    Query(query): Query<PaginationQuery>,
) -> Result<Json<PaginatedMovementsResponse>, ApiError> {
    require_admin(&auth)?;
    ensure_product(&state, auth.tenant_id, product_id).await?;
    let (page, page_size, offset) = paginate_offset(query.page, query.page_size);

    let rows = infra_postgres::inventory::list_stock_movements_by_product(
        &state.app_pool,
        auth.tenant_id,
        product_id,
        page_size as i64,
        offset,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let total = rows.len() as u64;

    Ok(Json(PaginatedMovementsResponse {
        items: rows.iter().map(movement_response).collect(),
        page,
        page_size,
        total,
    }))
}

async fn ensure_product(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
    product_id: Uuid,
) -> Result<(), ApiError> {
    let exists = infra_postgres::inventory::find_product_by_id(&state.app_pool, tenant_id, product_id)
        .await
        .map_err(|_| ApiError::internal())?
        .is_some();
    if exists {
        Ok(())
    } else {
        Err(ApiError::product_not_found())
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

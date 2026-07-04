use axum::{
    Json,
    extract::{Path, Query, State},
};
use chrono::{DateTime, Utc};
use domain_orders::OrderStatus;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::admin_orders::AdminOrderResponse;
use crate::auth::{require_admin, AuthUser};
use crate::error::ApiError;
use crate::pagination::paginate_offset;
use crate::portal::{load_order, map_order_error, map_postgres_order_error, order_to_response};
use crate::session::session_from_auth;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ListOrdersQuery {
    #[serde(default = "crate::pagination::default_page")]
    pub page: u32,
    #[serde(rename = "pageSize", default = "crate::pagination::default_page_size")]
    pub page_size: u32,
    pub status: Option<String>,
    #[serde(rename = "commerceId")]
    pub commerce_id: Option<Uuid>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct OrderSummaryResponse {
    pub id: Uuid,
    pub status: String,
    #[serde(rename = "commerceId")]
    pub commerce_id: Uuid,
    #[serde(rename = "totalAmount")]
    pub total_amount: i64,
    #[serde(rename = "totalCurrency")]
    pub total_currency: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct PaginatedOrdersResponse {
    pub items: Vec<OrderSummaryResponse>,
    pub page: u32,
    #[serde(rename = "pageSize")]
    pub page_size: u32,
    pub total: u64,
}

#[derive(Serialize)]
pub struct DeliverySummaryResponse {
    pub id: Uuid,
    #[serde(rename = "driverId")]
    pub driver_id: Uuid,
    pub status: String,
}

#[derive(Serialize)]
pub struct AdminOrderDetailResponse {
    #[serde(flatten)]
    pub order: crate::portal::PortalOrderResponse,
    #[serde(rename = "commerceId")]
    pub commerce_id: Uuid,
    #[serde(rename = "delivery", skip_serializing_if = "Option::is_none")]
    pub delivery: Option<DeliverySummaryResponse>,
}

pub async fn list_orders(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<ListOrdersQuery>,
) -> Result<Json<PaginatedOrdersResponse>, ApiError> {
    require_admin(&auth)?;
    let session = session_from_auth(&auth);
    let (page, page_size, offset) = paginate_offset(query.page, query.page_size);

    let rows = infra_postgres::orders::list_orders(
        &state.app_pool,
        &session,
        query.status.as_deref(),
        page_size as i64,
        offset,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let total = infra_postgres::orders::count_orders(&state.app_pool, &session, query.status.as_deref())
        .await
        .map_err(|_| ApiError::internal())? as u64;

    let mut items: Vec<OrderSummaryResponse> = Vec::new();
    for row in rows {
        let detail = infra_postgres::orders::find_order_detail(&state.app_pool, &session, row.id)
            .await
            .map_err(|_| ApiError::internal())?;
        let Some(detail) = detail else { continue };
        if query.commerce_id.is_some_and(|id| detail.commerce_id != id) {
            continue;
        }
        items.push(OrderSummaryResponse {
            id: row.id,
            status: row.status,
            commerce_id: detail.commerce_id,
            total_amount: row.total_amount,
            total_currency: row.total_currency,
            created_at: row.created_at,
        });
    }

    Ok(Json(PaginatedOrdersResponse {
        items,
        page,
        page_size,
        total,
    }))
}

pub async fn get_order(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<AdminOrderDetailResponse>, ApiError> {
    require_admin(&auth)?;
    let session = session_from_auth(&auth);
    let detail = infra_postgres::orders::find_order_detail(&state.app_pool, &session, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::order_not_found)?;
    let order = load_order(&state, &session, id).await?;
    let delivery = infra_postgres::deliveries::find_delivery_by_order_id(&state.app_pool, &session, id)
        .await
        .map_err(|_| ApiError::internal())?
        .map(|row| DeliverySummaryResponse {
            id: row.id,
            driver_id: row.driver_id,
            status: row.status,
        });

    Ok(Json(AdminOrderDetailResponse {
        commerce_id: detail.commerce_id,
        order: order_to_response(&order)?,
        delivery,
    }))
}

pub async fn cancel_order(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<AdminOrderResponse>, ApiError> {
    require_admin(&auth)?;
    let session = session_from_auth(&auth);
    let order = load_order(&state, &session, id).await?;
    let result = application::orders::cancel_order(order).map_err(map_order_error)?;
    infra_postgres::orders::cancel_order_transaction(
        &state.app_pool,
        &session,
        id,
        result.release_reservations,
    )
    .await
    .map_err(map_postgres_order_error)?;
    let cancelled = load_order(&state, &session, id).await?;
    Ok(Json(AdminOrderResponse {
        order: order_to_response(&cancelled)?,
    }))
}

pub async fn start_picking(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<AdminOrderResponse>, ApiError> {
    require_admin(&auth)?;
    let session = session_from_auth(&auth);
    let order = load_order(&state, &session, id).await?;
    if order.status() != OrderStatus::Approved {
        return Err(ApiError::invalid_order_transition());
    }
    let picking = order.start_picking().map_err(|e| map_order_error(e.into()))?;
    infra_postgres::orders::update_order_status(
        &state.app_pool,
        &session,
        id,
        OrderStatus::Picking.as_str(),
    )
    .await
    .map_err(map_postgres_order_error)?;
    Ok(Json(AdminOrderResponse {
        order: order_to_response(&picking)?,
    }))
}

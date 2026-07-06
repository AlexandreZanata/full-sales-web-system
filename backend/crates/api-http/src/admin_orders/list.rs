use axum::{
    Json,
    extract::{Path, RawQuery, State},
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use domain_orders::OrderStatus;
use serde::Serialize;
use uuid::Uuid;

use crate::admin_orders::AdminOrderResponse;
use crate::auth::{AuthUser, require_admin};
use crate::error::ApiError;
use crate::list_query::{
    CursorListResponse, ORDERS_LIST_CONFIG, build_cursor_page, decode_query_pairs,
    filter_eq_string, filter_eq_uuid, filter_gte_datetime, filter_lte_datetime, parse_list_query,
};
use crate::portal::{load_order, map_order_error, map_postgres_order_error, order_to_response};
use crate::session::session_from_auth;
use crate::state::AppState;

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
    RawQuery(query): RawQuery,
) -> Result<Json<CursorListResponse<OrderSummaryResponse>>, Response> {
    require_admin(&auth).map_err(IntoResponse::into_response)?;
    let session = session_from_auth(&auth);
    let parsed = parse_list_query(&decode_query_pairs(query.as_deref()), &ORDERS_LIST_CONFIG)
        .map_err(IntoResponse::into_response)?;
    let status = filter_eq_string(&parsed.filters, "status");
    let filters = infra_postgres::orders::OrderListFilters {
        status: status.as_deref(),
        commerce_id: filter_eq_uuid(&parsed.filters, "commerce_id"),
        from: filter_gte_datetime(&parsed.filters, "created_at"),
        to: filter_lte_datetime(&parsed.filters, "created_at"),
    };

    let rows = infra_postgres::orders::list_orders_cursor(
        &state.app_pool,
        &session,
        &filters,
        parsed.pagination.cursor,
        parsed.pagination.fetch_size() as i64,
    )
    .await
    .map_err(|_| IntoResponse::into_response(ApiError::internal()))?;

    let items: Vec<OrderSummaryResponse> = rows
        .into_iter()
        .map(|row| OrderSummaryResponse {
            id: row.id,
            status: row.status,
            commerce_id: row.commerce_id,
            total_amount: row.total_amount,
            total_currency: row.total_currency,
            created_at: row.created_at,
        })
        .collect();

    Ok(Json(build_cursor_page(
        items,
        parsed.pagination.limit,
        |order| order.id,
    )))
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
    let delivery =
        infra_postgres::deliveries::find_delivery_by_order_id(&state.app_pool, &session, id)
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
    let picking = order
        .start_picking()
        .map_err(|e| map_order_error(e.into()))?;
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

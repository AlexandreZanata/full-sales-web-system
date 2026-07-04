use application::orders::{
    ApproveOrderResult, approve_order as approve_order_domain, reject_order, reservation_lines,
};
use axum::{
    Json,
    extract::{Path, State},
};
use domain_inventory::ProductId;
use domain_orders::OrderStatus;
use infra_postgres::inventory::reservations::ReservationLine;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::{AuthUser, require_admin};
use crate::error::ApiError;
use crate::portal::{
    PortalOrderResponse, load_order, map_order_error, map_postgres_order_error, order_to_response,
};
use crate::session::session_from_auth;
use crate::state::AppState;
use crate::validation::ValidatedJson;

pub mod list;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RejectOrderRequest {
    pub reason: String,
}

#[derive(Serialize)]
pub struct AdminOrderResponse {
    #[serde(flatten)]
    pub order: PortalOrderResponse,
}

pub async fn approve_order(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<AdminOrderResponse>, ApiError> {
    require_admin(&auth)?;
    let session = session_from_auth(&auth);
    let order = load_order(&state, &session, id).await?;
    if order.status() != OrderStatus::PendingApproval {
        return Err(ApiError::invalid_order_transition());
    }

    let balances = stock_snapshot(&state, auth.tenant_id, order.items()).await?;
    let approved: ApproveOrderResult =
        approve_order_domain(order, balances.0, balances.1).map_err(map_order_error)?;

    let lines: Vec<ReservationLine> = reservation_lines(&approved.reservations)
        .into_iter()
        .map(|line| ReservationLine {
            id: line.id,
            order_id: line.order_id,
            order_item_id: line.order_item_id,
            product_id: line.product_id,
            quantity: line.quantity,
            driver_id: line.driver_id,
        })
        .collect();

    infra_postgres::orders::approve_order_transaction(&state.app_pool, &session, id, &lines)
        .await
        .map_err(|err| match err {
            infra_postgres::PostgresError::InsufficientAvailableStock => {
                ApiError::insufficient_stock()
            }
            infra_postgres::PostgresError::Database(_) => ApiError::invalid_order_transition(),
            _ => ApiError::internal(),
        })?;

    Ok(Json(AdminOrderResponse {
        order: order_to_response(&approved.order)?,
    }))
}

pub async fn reject_order_handler(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    ValidatedJson(body): ValidatedJson<RejectOrderRequest>,
) -> Result<Json<AdminOrderResponse>, ApiError> {
    require_admin(&auth)?;
    if body.reason.trim().is_empty() {
        return Err(ApiError::rejection_reason_required());
    }
    let session = session_from_auth(&auth);
    let order = load_order(&state, &session, id).await?;
    let rejected = reject_order(order, &body.reason).map_err(map_order_error)?;
    infra_postgres::orders::reject_order(&state.app_pool, &session, id, body.reason.trim())
        .await
        .map_err(map_postgres_order_error)?;
    Ok(Json(AdminOrderResponse {
        order: order_to_response(&rejected)?,
    }))
}

async fn stock_snapshot(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
    items: &[domain_orders::OrderItem],
) -> Result<
    (
        std::collections::HashMap<ProductId, i32>,
        std::collections::HashMap<ProductId, i32>,
    ),
    ApiError,
> {
    let mut balances = std::collections::HashMap::new();
    let mut reserved = std::collections::HashMap::new();
    for item in items {
        let product_id = item.product_id();
        if balances.contains_key(&product_id) {
            continue;
        }
        let (balance, active_reserved) =
            infra_postgres::inventory::reservations::tenant_stock_snapshot(
                &state.app_pool,
                tenant_id,
                product_id.as_uuid(),
            )
            .await
            .map_err(|_| ApiError::internal())?;
        balances.insert(product_id, balance);
        reserved.insert(product_id, active_reserved);
    }
    Ok((balances, reserved))
}

pub use list::{cancel_order, get_order, list_orders, start_picking};

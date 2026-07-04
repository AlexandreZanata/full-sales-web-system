mod support;

use application::deliveries::{confirm_delivery_and_create_sale, ConfirmDeliveryAndCreateSaleCommand};
use axum::{
    Json,
    extract::{Path, Query, State},
    response::Response,
};
use domain_deliveries::{Delivery, DeliveryCreateInput, DeliveryId, DeliveryStatus};
use domain_identity::UserId;
use domain_media::FileId;
use domain_orders::OrderStatus;
use domain_sales::SaleId;
use infra_postgres::deliveries::{DeliveryFilters, DeliveryInsert};
use uuid::Uuid;

use crate::auth::{require_admin, AuthUser};
use crate::error::ApiError;
use crate::portal::load_order;
use crate::session::session_from_auth;
use crate::state::AppState;
use crate::validation::ValidatedJson;

pub use support::{
    ConfirmDeliveryRequest, CreateDeliveryRequest, DeliveriesQuery, DeliveryResponse,
    PaginatedDeliveriesResponse,
};

pub async fn create_order_delivery(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(order_id): Path<Uuid>,
    ValidatedJson(body): ValidatedJson<CreateDeliveryRequest>,
) -> Result<Response, ApiError> {
    require_admin(&auth)?;
    let session = session_from_auth(&auth);
    let order = load_order(&state, &session, order_id).await?;
    if !matches!(order.status(), OrderStatus::Approved | OrderStatus::Picking) {
        return Err(ApiError::invalid_order_transition());
    }
    if support::order_has_delivery(&state, &session, order_id).await? {
        return Err(ApiError::bad_request("DELIVERY_EXISTS", "Order already has a delivery"));
    }
    support::ensure_active_driver(&state, auth.tenant_id, body.driver_id).await?;

    let delivery_id = DeliveryId::generate();
    Delivery::create(DeliveryCreateInput {
        id: delivery_id,
        tenant_id: auth.tenant_id,
        order_id: domain_orders::OrderId::from_uuid(order_id),
        driver_id: UserId::from_uuid(body.driver_id),
    });
    infra_postgres::deliveries::insert_delivery(
        &state.app_pool,
        &session,
        &DeliveryInsert {
            id: delivery_id.as_uuid(),
            order_id,
            driver_id: body.driver_id,
            status: DeliveryStatus::Waiting.as_str().to_owned(),
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;

    support::created_delivery_response(
        delivery_id.as_uuid(),
        order_id,
        body.driver_id,
    )
}

pub async fn list_deliveries(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<DeliveriesQuery>,
) -> Result<Json<PaginatedDeliveriesResponse>, ApiError> {
    support::require_delivery_reader(&auth)?;
    let session = session_from_auth(&auth);
    let page_size = query.page_size.clamp(1, 50);
    let page = query.page.max(1);
    let offset = ((page - 1) as i64) * (page_size as i64);
    let filters = DeliveryFilters {
        driver_id: None,
        status: query.status.clone(),
    };
    let rows = infra_postgres::deliveries::list_deliveries(
        &state.app_pool, &session, &filters, page_size as i64, offset,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    let total = infra_postgres::deliveries::count_deliveries(&state.app_pool, &session, &filters)
        .await
        .map_err(|_| ApiError::internal())? as u64;
    Ok(Json(PaginatedDeliveriesResponse {
        items: rows.into_iter().map(support::row_to_response).collect(),
        page,
        page_size,
        total,
    }))
}

pub async fn get_delivery(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<DeliveryResponse>, ApiError> {
    support::require_delivery_reader(&auth)?;
    Ok(Json(support::row_to_response(
        support::load_row(&state, &auth, id).await?,
    )))
}

pub async fn start_delivery_transit(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<DeliveryResponse>, ApiError> {
    support::require_assigned_driver(&auth)?;
    let session = session_from_auth(&auth);
    let row = support::load_row(&state, &auth, id).await?;
    support::restore(&row, auth.tenant_id)?
        .start_transit(UserId::from_uuid(auth.user_id))
        .map_err(|e| support::map_app_error(e.into()))?;
    infra_postgres::deliveries::start_delivery_transit(
        &state.app_pool, &session, id, auth.user_id, row.order_id,
    )
    .await
    .map_err(support::map_postgres_error)?;
    get_delivery(State(state), auth, Path(id)).await
}

pub async fn confirm_delivery(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    ValidatedJson(body): ValidatedJson<ConfirmDeliveryRequest>,
) -> Result<Json<DeliveryResponse>, ApiError> {
    support::require_assigned_driver(&auth)?;
    if body.items.is_empty() {
        return Err(ApiError::bad_request("VALIDATION_ERROR", "items are required"));
    }
    let driver_session = session_from_auth(&auth);
    let admin_session = support::admin_session(&auth);
    let row = support::load_row(&state, &auth, id).await?;
    let order = load_order(&state, &admin_session, row.order_id).await?;
    let sale_id = SaleId::generate();
    let result = confirm_delivery_and_create_sale(ConfirmDeliveryAndCreateSaleCommand {
        delivery: support::restore(&row, auth.tenant_id)?,
        order,
        items: support::parse_items(&body.items)?,
        proof_file_id: FileId::from_uuid(body.proof_file_id),
        latitude: body.latitude,
        longitude: body.longitude,
        received_by_name: body.received_by_name,
        acting_driver: UserId::from_uuid(auth.user_id),
        sale_id,
    })
    .map_err(support::map_app_error)?;
    let tx = support::confirm_tx(&row, &result, body.proof_file_id, sale_id.as_uuid())
        .map_err(|_| ApiError::internal())?;
    infra_postgres::deliveries::confirm_delivery_transaction(
        &state.app_pool,
        &driver_session,
        &admin_session,
        &tx,
    )
    .await
    .map_err(support::map_confirm_postgres_error)?;
    Ok(Json(support::delivery_response(
        id,
        row.order_id,
        row.driver_id,
        DeliveryStatus::Delivered,
        Some(sale_id.as_uuid()),
    )))
}

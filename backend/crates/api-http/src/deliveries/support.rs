use application::deliveries::{sale_lines_from_result, ConfirmDeliveryAndCreateSaleResult};
use axum::{
    http::{HeaderValue, StatusCode},
    response::Response,
};
use domain_deliveries::{Delivery, DeliveryError, DeliveryId, DeliveryStatus};
use domain_identity::{Role, UserId};
use domain_inventory::Quantity;
use domain_orders::{DeliveredItemInput, OrderItemId};
use infra_postgres::deliveries::{ConfirmDeliveryItemUpdate, ConfirmDeliveryTxInput, DeliveryRow};
use infra_postgres::rls::SessionContext;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::session::session_from_auth;
use crate::state::AppState;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateDeliveryRequest {
    #[serde(rename = "driverId")]
    pub driver_id: Uuid,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfirmDeliveryItemRequest {
    #[serde(rename = "orderItemId")]
    pub order_item_id: Uuid,
    #[serde(rename = "quantityDelivered")]
    pub quantity_delivered: i32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfirmDeliveryRequest {
    #[serde(rename = "proofFileId")]
    pub proof_file_id: Uuid,
    pub items: Vec<ConfirmDeliveryItemRequest>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    #[serde(rename = "receivedByName")]
    pub received_by_name: Option<String>,
}

#[derive(Serialize)]
pub struct DeliveryResponse {
    pub id: Uuid,
    #[serde(rename = "orderId")]
    pub order_id: Uuid,
    #[serde(rename = "driverId")]
    pub driver_id: Uuid,
    pub status: DeliveryStatus,
    #[serde(rename = "saleId", skip_serializing_if = "Option::is_none")]
    pub sale_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct PaginatedDeliveriesResponse {
    pub items: Vec<DeliveryResponse>,
    pub page: u32,
    #[serde(rename = "pageSize")]
    pub page_size: u32,
    pub total: u64,
}

#[derive(Deserialize)]
pub struct DeliveriesQuery {
    pub status: Option<String>,
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

pub async fn order_has_delivery(
    state: &AppState,
    session: &SessionContext,
    order_id: Uuid,
) -> Result<bool, ApiError> {
    Ok(
        infra_postgres::deliveries::find_delivery_by_order_id(&state.app_pool, session, order_id)
            .await
            .map_err(|_| ApiError::internal())?
            .is_some(),
    )
}

pub async fn ensure_active_driver(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
    driver_id: Uuid,
) -> Result<(), ApiError> {
    let driver = infra_postgres::identity::find_user_by_id(&state.app_pool, tenant_id, driver_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(|| ApiError::bad_request("VALIDATION_ERROR", "Driver not found"))?;
    if driver.role != Role::Driver.as_str() || !driver.active {
        return Err(ApiError::bad_request("VALIDATION_ERROR", "Invalid driverId"));
    }
    Ok(())
}

pub fn created_delivery_response(
    id: Uuid,
    order_id: Uuid,
    driver_id: Uuid,
) -> Result<Response, ApiError> {
    let location = format!("/v1/deliveries/{id}");
    let body = serde_json::to_vec(&delivery_response(
        id,
        order_id,
        driver_id,
        DeliveryStatus::Waiting,
        None,
    ))
    .map_err(|_| ApiError::internal())?;
    Response::builder()
        .status(StatusCode::CREATED)
        .header(http::header::CONTENT_TYPE, "application/json")
        .header(
            http::header::LOCATION,
            HeaderValue::from_str(&location).map_err(|_| ApiError::internal())?,
        )
        .body(axum::body::Body::from(body))
        .map_err(|_| ApiError::internal())
}

pub async fn load_row(state: &AppState, auth: &AuthUser, id: Uuid) -> Result<DeliveryRow, ApiError> {
    infra_postgres::deliveries::find_delivery_by_id(&state.app_pool, &session_from_auth(auth), id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::delivery_not_found)
}

pub fn restore(row: &DeliveryRow, tenant_id: domain_shared::TenantId) -> Result<Delivery, ApiError> {
    Ok(Delivery::restore(
        DeliveryId::from_uuid(row.id),
        tenant_id,
        domain_orders::OrderId::from_uuid(row.order_id),
        UserId::from_uuid(row.driver_id),
        row.status.parse().map_err(|_| ApiError::internal())?,
        None,
        None,
        None,
        None,
    ))
}

pub fn parse_items(items: &[ConfirmDeliveryItemRequest]) -> Result<Vec<DeliveredItemInput>, ApiError> {
    items
        .iter()
        .map(|item| {
            Ok(DeliveredItemInput {
                order_item_id: OrderItemId::from_uuid(item.order_item_id),
                quantity_delivered: Quantity::of(item.quantity_delivered).map_err(|_| {
                    ApiError::bad_request("VALIDATION_ERROR", "Invalid quantityDelivered")
                })?,
            })
        })
        .collect()
}

pub fn confirm_tx(
    row: &DeliveryRow,
    result: &ConfirmDeliveryAndCreateSaleResult,
    proof_file_id: Uuid,
    sale_id: Uuid,
) -> Result<ConfirmDeliveryTxInput, domain_shared::DomainError> {
    let sale_lines = sale_lines_from_result(&result.sale)?;
    Ok(ConfirmDeliveryTxInput {
        delivery_id: row.id,
        order_id: row.order_id,
        driver_id: row.driver_id,
        order_status: result.order.status().as_str().to_owned(),
        proof_file_id,
        latitude: result.delivery.latitude(),
        longitude: result.delivery.longitude(),
        received_by_name: result.delivery.received_by_name().map(str::to_owned),
        order_items: result
            .order
            .items()
            .iter()
            .map(|item| ConfirmDeliveryItemUpdate {
                order_item_id: item.id().as_uuid(),
                quantity_delivered: item.quantity_delivered().map(|q| q.value()).unwrap_or(0),
            })
            .collect(),
        sale_id,
        commerce_id: result.order.commerce_id().as_uuid(),
        sale_items: sale_lines
            .iter()
            .map(|line| infra_postgres::sales::NewSaleItem {
                id: Uuid::now_v7(),
                sale_id,
                product_id: line.product_id,
                quantity: line.quantity,
                unit_price_amount: line.unit_price_amount,
                unit_price_currency: line.unit_price_currency.clone(),
                line_total_amount: line.line_total_amount,
            })
            .collect(),
        stock_lines: sale_lines
            .iter()
            .map(|line| infra_postgres::sales::ConfirmSaleItem {
                product_id: line.product_id,
                quantity: line.quantity,
            })
            .collect(),
    })
}

pub fn row_to_response(row: DeliveryRow) -> DeliveryResponse {
    let status = row.status.parse().unwrap_or(DeliveryStatus::Waiting);
    delivery_response(row.id, row.order_id, row.driver_id, status, None)
}

pub fn delivery_response(
    id: Uuid,
    order_id: Uuid,
    driver_id: Uuid,
    status: DeliveryStatus,
    sale_id: Option<Uuid>,
) -> DeliveryResponse {
    DeliveryResponse {
        id,
        order_id,
        driver_id,
        status,
        sale_id,
    }
}

pub fn admin_session(auth: &AuthUser) -> SessionContext {
    SessionContext {
        tenant_id: auth.tenant_id,
        role: Role::Admin.as_str().to_owned(),
        user_id: auth.user_id,
        commerce_id: auth.commerce_id,
    }
}

pub fn require_delivery_reader(auth: &AuthUser) -> Result<(), ApiError> {
    match auth.role {
        Role::Admin | Role::Driver => Ok(()),
        _ => Err(ApiError::forbidden()),
    }
}

pub fn require_assigned_driver(auth: &AuthUser) -> Result<(), ApiError> {
    if auth.role == Role::Driver {
        Ok(())
    } else {
        Err(ApiError::forbidden())
    }
}

pub fn map_app_error(err: application::deliveries::DeliveriesAppError) -> ApiError {
    match err {
        application::deliveries::DeliveriesAppError::Delivery(DeliveryError::ProofRequired) => {
            ApiError::proof_required()
        }
        application::deliveries::DeliveriesAppError::Delivery(DeliveryError::DriverNotAssigned) => {
            ApiError::forbidden()
        }
        application::deliveries::DeliveriesAppError::Delivery(DeliveryError::InvalidTransition { .. }) => {
            ApiError::invalid_delivery_transition()
        }
        application::deliveries::DeliveriesAppError::Order(
            domain_orders::OrderError::InvalidDeliveredQuantity
            | domain_orders::OrderError::OrderItemNotFound,
        ) => ApiError::bad_request("VALIDATION_ERROR", "Invalid delivery items"),
        application::deliveries::DeliveriesAppError::Order(
            domain_orders::OrderError::InvalidTransition { .. },
        ) => ApiError::invalid_order_transition(),
        _ => ApiError::bad_request("VALIDATION_ERROR", "Invalid delivery request"),
    }
}

pub fn map_postgres_error(err: infra_postgres::PostgresError) -> ApiError {
    match err {
        infra_postgres::PostgresError::Database(_) => ApiError::invalid_delivery_transition(),
        _ => ApiError::internal(),
    }
}

pub fn map_confirm_postgres_error(err: infra_postgres::PostgresError) -> ApiError {
    match err {
        infra_postgres::PostgresError::InsufficientAvailableStock => ApiError::insufficient_stock(),
        infra_postgres::PostgresError::Database(_) => ApiError::invalid_delivery_transition(),
        _ => ApiError::internal(),
    }
}

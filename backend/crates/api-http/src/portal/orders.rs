use application::orders::{
    CreatePortalOrderCommand, PortalOrderLineInput, UpdatePortalDraftCommand,
    create_portal_order as build_portal_order, order_to_dto,
    submit_portal_order as submit_order_domain, update_portal_draft,
};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::Response,
};
use domain_commerces::CommerceId;
use domain_identity::UserId;
use domain_inventory::{ProductId, Quantity};
use domain_orders::{Order, OrderId, OrderItem, OrderItemId, OrderStatus};
use domain_shared::{Currency, Money};
use infra_postgres::orders::{OrderInsert, OrderItemInsert};
use infra_postgres::rls::SessionContext;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::portal::products::require_commerce_contact;
use crate::session::session_from_auth;
use crate::state::AppState;
use crate::validation::{ValidatedJson, to_json_bytes};

#[derive(Deserialize)]
pub struct PortalOrdersQuery {
    pub status: Option<String>,
    #[serde(default = "super::products::default_page")]
    pub page: u32,
    #[serde(rename = "pageSize", default = "super::products::default_page_size")]
    pub page_size: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PortalOrderLineRequest {
    #[serde(rename = "productId")]
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreatePortalOrderRequest {
    #[serde(rename = "deliveryAddressId")]
    pub delivery_address_id: Uuid,
    pub notes: Option<String>,
    pub items: Vec<PortalOrderLineRequest>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdatePortalOrderRequest {
    #[serde(rename = "deliveryAddressId")]
    pub delivery_address_id: Uuid,
    pub notes: Option<String>,
    pub items: Vec<PortalOrderLineRequest>,
}

#[derive(Serialize)]
pub struct PortalOrderItemResponse {
    pub id: Uuid,
    #[serde(rename = "productId")]
    pub product_id: Uuid,
    pub quantity: i32,
    #[serde(rename = "unitPriceAmount")]
    pub unit_price_amount: i64,
    #[serde(rename = "unitPriceCurrency")]
    pub unit_price_currency: String,
    #[serde(rename = "lineTotalAmount")]
    pub line_total_amount: i64,
}

#[derive(Serialize)]
pub(crate) struct PortalOrderResponse {
    pub id: Uuid,
    pub status: OrderStatus,
    #[serde(rename = "deliveryAddressId")]
    pub delivery_address_id: Uuid,
    pub notes: Option<String>,
    #[serde(rename = "totalAmount")]
    pub total_amount: i64,
    #[serde(rename = "totalCurrency")]
    pub total_currency: String,
    #[serde(rename = "rejectionReason")]
    pub rejection_reason: Option<String>,
    pub items: Vec<PortalOrderItemResponse>,
}

#[derive(Serialize)]
pub struct PortalOrderSummaryResponse {
    pub id: Uuid,
    pub status: String,
    #[serde(rename = "totalAmount")]
    pub total_amount: i64,
    #[serde(rename = "totalCurrency")]
    pub total_currency: String,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub struct PaginatedPortalOrdersResponse {
    pub items: Vec<PortalOrderSummaryResponse>,
    pub page: u32,
    #[serde(rename = "pageSize")]
    pub page_size: u32,
    pub total: u64,
}

pub async fn list_portal_orders(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<PortalOrdersQuery>,
) -> Result<Json<PaginatedPortalOrdersResponse>, ApiError> {
    let _ = require_commerce_contact(&auth)?;
    let session = session_from_auth(&auth);
    let page_size = query.page_size.clamp(1, 50);
    let page = query.page.max(1);
    let offset = ((page - 1) as i64) * (page_size as i64);

    let filters = infra_postgres::orders::OrderListFilters {
        status: query.status.as_deref(),
        commerce_id: None,
        from: None,
        to: None,
    };

    let rows = infra_postgres::orders::list_orders(
        &state.app_pool,
        &session,
        &filters,
        page_size as i64,
        offset,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let total = infra_postgres::orders::count_orders(&state.app_pool, &session, &filters)
        .await
        .map_err(|_| ApiError::internal())? as u64;

    Ok(Json(PaginatedPortalOrdersResponse {
        items: rows
            .into_iter()
            .map(|row| PortalOrderSummaryResponse {
                id: row.id,
                status: row.status,
                total_amount: row.total_amount,
                total_currency: row.total_currency,
                created_at: row.created_at,
            })
            .collect(),
        page,
        page_size,
        total,
    }))
}

pub async fn get_portal_order(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<PortalOrderResponse>, ApiError> {
    let _ = require_commerce_contact(&auth)?;
    let order = load_order(&state, &session_from_auth(&auth), id).await?;
    Ok(Json(order_to_response(&order)?))
}

pub async fn create_portal_order(
    State(state): State<AppState>,
    auth: AuthUser,
    ValidatedJson(body): ValidatedJson<CreatePortalOrderRequest>,
) -> Result<Response, ApiError> {
    let commerce_id = require_commerce_contact(&auth)?;
    validate_order_lines(&body.items)?;
    let session = session_from_auth(&auth);
    let order_id = OrderId::generate();
    let (commerce, address, lines) = load_create_inputs(
        &state,
        &auth,
        commerce_id,
        body.delivery_address_id,
        &body.items,
    )
    .await?;

    let order = build_portal_order(CreatePortalOrderCommand {
        order_id,
        tenant_id: auth.tenant_id,
        commerce,
        delivery_address: address,
        created_by: UserId::from_uuid(auth.user_id),
        notes: body.notes,
        lines,
    })
    .map_err(map_order_error)?;

    persist_new_order(&state, &session, &order).await?;
    let response = order_to_response(&order)?;
    let location = format!("/v1/portal/orders/{}", order.id().as_uuid());
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header(http::header::CONTENT_TYPE, "application/json")
        .header(http::header::LOCATION, location)
        .body(axum::body::Body::from(to_json_bytes(&response)?))
        .map_err(|_| ApiError::internal())?)
}

pub async fn update_portal_order(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    ValidatedJson(body): ValidatedJson<UpdatePortalOrderRequest>,
) -> Result<Json<PortalOrderResponse>, ApiError> {
    let commerce_id = require_commerce_contact(&auth)?;
    validate_order_lines(&body.items)?;
    let session = session_from_auth(&auth);
    let existing = load_order(&state, &session, id).await?;
    if existing.status() != OrderStatus::Draft {
        return Err(ApiError::invalid_order_transition());
    }

    let (commerce, address, lines) = load_create_inputs(
        &state,
        &auth,
        commerce_id,
        body.delivery_address_id,
        &body.items,
    )
    .await?;

    let order = update_portal_draft(UpdatePortalDraftCommand {
        order: existing,
        commerce,
        delivery_address: address,
        notes: body.notes,
        lines,
    })
    .map_err(map_order_error)?;

    let (total_amount, total_currency) = order_total(&order)?;
    let items = order_item_inserts(&order);
    infra_postgres::orders::replace_draft_order(
        &state.app_pool,
        &session,
        id,
        order.delivery_address_id().as_uuid(),
        order.notes().map(str::to_owned),
        total_amount,
        &total_currency,
        &items,
    )
    .await
    .map_err(map_postgres_order_error)?;

    Ok(Json(order_to_response(&order)?))
}

pub async fn submit_portal_order(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<PortalOrderResponse>, ApiError> {
    let _ = require_commerce_contact(&auth)?;
    let session = session_from_auth(&auth);
    let existing = load_order(&state, &session, id).await?;
    let order = submit_order_domain(existing).map_err(map_order_error)?;
    infra_postgres::orders::submit_order(&state.app_pool, &session, id)
        .await
        .map_err(map_postgres_order_error)?;
    Ok(Json(order_to_response(&order)?))
}

pub async fn cancel_portal_order(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let _ = require_commerce_contact(&auth)?;
    let session = session_from_auth(&auth);
    let existing = load_order(&state, &session, id).await?;
    if existing.status() != OrderStatus::Draft {
        return Err(ApiError::invalid_order_transition());
    }
    infra_postgres::orders::cancel_draft_order(&state.app_pool, &session, id)
        .await
        .map_err(map_postgres_order_error)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn load_create_inputs(
    state: &AppState,
    auth: &AuthUser,
    commerce_id: Uuid,
    delivery_address_id: Uuid,
    items: &[PortalOrderLineRequest],
) -> Result<
    (
        domain_commerces::Commerce,
        domain_commerces::CommerceAddress,
        Vec<PortalOrderLineInput>,
    ),
    ApiError,
> {
    let commerce_row = infra_postgres::commerces::find_commerce_by_id(
        &state.app_pool,
        auth.tenant_id,
        commerce_id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::commerce_not_found)?;

    let commerce = application::restore_commerce(
        commerce_row.id,
        &commerce_row.cnpj,
        &commerce_row.legal_name,
        &commerce_row.trade_name,
        auth.tenant_id,
        commerce_row.active,
    )
    .map_err(|_| ApiError::commerce_not_found())?;

    let address_row = infra_postgres::commerces::addresses::find_address_by_id(
        &state.app_pool,
        auth.tenant_id,
        delivery_address_id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::invalid_delivery_address)?;

    if address_row.commerce_id != commerce_id {
        return Err(ApiError::invalid_delivery_address());
    }

    let address = application::restore_commerce_address(
        &commerce,
        application::AddressRowInput {
            id: address_row.id,
            address_type: &address_row.address_type,
            street: &address_row.street,
            number: &address_row.number,
            district: address_row.district.as_deref(),
            city: &address_row.city,
            state: &address_row.state,
            postal_code: &address_row.postal_code,
            latitude: address_row.latitude,
            longitude: address_row.longitude,
            is_primary: address_row.is_primary,
        },
    )
    .map_err(|_| ApiError::invalid_delivery_address())?;

    let product_ids: Vec<Uuid> = items.iter().map(|line| line.product_id).collect();
    let product_rows = infra_postgres::inventory::find_products_by_ids(
        &state.app_pool,
        auth.tenant_id,
        &product_ids,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let mut lines = Vec::with_capacity(items.len());
    for line in items {
        let row = product_rows
            .iter()
            .find(|p| p.id == line.product_id)
            .ok_or_else(ApiError::product_not_found)?;
        if !row.active {
            return Err(ApiError::inactive_product());
        }
        let product = application::products::restore_product(
            row.id,
            &row.name,
            &row.sku,
            row.price_amount,
            &row.price_currency,
            auth.tenant_id,
            row.active,
            row.category.as_deref(),
            &row.unit_of_measure,
        )
        .map_err(|_| ApiError::internal())?;
        lines.push(PortalOrderLineInput {
            item_id: OrderItemId::generate(),
            product,
            quantity: line.quantity,
        });
    }

    Ok((commerce, address, lines))
}

async fn persist_new_order(
    state: &AppState,
    session: &SessionContext,
    order: &Order,
) -> Result<(), ApiError> {
    let (total_amount, total_currency) = order_total(order)?;
    let insert = OrderInsert {
        id: order.id().as_uuid(),
        commerce_id: order.commerce_id().as_uuid(),
        created_by_user_id: order.created_by().as_uuid(),
        source: order.source().as_str().to_owned(),
        status: order.status().as_str().to_owned(),
        delivery_address_id: order.delivery_address_id().as_uuid(),
        notes: order.notes().map(str::to_owned),
        total_amount,
        total_currency,
    };
    infra_postgres::orders::insert_order_with_items(
        &state.app_pool,
        session,
        &insert,
        &order_item_inserts(order),
    )
    .await
    .map_err(|_| ApiError::internal())
}

pub(crate) async fn load_order(
    state: &AppState,
    session: &SessionContext,
    order_id: Uuid,
) -> Result<Order, ApiError> {
    let detail = infra_postgres::orders::find_order_detail(&state.app_pool, session, order_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::order_not_found)?;
    let items = infra_postgres::orders::list_order_items(&state.app_pool, session, order_id)
        .await
        .map_err(|_| ApiError::internal())?;
    restore_order(session.tenant_id, &detail, &items).map_err(|_| ApiError::internal())
}

fn restore_order(
    tenant_id: domain_shared::TenantId,
    detail: &infra_postgres::orders::OrderDetailRow,
    items: &[infra_postgres::orders::OrderItemRow],
) -> Result<Order, domain_orders::OrderError> {
    let currency = Currency::brl();
    let restored_items: Vec<OrderItem> = items
        .iter()
        .map(|row| {
            Ok(OrderItem::restore(
                OrderItemId::from_uuid(row.id),
                ProductId::from_uuid(row.product_id),
                Quantity::of(row.quantity_requested)
                    .map_err(|_| domain_shared::DomainError::MoneyOverflow)?,
                None,
                Money::new(row.unit_price_amount, currency.clone())?,
                Money::new(row.line_total_amount, currency.clone())?,
            ))
        })
        .collect::<Result<Vec<_>, domain_shared::DomainError>>()
        .map_err(|_| domain_orders::OrderError::EmptyOrder)?;

    Ok(Order::restore(
        OrderId::from_uuid(detail.id),
        tenant_id,
        CommerceId::from_uuid(detail.commerce_id),
        UserId::from_uuid(detail.created_by_user_id),
        detail.source.parse()?,
        domain_commerces::CommerceAddressId::from_uuid(detail.delivery_address_id),
        detail.notes.clone(),
        detail.status.parse()?,
        detail.rejection_reason.clone(),
        restored_items,
    ))
}

fn order_item_inserts(order: &Order) -> Vec<OrderItemInsert> {
    order
        .items()
        .iter()
        .map(|item| OrderItemInsert {
            id: item.id().as_uuid(),
            order_id: order.id().as_uuid(),
            product_id: item.product_id().as_uuid(),
            quantity_requested: item.quantity_requested().value(),
            unit_price_amount: item.unit_price().amount_minor(),
            unit_price_currency: item.unit_price().currency().as_str().to_owned(),
            line_total_amount: item.line_total().amount_minor(),
        })
        .collect()
}

fn order_total(order: &Order) -> Result<(i64, String), ApiError> {
    let total = order.total().map_err(|_| ApiError::internal())?;
    Ok((total.amount_minor(), total.currency().as_str().to_owned()))
}

pub(crate) fn order_to_response(order: &Order) -> Result<PortalOrderResponse, ApiError> {
    let dto = order_to_dto(order).map_err(|_| ApiError::internal())?;
    Ok(PortalOrderResponse {
        id: dto.id,
        status: dto.status,
        delivery_address_id: dto.delivery_address_id,
        notes: dto.notes,
        total_amount: dto.total_amount,
        total_currency: dto.total_currency,
        rejection_reason: dto.rejection_reason,
        items: dto
            .items
            .into_iter()
            .map(|item| PortalOrderItemResponse {
                id: item.id,
                product_id: item.product_id,
                quantity: item.quantity,
                unit_price_amount: item.unit_price_amount,
                unit_price_currency: item.unit_price_currency,
                line_total_amount: item.line_total_amount,
            })
            .collect(),
    })
}

fn validate_order_lines(items: &[PortalOrderLineRequest]) -> Result<(), ApiError> {
    if items.is_empty() {
        return Err(ApiError::bad_request(
            "VALIDATION_ERROR",
            "At least one order item is required",
        ));
    }
    for line in items {
        if line.quantity <= 0 {
            return Err(ApiError::bad_request(
                "VALIDATION_ERROR",
                "Item quantity must be positive",
            ));
        }
    }
    Ok(())
}

pub(crate) fn map_order_error(err: application::orders::OrdersAppError) -> ApiError {
    match err {
        application::orders::OrdersAppError::Order(domain_orders::OrderError::EmptyOrder) => {
            ApiError::bad_request("VALIDATION_ERROR", "Order must have at least one item")
        }
        application::orders::OrdersAppError::Order(domain_orders::OrderError::InactiveProduct) => {
            ApiError::inactive_product()
        }
        application::orders::OrdersAppError::Order(
            domain_orders::OrderError::InvalidDeliveryAddress,
        )
        | application::orders::OrdersAppError::Order(domain_orders::OrderError::InactiveCommerce) => {
            ApiError::invalid_delivery_address()
        }
        application::orders::OrdersAppError::Order(
            domain_orders::OrderError::InvalidTransition { .. },
        ) => ApiError::invalid_order_transition(),
        application::orders::OrdersAppError::Order(
            domain_orders::OrderError::RejectionReasonRequired,
        ) => ApiError::rejection_reason_required(),
        application::orders::OrdersAppError::InsufficientAvailableStock => {
            ApiError::insufficient_stock()
        }
        _ => ApiError::internal(),
    }
}

pub(crate) fn map_postgres_order_error(err: infra_postgres::PostgresError) -> ApiError {
    match err {
        infra_postgres::PostgresError::Database(_) => ApiError::order_not_found(),
        _ => ApiError::internal(),
    }
}

use super::idempotency::{created_json_response, idempotency_key, replay_idempotency};
use application::sales::{CreateSaleCommand, CreateSaleLineInput};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::Response,
};
use domain_identity::UserId;
use domain_sales::{PaymentMethod, SaleId};
use infra_redis::IdempotencyRecord;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::sales::types::{
    CreateSaleRequest, map_products_app_error, map_sales_app_error, require_can_record_sale,
    sale_response_from_dto,
};
use crate::state::AppState;
use crate::validation::{ValidatedJson, to_json_bytes};

pub async fn create_sale(
    State(state): State<AppState>,
    auth: AuthUser,
    headers: HeaderMap,
    ValidatedJson(body): ValidatedJson<CreateSaleRequest>,
) -> Result<Response, ApiError> {
    require_can_record_sale(&auth)?;

    if body.items.is_empty() {
        return Err(ApiError::bad_request(
            "VALIDATION_ERROR",
            "At least one sale item is required",
        ));
    }

    if let Some(key) = idempotency_key(&headers)
        && let Some(record) = state.idempotency_store.get(auth.tenant_id, &key)
    {
        return replay_idempotency(record);
    }

    let payment_method = PaymentMethod::parse(&body.payment_method)
        .map_err(|_| ApiError::bad_request("VALIDATION_ERROR", "Invalid payment method"))?;
    let payment_method_db = payment_method.as_str().to_owned();

    let commerce_row = infra_postgres::commerces::find_commerce_by_id(
        &state.app_pool,
        auth.tenant_id,
        body.commerce_id,
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

    let product_ids: Vec<Uuid> = body.items.iter().map(|i| i.product_id).collect();
    let product_rows = infra_postgres::inventory::find_products_by_ids(
        &state.app_pool,
        auth.tenant_id,
        &product_ids,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let products: Vec<domain_inventory::Product> = product_rows
        .iter()
        .map(|row| {
            application::products::restore_product(
                row.id,
                &row.name,
                &row.sku,
                row.price_amount,
                &row.price_currency,
                auth.tenant_id,
                row.active,
                row.category_name.as_deref(),
                &row.unit_of_measure,
            )
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(map_products_app_error)?;

    let sale_id = SaleId::generate();
    let command = CreateSaleCommand {
        sale_id,
        driver_id: UserId::from_uuid(auth.user_id),
        commerce_id: body.commerce_id,
        payment_method,
        tenant_id: auth.tenant_id,
        items: body
            .items
            .iter()
            .map(|item| CreateSaleLineInput {
                product_id: item.product_id,
                quantity: item.quantity,
            })
            .collect(),
    };

    let sale = application::sales::create_sale(commerce, products, command)
        .map_err(map_sales_app_error)?;
    let dto = application::sales::sale_to_dto(&sale).map_err(|_| ApiError::internal())?;
    let response = sale_response_from_dto(&dto);

    let db_items: Vec<infra_postgres::sales::NewSaleItem> = sale
        .items()
        .iter()
        .map(|item| infra_postgres::sales::NewSaleItem {
            id: Uuid::now_v7(),
            sale_id: sale.id().as_uuid(),
            product_id: item.product_id().as_uuid(),
            quantity: item.quantity().value(),
            unit_price_amount: item.unit_price().amount_minor(),
            unit_price_currency: item.unit_price().currency().as_str().to_owned(),
            line_total_amount: item.line_total().amount_minor(),
        })
        .collect();

    infra_postgres::sales::insert_sale_with_items(
        &state.app_pool,
        auth.tenant_id,
        infra_postgres::sales::SaleInsert {
            sale_id: sale.id().as_uuid(),
            driver_id: auth.user_id,
            commerce_id: body.commerce_id,
            payment_method: payment_method_db,
            total_amount: dto.total.amount_minor(),
            total_currency: dto.total.currency().as_str().to_owned(),
            items: db_items,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let location = format!("/v1/sales/{}", sale.id());
    let body_bytes = to_json_bytes(&response)?;
    if let Some(key) = idempotency_key(&headers) {
        state.idempotency_store.put(
            auth.tenant_id,
            &key,
            IdempotencyRecord {
                status_code: http::StatusCode::CREATED.as_u16(),
                body: body_bytes.clone(),
                location: Some(location.clone()),
            },
        );
    }

    created_json_response(body_bytes, &location)
}

pub async fn confirm_sale(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<axum::Json<super::types::SaleResponse>, ApiError> {
    require_can_record_sale(&auth)?;

    let row = infra_postgres::sales::find_sale_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::sale_not_found)?;

    if auth.role != domain_identity::Role::Admin && row.driver_id != auth.user_id {
        return Err(ApiError::sale_not_found());
    }

    if row.status != "Pending" {
        return Err(ApiError::invalid_transition());
    }

    let item_rows = infra_postgres::sales::list_sale_items(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?;

    let items: Vec<infra_postgres::sales::ConfirmSaleItem> = item_rows
        .iter()
        .map(|item| infra_postgres::sales::ConfirmSaleItem {
            product_id: item.product_id,
            quantity: item.quantity,
        })
        .collect();

    infra_postgres::sales::confirm_sale_with_stock(
        &state.app_pool,
        auth.tenant_id,
        auth.user_id,
        id,
        &items,
    )
    .await
    .map_err(super::types::map_confirm_sale_error)?;

    super::query::get_sale(State(state), auth, Path(id)).await
}

pub async fn cancel_sale(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<axum::Json<super::types::SaleResponse>, ApiError> {
    require_can_record_sale(&auth)?;

    let row = infra_postgres::sales::find_sale_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::sale_not_found)?;

    if auth.role != domain_identity::Role::Admin && row.driver_id != auth.user_id {
        return Err(ApiError::sale_not_found());
    }

    if row.status != "Pending" {
        return Err(ApiError::invalid_transition());
    }

    let updated = infra_postgres::sales::cancel_sale_status(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?;
    if !updated {
        return Err(ApiError::invalid_transition());
    }

    super::query::get_sale(State(state), auth, Path(id)).await
}

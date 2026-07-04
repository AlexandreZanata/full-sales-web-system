use application::sales::{CreateSaleCommand, CreateSaleLineInput};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::Response,
};
use domain_identity::{Role, UserId};
use domain_sales::{PaymentMethod, SaleId, SaleStatus};
use infra_redis::IdempotencyRecord;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;
use crate::validation::{ValidatedJson, to_json_bytes};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateSaleItemRequest {
    #[serde(rename = "productId")]
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateSaleRequest {
    #[serde(rename = "commerceId")]
    pub commerce_id: Uuid,
    pub items: Vec<CreateSaleItemRequest>,
    #[serde(rename = "paymentMethod")]
    pub payment_method: String,
}

#[derive(Serialize)]
pub struct SaleItemResponse {
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
pub struct SaleResponse {
    pub id: Uuid,
    #[serde(rename = "commerceId")]
    pub commerce_id: Uuid,
    #[serde(rename = "driverId")]
    pub driver_id: Uuid,
    pub status: SaleStatus,
    #[serde(rename = "paymentMethod")]
    pub payment_method: PaymentMethod,
    #[serde(rename = "totalAmount")]
    pub total_amount: i64,
    #[serde(rename = "totalCurrency")]
    pub total_currency: String,
    pub items: Vec<SaleItemResponse>,
}

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
    let mut response_builder = Response::builder().status(StatusCode::CREATED);
    if let Ok(value) = HeaderValue::from_str(&location) {
        response_builder = response_builder.header(http::header::LOCATION, value);
    }

    let body_bytes = to_json_bytes(&response)?;
    if let Some(key) = idempotency_key(&headers) {
        state.idempotency_store.put(
            auth.tenant_id,
            &key,
            IdempotencyRecord {
                status_code: StatusCode::CREATED.as_u16(),
                body: body_bytes.clone(),
                location: Some(location),
            },
        );
    }

    response_builder
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(axum::body::Body::from(body_bytes))
        .map_err(|_| ApiError::internal())
}

pub async fn get_sale(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<SaleResponse>, ApiError> {
    let row = infra_postgres::sales::find_sale_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::sale_not_found)?;

    if auth.role != Role::Admin && row.driver_id != auth.user_id {
        return Err(ApiError::sale_not_found());
    }

    let items = infra_postgres::sales::list_sale_items(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?;

    let status = parse_sale_status(&row.status)?;
    let payment_method =
        PaymentMethod::parse(&row.payment_method).map_err(|_| ApiError::internal())?;

    Ok(Json(SaleResponse {
        id: row.id,
        commerce_id: row.commerce_id,
        driver_id: row.driver_id,
        status,
        payment_method,
        total_amount: row.total_amount,
        total_currency: row.total_currency,
        items: items
            .into_iter()
            .map(|item| SaleItemResponse {
                product_id: item.product_id,
                quantity: item.quantity,
                unit_price_amount: item.unit_price_amount,
                unit_price_currency: item.unit_price_currency,
                line_total_amount: item.line_total_amount,
            })
            .collect(),
    }))
}

pub async fn confirm_sale(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<SaleResponse>, ApiError> {
    require_can_record_sale(&auth)?;

    let row = infra_postgres::sales::find_sale_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::sale_not_found)?;

    if auth.role != Role::Admin && row.driver_id != auth.user_id {
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
    .map_err(map_confirm_sale_error)?;

    get_sale(State(state), auth, Path(id)).await
}

pub async fn cancel_sale(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<SaleResponse>, ApiError> {
    require_can_record_sale(&auth)?;

    let row = infra_postgres::sales::find_sale_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::sale_not_found)?;

    if auth.role != Role::Admin && row.driver_id != auth.user_id {
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

    get_sale(State(state), auth, Path(id)).await
}

fn sale_response_from_dto(dto: &application::sales::SaleDto) -> SaleResponse {
    SaleResponse {
        id: dto.id.as_uuid(),
        commerce_id: dto.commerce_id,
        driver_id: dto.driver_id.as_uuid(),
        status: dto.status,
        payment_method: dto.payment_method,
        total_amount: dto.total.amount_minor(),
        total_currency: dto.total.currency().as_str().to_owned(),
        items: dto
            .items
            .iter()
            .map(|item| SaleItemResponse {
                product_id: item.product_id,
                quantity: item.quantity,
                unit_price_amount: item.unit_price_amount,
                unit_price_currency: item.unit_price_currency.clone(),
                line_total_amount: item.line_total_amount,
            })
            .collect(),
    }
}

fn parse_sale_status(value: &str) -> Result<SaleStatus, ApiError> {
    match value {
        "Pending" => Ok(SaleStatus::Pending),
        "Confirmed" => Ok(SaleStatus::Confirmed),
        "Cancelled" => Ok(SaleStatus::Cancelled),
        _ => Err(ApiError::internal()),
    }
}

fn require_can_record_sale(auth: &AuthUser) -> Result<(), ApiError> {
    match auth.role {
        Role::Admin | Role::Driver | Role::Seller => Ok(()),
        Role::CommerceContact => Err(ApiError::forbidden()),
    }
}

fn idempotency_key(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Idempotency-Key")
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .filter(|k| !k.is_empty())
        .map(str::to_owned)
}

fn replay_idempotency(record: IdempotencyRecord) -> Result<Response, ApiError> {
    let mut builder = Response::builder().status(record.status_code);
    if let Some(location) = record.location
        && let Ok(value) = HeaderValue::from_str(&location)
    {
        builder = builder.header(http::header::LOCATION, value);
    }
    builder
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(axum::body::Body::from(record.body))
        .map_err(|_| ApiError::internal())
}

fn map_confirm_sale_error(err: infra_postgres::sales::ConfirmSaleError) -> ApiError {
    match err {
        infra_postgres::sales::ConfirmSaleError::InsufficientStock => {
            ApiError::insufficient_stock()
        }
        infra_postgres::sales::ConfirmSaleError::InvalidTransition => {
            ApiError::invalid_transition()
        }
        infra_postgres::sales::ConfirmSaleError::Database(_) => ApiError::internal(),
    }
}

fn map_sales_app_error(err: application::sales::SalesAppError) -> ApiError {
    match err {
        application::sales::SalesAppError::Sale(domain_sales::SaleError::InactiveCommerce) => {
            ApiError::inactive_commerce()
        }
        application::sales::SalesAppError::Sale(domain_sales::SaleError::InactiveProduct) => {
            ApiError::inactive_product()
        }
        application::sales::SalesAppError::Inventory(
            domain_inventory::InventoryError::InvalidQuantity,
        ) => ApiError::bad_request("VALIDATION_ERROR", "Quantity must be a positive integer"),
        application::sales::SalesAppError::ProductNotFound => ApiError::product_not_found(),
        application::sales::SalesAppError::CommerceNotFound => ApiError::commerce_not_found(),
        application::sales::SalesAppError::SaleNotFound => ApiError::sale_not_found(),
        application::sales::SalesAppError::InsufficientStock => ApiError::insufficient_stock(),
        application::sales::SalesAppError::Sale(domain_sales::SaleError::InvalidTransition {
            ..
        }) => ApiError::invalid_transition(),
        _ => ApiError::bad_request("VALIDATION_ERROR", "Invalid sale request"),
    }
}

fn map_products_app_error(err: application::products::ProductsAppError) -> ApiError {
    match err {
        application::products::ProductsAppError::Inventory(_) => {
            ApiError::bad_request("VALIDATION_ERROR", "Invalid product data")
        }
        application::products::ProductsAppError::Domain(_) => ApiError::internal(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_create_sale_request_schema_when_unknown_field_then_denied() {
        let json = r#"{"commerceId":"550e8400-e29b-41d4-a716-446655440000","items":[],"paymentMethod":"cash","extra":1}"#;
        let result = serde_json::from_str::<CreateSaleRequest>(json);
        assert!(result.is_err());
    }
}

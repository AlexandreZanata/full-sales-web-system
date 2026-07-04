use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::products::{
    ProductResponse, product_response_from_row, require_can_read_products,
    require_can_write_products,
};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub sku: String,
    #[serde(rename = "priceAmount")]
    pub price_amount: i64,
    #[serde(rename = "priceCurrency", default = "default_currency")]
    pub price_currency: String,
    pub category: Option<String>,
    #[serde(rename = "unitOfMeasure", default = "default_uom")]
    pub unit_of_measure: String,
}

#[derive(Deserialize, Default)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    #[serde(rename = "priceAmount")]
    pub price_amount: Option<i64>,
    #[serde(rename = "priceCurrency")]
    pub price_currency: Option<String>,
    pub active: Option<bool>,
    pub category: Option<String>,
    #[serde(rename = "unitOfMeasure")]
    pub unit_of_measure: Option<String>,
}

#[derive(Serialize)]
pub struct ProductDetailResponse {
    #[serde(flatten)]
    pub product: ProductResponse,
    pub category: Option<String>,
    #[serde(rename = "unitOfMeasure")]
    pub unit_of_measure: String,
}

fn default_currency() -> String {
    "BRL".into()
}

fn default_uom() -> String {
    "Unit".into()
}

pub async fn create_product(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<ProductDetailResponse>), ApiError> {
    require_can_write_products(&auth)?;
    let product_id = Uuid::now_v7();
    let _ = application::products::restore_product(
        product_id,
        &body.name,
        &body.sku,
        body.price_amount,
        &body.price_currency,
        auth.tenant_id,
        true,
        body.category.as_deref(),
        &body.unit_of_measure,
    )
    .map_err(map_products_error)?;

    infra_postgres::inventory::insert_product_with_catalog(
        &state.app_pool,
        auth.tenant_id,
        infra_postgres::inventory::ProductInsert {
            id: product_id,
            sku: body.sku,
            name: body.name,
            price_amount: body.price_amount,
            price_currency: body.price_currency.clone(),
            category: body.category.clone(),
            unit_of_measure: body.unit_of_measure.clone(),
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;

    get_product(State(state), auth, Path(product_id))
        .await
        .map(|json| (StatusCode::CREATED, json))
}

pub async fn get_product(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ProductDetailResponse>, ApiError> {
    require_can_read_products(&auth)?;
    let row = infra_postgres::inventory::find_product_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::product_not_found)?;
    Ok(Json(product_detail_from_row(&row)))
}

pub async fn update_product(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateProductRequest>,
) -> Result<Json<ProductDetailResponse>, ApiError> {
    require_can_write_products(&auth)?;
    let updated = infra_postgres::inventory::update_product(
        &state.app_pool,
        auth.tenant_id,
        id,
        &infra_postgres::inventory::ProductUpdate {
            name: body.name,
            price_amount: body.price_amount,
            price_currency: body.price_currency,
            active: body.active,
            category: body.category,
            unit_of_measure: body.unit_of_measure,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;
    if !updated {
        return Err(ApiError::product_not_found());
    }
    get_product(State(state), auth, Path(id)).await
}

pub(crate) async fn ensure_product(
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

fn product_detail_from_row(row: &infra_postgres::inventory::ProductRow) -> ProductDetailResponse {
    ProductDetailResponse {
        product: product_response_from_row(row),
        category: row.category.clone(),
        unit_of_measure: row.unit_of_measure.clone(),
    }
}

fn map_products_error(err: application::products::ProductsAppError) -> ApiError {
    match err {
        application::products::ProductsAppError::Inventory(_) => {
            ApiError::bad_request("VALIDATION_ERROR", "Invalid product data")
        }
        application::products::ProductsAppError::Domain(_) => ApiError::internal(),
    }
}

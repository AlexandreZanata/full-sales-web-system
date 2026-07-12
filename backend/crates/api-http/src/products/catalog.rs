use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::products::{ProductResponse, require_can_read_products, require_can_write_products};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub sku: String,
    #[serde(rename = "priceAmount")]
    pub price_amount: i64,
    #[serde(rename = "priceCurrency", default = "default_currency")]
    pub price_currency: String,
    #[serde(rename = "compareAtPrice")]
    pub compare_at_price: Option<i64>,
    #[serde(rename = "categoryId")]
    pub category_id: Option<Uuid>,
    #[serde(rename = "unitOfMeasure", default = "default_uom")]
    pub unit_of_measure: String,
    pub description: Option<String>,
    #[serde(rename = "category")]
    pub legacy_category: Option<String>,
}

#[derive(Deserialize, Default)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    #[serde(rename = "priceAmount")]
    pub price_amount: Option<i64>,
    #[serde(rename = "priceCurrency")]
    pub price_currency: Option<String>,
    #[serde(rename = "compareAtPrice")]
    pub compare_at_price: Option<Option<i64>>,
    pub active: Option<bool>,
    #[serde(rename = "categoryId")]
    pub category_id: Option<Option<Uuid>>,
    #[serde(rename = "unitOfMeasure")]
    pub unit_of_measure: Option<String>,
    pub description: Option<Option<String>>,
    #[serde(rename = "isFeatured")]
    pub is_featured: Option<bool>,
    #[serde(rename = "category")]
    pub legacy_category: Option<String>,
}

#[derive(Serialize)]
pub struct ProductDetailResponse {
    #[serde(flatten)]
    pub product: ProductResponse,
    #[serde(rename = "categoryId", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<Uuid>,
    #[serde(rename = "categoryName", skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    #[serde(rename = "categorySlug", skip_serializing_if = "Option::is_none")]
    pub category_slug: Option<String>,
    #[serde(rename = "unitOfMeasure")]
    pub unit_of_measure: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
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
    if body.legacy_category.is_some() {
        return Err(ApiError::bad_request(
            "VALIDATION_ERROR",
            "Use categoryId instead of category",
        ));
    }
    if let Some(category_id) = body.category_id {
        ensure_category(&state, auth.tenant_id, category_id).await?;
    }

    if let Some(description) = body.description.as_deref() {
        validate_description(description)?;
    }
    validate_compare_at_price(body.price_amount, body.compare_at_price)?;

    let product_id = Uuid::now_v7();
    let _ = application::products::restore_product(
        product_id,
        &body.name,
        &body.sku,
        body.price_amount,
        &body.price_currency,
        auth.tenant_id,
        true,
        None,
        &body.unit_of_measure,
    )
    .map_err(map_products_error)?;

    infra_postgres::inventory::insert_product_with_catalog(
        &state.app_pool,
        auth.tenant_id,
        infra_postgres::inventory::ProductInsert {
            id: product_id,
            sku: body.sku.clone(),
            name: body.name,
            price_amount: body.price_amount,
            price_currency: body.price_currency.clone(),
            compare_at_price: body.compare_at_price,
            category_id: body.category_id,
            unit_of_measure: body.unit_of_measure.clone(),
            description: normalize_description(body.description.as_deref()),
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;

    crate::catalog_events::notify_product_changed(
        &state.catalog_events,
        "created",
        product_id,
        &body.sku,
    );

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
    let images =
        super::primary_image::load_primary_images_by_product_id(&state, auth.tenant_id, &[id])
            .await?;
    Ok(Json(product_detail_from_row(
        &row,
        images.get(&id).cloned(),
    )))
}

pub async fn update_product(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateProductRequest>,
) -> Result<Json<ProductDetailResponse>, ApiError> {
    require_can_write_products(&auth)?;
    if body.legacy_category.is_some() {
        return Err(ApiError::bad_request(
            "VALIDATION_ERROR",
            "Use categoryId instead of category",
        ));
    }
    if let Some(Some(category_id)) = body.category_id {
        ensure_category(&state, auth.tenant_id, category_id).await?;
    }

    if let Some(Some(description)) = body.description.as_ref() {
        validate_description(description)?;
    }

    let existing =
        infra_postgres::inventory::find_product_by_id(&state.app_pool, auth.tenant_id, id)
            .await
            .map_err(|_| ApiError::internal())?
            .ok_or_else(ApiError::product_not_found)?;
    let next_price = body.price_amount.unwrap_or(existing.price_amount);
    let next_compare = if body.compare_at_price.is_some() {
        body.compare_at_price.as_ref().and_then(|value| *value)
    } else {
        existing.compare_at_price
    };
    validate_compare_at_price(next_price, next_compare)?;

    let updated = infra_postgres::inventory::update_product(
        &state.app_pool,
        auth.tenant_id,
        id,
        &infra_postgres::inventory::ProductUpdate {
            name: body.name,
            price_amount: body.price_amount,
            price_currency: body.price_currency,
            compare_at_price: body.compare_at_price,
            active: body.active,
            category_id: body.category_id,
            unit_of_measure: body.unit_of_measure,
            description: body
                .description
                .as_ref()
                .map(|value| normalize_description(value.as_deref())),
            is_featured: body.is_featured,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;
    if !updated {
        return Err(ApiError::product_not_found());
    }
    let row = infra_postgres::inventory::find_product_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::product_not_found)?;
    crate::catalog_events::notify_product_changed(&state.catalog_events, "updated", id, &row.sku);
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

async fn ensure_category(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
    category_id: Uuid,
) -> Result<(), ApiError> {
    let exists = infra_postgres::inventory::product_categories::find_category_by_id(
        &state.app_pool,
        tenant_id,
        category_id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .is_some();
    if exists {
        Ok(())
    } else {
        Err(ApiError::not_found_with_code(
            "CATEGORY_NOT_FOUND",
            "Category not found",
        ))
    }
}

fn product_detail_from_row(
    row: &infra_postgres::inventory::ProductRow,
    primary_image: Option<super::primary_image::PrimaryImageRef>,
) -> ProductDetailResponse {
    ProductDetailResponse {
        product: super::product_response_from_row(row, primary_image),
        category_id: row.category_id,
        category_name: row.category_name.clone(),
        category_slug: row.category_slug.clone(),
        unit_of_measure: row.unit_of_measure.clone(),
        description: row.description.clone(),
    }
}

fn validate_compare_at_price(
    price_amount: i64,
    compare_at_price: Option<i64>,
) -> Result<(), ApiError> {
    if let Some(compare_at) = compare_at_price
        && compare_at <= price_amount
    {
        return Err(ApiError::bad_request(
            "VALIDATION_ERROR",
            "compareAtPrice must be greater than priceAmount",
        ));
    }
    Ok(())
}

fn validate_description(description: &str) -> Result<(), ApiError> {
    if description.chars().count() > 2_000 {
        return Err(ApiError::bad_request(
            "VALIDATION_ERROR",
            "Description must be at most 2000 characters",
        ));
    }
    Ok(())
}

fn normalize_description(description: Option<&str>) -> Option<String> {
    description
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}

fn map_products_error(err: application::products::ProductsAppError) -> ApiError {
    match err {
        application::products::ProductsAppError::Inventory(_) => {
            ApiError::bad_request("VALIDATION_ERROR", "Invalid product data")
        }
        application::products::ProductsAppError::Domain(_) => ApiError::internal(),
    }
}

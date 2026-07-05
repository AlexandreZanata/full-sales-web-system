use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::products::require_can_write_products;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct AttachImageRequest {
    #[serde(rename = "fileId")]
    pub file_id: Uuid,
    #[serde(rename = "isPrimary", default)]
    pub is_primary: bool,
    #[serde(rename = "sortOrder", default)]
    pub sort_order: Option<i32>,
}

#[derive(Serialize)]
pub struct ProductImageResponse {
    pub id: Uuid,
    #[serde(rename = "fileId")]
    pub file_id: Uuid,
    #[serde(rename = "sortOrder")]
    pub sort_order: i32,
    #[serde(rename = "isPrimary")]
    pub is_primary: bool,
}

#[derive(Serialize)]
pub struct ProductImagesListResponse {
    pub items: Vec<ProductImageResponse>,
}

pub async fn list_product_images(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(product_id): Path<Uuid>,
) -> Result<Json<ProductImagesListResponse>, ApiError> {
    require_can_write_products(&auth)?;
    super::catalog::ensure_product(&state, auth.tenant_id, product_id).await?;

    let rows = infra_postgres::inventory::product_images::list_product_images(
        &state.app_pool,
        auth.tenant_id,
        product_id,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    Ok(Json(ProductImagesListResponse {
        items: rows.iter().map(product_image_response).collect(),
    }))
}

pub async fn attach_product_image(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(product_id): Path<Uuid>,
    Json(body): Json<AttachImageRequest>,
) -> Result<(StatusCode, Json<ProductImageResponse>), ApiError> {
    require_can_write_products(&auth)?;
    super::catalog::ensure_product(&state, auth.tenant_id, product_id).await?;

    if body.is_primary {
        infra_postgres::inventory::product_images::clear_primary_for_product(
            &state.app_pool,
            auth.tenant_id,
            product_id,
        )
        .await
        .map_err(|_| ApiError::internal())?;
    }

    let image_id = Uuid::now_v7();
    infra_postgres::inventory::product_images::insert_product_image(
        &state.app_pool,
        auth.tenant_id,
        infra_postgres::inventory::product_images::ProductImageInsert {
            id: image_id,
            product_id,
            file_id: body.file_id,
            sort_order: body.sort_order.unwrap_or(0),
            is_primary: body.is_primary,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let row = infra_postgres::inventory::product_images::find_product_image_by_id(
        &state.app_pool,
        auth.tenant_id,
        image_id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::internal)?;

    notify_product_image_changed(&state, auth.tenant_id, product_id).await?;

    Ok((StatusCode::CREATED, Json(product_image_response(&row))))
}

pub async fn delete_product_image(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((product_id, image_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, ApiError> {
    require_can_write_products(&auth)?;
    super::catalog::ensure_product(&state, auth.tenant_id, product_id).await?;
    let image = infra_postgres::inventory::product_images::find_product_image_by_id(
        &state.app_pool,
        auth.tenant_id,
        image_id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::media_not_found)?;

    if image.product_id != product_id {
        return Err(ApiError::media_not_found());
    }

    let deleted = infra_postgres::inventory::product_images::delete_product_image(
        &state.app_pool,
        auth.tenant_id,
        image_id,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    if !deleted {
        return Err(ApiError::media_not_found());
    }
    notify_product_image_changed(&state, auth.tenant_id, product_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn notify_product_image_changed(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
    product_id: Uuid,
) -> Result<(), ApiError> {
    let row = infra_postgres::inventory::find_product_by_id(&state.app_pool, tenant_id, product_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::product_not_found)?;
    crate::catalog_events::notify_product_changed(
        &state.catalog_events,
        "updated",
        product_id,
        &row.sku,
    );
    Ok(())
}

fn product_image_response(
    row: &infra_postgres::inventory::product_images::ProductImageRow,
) -> ProductImageResponse {
    ProductImageResponse {
        id: row.id,
        file_id: row.file_id,
        sort_order: row.sort_order,
        is_primary: row.is_primary,
    }
}

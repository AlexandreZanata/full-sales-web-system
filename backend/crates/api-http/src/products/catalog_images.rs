use axum::{
    Json,
    extract::{Path, RawQuery, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::list_query::{
    CursorListResponse, PRODUCT_IMAGES_LIST_CONFIG, build_cursor_page, decode_query_pairs,
    parse_list_query,
};
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

pub async fn list_product_images(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(product_id): Path<Uuid>,
    RawQuery(query): RawQuery,
) -> Result<Json<CursorListResponse<ProductImageResponse>>, Response> {
    require_can_write_products(&auth).map_err(IntoResponse::into_response)?;
    super::catalog::ensure_product(&state, auth.tenant_id, product_id)
        .await
        .map_err(IntoResponse::into_response)?;
    let parsed = parse_list_query(
        &decode_query_pairs(query.as_deref()),
        &PRODUCT_IMAGES_LIST_CONFIG,
    )
    .map_err(IntoResponse::into_response)?;

    let rows = infra_postgres::inventory::product_images::list_product_images_cursor(
        &state.app_pool,
        auth.tenant_id,
        product_id,
        parsed.pagination.cursor,
        parsed.pagination.fetch_size() as i64,
    )
    .await
    .map_err(|_| IntoResponse::into_response(ApiError::internal()))?;

    let items: Vec<ProductImageResponse> = rows.iter().map(product_image_response).collect();
    Ok(Json(build_cursor_page(
        items,
        parsed.pagination.limit,
        |image| image.id,
    )))
}

pub async fn attach_product_image(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(product_id): Path<Uuid>,
    Json(body): Json<AttachImageRequest>,
) -> Result<(StatusCode, Json<ProductImageResponse>), ApiError> {
    require_can_write_products(&auth)?;
    super::catalog::ensure_product(&state, auth.tenant_id, product_id).await?;

    let file = infra_postgres::media::find_file_by_id(&state.app_pool, auth.tenant_id, body.file_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::media_not_found)?;
    if file.entity_type != "Product" || file.entity_id != product_id {
        return Err(ApiError::bad_request(
            "VALIDATION_ERROR",
            "fileId does not belong to this product",
        ));
    }

    let existing = infra_postgres::inventory::product_images::list_product_images(
        &state.app_pool,
        auth.tenant_id,
        product_id,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    let has_primary = existing.iter().any(|row| row.is_primary);
    let is_primary = body.is_primary || !has_primary;

    if is_primary {
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
            is_primary,
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

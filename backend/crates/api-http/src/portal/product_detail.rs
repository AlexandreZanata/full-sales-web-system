use std::time::Duration;

use axum::{
    Json,
    extract::{Path, State},
};
use infra_storage::object_storage::DEFAULT_PRESIGN_TTL_SECS;
use serde::Serialize;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;

use super::products::{portal_product_from_row, require_commerce_contact, resolve_public_catalog_tenant};

#[derive(Serialize)]
pub struct PortalProductDetailResponse {
    #[serde(flatten)]
    pub product: super::products::PortalProductResponse,
    #[serde(rename = "unitOfMeasure")]
    pub unit_of_measure: String,
    #[serde(rename = "imageUrls")]
    pub image_urls: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

pub async fn get_portal_product_by_id(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<PortalProductDetailResponse>, ApiError> {
    let _ = require_commerce_contact(&auth)?;
    get_product_detail_for_tenant(&state, auth.tenant_id, id).await
}

pub async fn get_public_product_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PortalProductDetailResponse>, ApiError> {
    let tenant_id = resolve_public_catalog_tenant()?;
    get_product_detail_for_tenant(&state, tenant_id, id).await
}

async fn get_product_detail_for_tenant(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
    id: Uuid,
) -> Result<Json<PortalProductDetailResponse>, ApiError> {
    let row =
        infra_postgres::inventory::find_portal_product_by_id(&state.app_pool, tenant_id, id)
            .await
            .map_err(|_| ApiError::internal())?
            .ok_or_else(ApiError::product_not_found)?;

    let gallery = infra_postgres::inventory::product_images::find_gallery_images_for_product(
        &state.app_pool,
        tenant_id,
        id,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    Ok(Json(
        build_portal_product_detail(state, &row, &gallery).await?,
    ))
}

async fn build_portal_product_detail(
    state: &AppState,
    row: &infra_postgres::inventory::ProductRow,
    gallery: &[infra_postgres::inventory::product_images::ProductGalleryImageRow],
) -> Result<PortalProductDetailResponse, ApiError> {
    let ttl = Duration::from_secs(DEFAULT_PRESIGN_TTL_SECS);
    let (primary_file_id, primary_image_url) =
        resolve_primary_image(state, gallery, ttl).await;
    let mut image_urls = Vec::new();

    for image in gallery {
        if primary_file_id == Some(image.file_id) {
            continue;
        }
        if let Some(url) = presign_gallery_url(state, image, ttl).await {
            image_urls.push(url);
        }
    }

    Ok(PortalProductDetailResponse {
        product: portal_product_from_row(row, primary_image_url),
        unit_of_measure: row.unit_of_measure.clone(),
        image_urls,
        description: row.description.clone(),
    })
}

async fn resolve_primary_image(
    state: &AppState,
    gallery: &[infra_postgres::inventory::product_images::ProductGalleryImageRow],
    ttl: Duration,
) -> (Option<Uuid>, Option<String>) {
    let primary = gallery
        .iter()
        .find(|image| image.is_primary)
        .or_else(|| gallery.first());

    let Some(image) = primary else {
        return (None, None);
    };

    let url = presign_gallery_url(state, image, ttl).await;
    (Some(image.file_id), url)
}

async fn presign_gallery_url(
    state: &AppState,
    image: &infra_postgres::inventory::product_images::ProductGalleryImageRow,
    ttl: Duration,
) -> Option<String> {
    state
        .storage
        .presigned_get(&image.bucket, &image.object_key, ttl)
        .await
        .ok()
        .map(|presigned| crate::media::catalog_image_url(image.file_id, &presigned.url))
}

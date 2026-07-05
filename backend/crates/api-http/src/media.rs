mod support;

use axum::{
    Json,
    body::Body,
    extract::{Multipart, Path, State},
    http::{HeaderValue, header},
    response::Response,
};
use domain_media::{FileId, compute_sha256, validate_upload};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;

pub use support::{MediaUploadResponse, MediaUrlResponse, authenticated_media_content_url, catalog_image_url};

pub async fn upload_media(
    State(state): State<AppState>,
    auth: AuthUser,
    mut multipart: Multipart,
) -> Result<Json<MediaUploadResponse>, ApiError> {
    let (bytes, mime_type, entity_type, entity_id) =
        support::parse_multipart(&mut multipart).await?;
    support::ensure_can_access_entity(&state, &auth, entity_type, entity_id).await?;

    let size_bytes = bytes.len() as u64;
    let sha256 = compute_sha256(&bytes);
    validate_upload(&mime_type, size_bytes, &bytes, &sha256).map_err(support::map_media_error)?;

    let file_id = FileId::generate();
    let bucket = support::media_bucket();
    let object_key = support::object_key_for(entity_type, entity_id, file_id.as_uuid(), &mime_type);

    state
        .storage
        .put_object(&bucket, &object_key, &bytes, &mime_type)
        .await
        .map_err(|_| ApiError::internal())?;

    infra_postgres::media::insert_file(
        &state.app_pool,
        auth.tenant_id,
        infra_postgres::media::FileInsert {
            id: file_id.as_uuid(),
            entity_type: entity_type.as_str().to_owned(),
            entity_id,
            bucket,
            object_key,
            mime_type: mime_type.clone(),
            size_bytes: size_bytes as i64,
            sha256: sha256.clone(),
            uploaded_by_user_id: auth.user_id,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;

    Ok(Json(MediaUploadResponse {
        id: file_id.as_uuid(),
        entity_type: entity_type.as_str().to_owned(),
        entity_id,
        mime_type,
        size_bytes,
        sha256,
    }))
}

pub async fn get_media_url(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<MediaUrlResponse>, ApiError> {
    let row = infra_postgres::media::find_file_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::media_not_found)?;

    let entity_type = support::parse_entity_type(&row.entity_type)?;
    support::ensure_can_access_entity(&state, &auth, entity_type, row.entity_id).await?;

    let presigned = support::presign(&state, &row.bucket, &row.object_key).await?;
    Ok(Json(MediaUrlResponse {
        url: support::authenticated_media_content_url(id, &presigned.url),
        expires_at: presigned.expires_at,
    }))
}

pub async fn get_media_content(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Response, ApiError> {
    let row = infra_postgres::media::find_file_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::media_not_found)?;

    let entity_type = support::parse_entity_type(&row.entity_type)?;
    support::ensure_can_access_entity(&state, &auth, entity_type, row.entity_id).await?;

    let (bytes, content_type) = state
        .storage
        .get_object(&row.bucket, &row.object_key)
        .await
        .map_err(support::map_storage_error)?;

    let header_value = HeaderValue::from_str(&content_type)
        .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream"));

    Response::builder()
        .header(header::CONTENT_TYPE, header_value)
        .body(Body::from(bytes))
        .map_err(|_| ApiError::internal())
}

pub async fn presign_object(
    state: &AppState,
    bucket: &str,
    object_key: &str,
) -> Result<MediaUrlResponse, ApiError> {
    support::presign(state, bucket, object_key).await
}

pub async fn get_public_product_media_content(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Response, ApiError> {
    let tenant_id = crate::portal::resolve_public_catalog_tenant()?;
    let row = infra_postgres::inventory::product_images::find_active_product_media(
        &state.app_pool,
        tenant_id,
        id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::media_not_found)?;

    let (bytes, content_type) = state
        .storage
        .get_object(&row.bucket, &row.object_key)
        .await
        .map_err(support::map_storage_error)?;

    let resolved_type = if content_type.is_empty() {
        row.mime_type
    } else {
        content_type
    };
    let header_value = HeaderValue::from_str(&resolved_type)
        .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream"));

    Response::builder()
        .header(header::CONTENT_TYPE, header_value)
        .header(header::CACHE_CONTROL, "public, max-age=300")
        .body(Body::from(bytes))
        .map_err(|_| ApiError::internal())
}

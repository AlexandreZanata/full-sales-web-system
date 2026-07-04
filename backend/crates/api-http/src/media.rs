mod support;

use axum::{
    Json,
    extract::{Multipart, Path, State},
};
use domain_media::{compute_sha256, validate_upload, FileId};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;

pub use support::{MediaUploadResponse, MediaUrlResponse};

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
    Ok(Json(presigned))
}

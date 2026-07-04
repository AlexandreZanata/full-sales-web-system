use std::str::FromStr;
use std::time::Duration;

use axum::extract::Multipart;
use domain_identity::Role;
use domain_media::{FileEntityType, MediaError};
use infra_storage::object_storage::DEFAULT_PRESIGN_TTL_SECS;
use serde::Serialize;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::session::session_from_auth;
use crate::state::AppState;

#[derive(Serialize)]
pub struct MediaUploadResponse {
    pub id: Uuid,
    #[serde(rename = "entityType")]
    pub entity_type: String,
    #[serde(rename = "entityId")]
    pub entity_id: Uuid,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(rename = "sizeBytes")]
    pub size_bytes: u64,
    pub sha256: String,
}

#[derive(Serialize)]
pub struct MediaUrlResponse {
    pub url: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

const MEDIA_BUCKET_ENV: &str = "MEDIA_BUCKET";

pub async fn parse_multipart(
    multipart: &mut Multipart,
) -> Result<(Vec<u8>, String, FileEntityType, Uuid), ApiError> {
    let mut bytes: Option<Vec<u8>> = None;
    let mut mime_type: Option<String> = None;
    let mut entity_type: Option<String> = None;
    let mut entity_id: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| ApiError::bad_request("VALIDATION_ERROR", "Invalid multipart body"))?
    {
        match field.name() {
            Some("file") => {
                mime_type = field.content_type().map(str::to_owned);
                bytes = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|_| ApiError::bad_request("VALIDATION_ERROR", "Invalid file field"))?
                        .to_vec(),
                );
            }
            Some("entityType") => {
                entity_type = Some(field.text().await.map_err(|_| {
                    ApiError::bad_request("VALIDATION_ERROR", "Invalid entityType field")
                })?);
            }
            Some("entityId") => {
                entity_id = Some(field.text().await.map_err(|_| {
                    ApiError::bad_request("VALIDATION_ERROR", "Invalid entityId field")
                })?);
            }
            _ => {}
        }
    }

    Ok((
        bytes.ok_or_else(|| ApiError::bad_request("VALIDATION_ERROR", "file is required"))?,
        mime_type.unwrap_or_else(|| "application/octet-stream".to_owned()),
        FileEntityType::from_str(
            &entity_type
                .ok_or_else(|| ApiError::bad_request("VALIDATION_ERROR", "entityType is required"))?,
        )
        .map_err(map_media_error)?,
        Uuid::parse_str(
            &entity_id.ok_or_else(|| ApiError::bad_request("VALIDATION_ERROR", "entityId is required"))?,
        )
        .map_err(|_| ApiError::bad_request("VALIDATION_ERROR", "Invalid entityId"))?,
    ))
}

pub fn parse_entity_type(value: &str) -> Result<FileEntityType, ApiError> {
    FileEntityType::from_str(value).map_err(map_media_error)
}

pub async fn ensure_can_access_entity(
    state: &AppState,
    auth: &AuthUser,
    entity_type: FileEntityType,
    entity_id: Uuid,
) -> Result<(), ApiError> {
    if auth.role == Role::Admin {
        return Ok(());
    }
    match entity_type {
        FileEntityType::User if auth.user_id == entity_id => Ok(()),
        FileEntityType::Commerce
            if auth.role == Role::CommerceContact && auth.commerce_id == Some(entity_id) =>
        {
            Ok(())
        }
        FileEntityType::Product if matches!(auth.role, Role::Seller | Role::Driver) => Ok(()),
        FileEntityType::Delivery if auth.role == Role::Driver => {
            let delivery = infra_postgres::deliveries::find_delivery_by_id(
                &state.app_pool,
                &session_from_auth(auth),
                entity_id,
            )
            .await
            .map_err(|_| ApiError::internal())?
            .ok_or_else(ApiError::forbidden)?;
            if delivery.driver_id == auth.user_id {
                Ok(())
            } else {
                Err(ApiError::forbidden())
            }
        }
        _ => Err(ApiError::forbidden()),
    }
}

pub async fn presign(
    state: &AppState,
    bucket: &str,
    object_key: &str,
) -> Result<MediaUrlResponse, ApiError> {
    let ttl = Duration::from_secs(DEFAULT_PRESIGN_TTL_SECS);
    let presigned = state
        .storage
        .presigned_get(bucket, object_key, ttl)
        .await
        .map_err(|_| ApiError::internal())?;
    Ok(MediaUrlResponse {
        url: presigned.url,
        expires_at: chrono::Utc::now() + chrono::Duration::seconds(presigned.expires_in_secs as i64),
    })
}

pub fn media_bucket() -> String {
    std::env::var(MEDIA_BUCKET_ENV).unwrap_or_else(|_| "media".into())
}

pub fn object_key_for(entity_type: FileEntityType, entity_id: Uuid, file_id: Uuid, mime: &str) -> String {
    let ext = match mime {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/webp" => "webp",
        _ => "bin",
    };
    format!(
        "{}/{}/{}.{}",
        entity_type.as_str().to_lowercase(),
        entity_id,
        file_id,
        ext
    )
}

pub fn map_media_error(err: MediaError) -> ApiError {
    match err {
        MediaError::InvalidMimeType => ApiError::invalid_mime(),
        MediaError::FileTooLarge => ApiError::file_too_large(),
        MediaError::InvalidEntityType => {
            ApiError::bad_request("VALIDATION_ERROR", "Invalid entityType")
        }
        _ => ApiError::bad_request("VALIDATION_ERROR", "Invalid media upload"),
    }
}

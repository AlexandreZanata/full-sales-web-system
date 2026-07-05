use axum::{
    Json,
    extract::State,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::{AuthUser, require_admin};
use crate::error::ApiError;
use crate::media;
use crate::state::AppState;

#[derive(Serialize)]
pub struct SettingsResponse {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "logoFileId", skip_serializing_if = "Option::is_none")]
    pub logo_file_id: Option<Uuid>,
    #[serde(rename = "logoUrl", skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
}

#[derive(Deserialize, Default)]
pub struct UpdateSettingsRequest {
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateLogoRequest {
    #[serde(rename = "fileId")]
    pub file_id: Uuid,
}

pub async fn get_settings(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<SettingsResponse>, ApiError> {
    settings_response(&state, auth.tenant_id).await
}

pub async fn patch_settings(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<UpdateSettingsRequest>,
) -> Result<Json<SettingsResponse>, ApiError> {
    require_admin(&auth)?;
    let Some(display_name) = body.display_name else {
        return settings_response(&state, auth.tenant_id).await;
    };
    let trimmed = display_name.trim();
    if trimmed.is_empty() || trimmed.len() > 200 {
        return Err(ApiError::bad_request(
            "VALIDATION_ERROR",
            "displayName must be 1–200 characters",
        ));
    }

    let updated = infra_postgres::shared::update_tenant_display_name(
        &state.app_pool,
        auth.tenant_id,
        trimmed,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    if !updated {
        return Err(ApiError::not_found());
    }

    settings_response(&state, auth.tenant_id).await
}

pub async fn update_site_logo(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<UpdateLogoRequest>,
) -> Result<Json<SettingsResponse>, ApiError> {
    require_admin(&auth)?;
    let file = infra_postgres::media::find_file_by_id(&state.app_pool, auth.tenant_id, body.file_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::media_not_found)?;

    if file.entity_type != "Tenant" || file.entity_id != auth.tenant_id.as_uuid() {
        return Err(ApiError::media_not_found());
    }

    let updated = infra_postgres::shared::update_tenant_logo(
        &state.app_pool,
        auth.tenant_id,
        Some(body.file_id),
    )
    .await
    .map_err(|_| ApiError::internal())?;
    if !updated {
        return Err(ApiError::not_found());
    }

    settings_response(&state, auth.tenant_id).await
}

async fn settings_response(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
) -> Result<Json<SettingsResponse>, ApiError> {
    let row = infra_postgres::shared::find_tenant_by_id(&state.app_pool, tenant_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;

    let logo_url = match row.logo_file_id {
        Some(file_id) => {
            let file = infra_postgres::media::find_file_by_id(&state.app_pool, tenant_id, file_id)
                .await
                .map_err(|_| ApiError::internal())?
                .ok_or_else(ApiError::internal)?;
            let presigned = media::presign_object(state, &file.bucket, &file.object_key).await?;
            Some(media::authenticated_media_content_url(
                file_id,
                &presigned.url,
            ))
        }
        None => None,
    };

    Ok(Json(SettingsResponse {
        display_name: row.display_name,
        logo_file_id: row.logo_file_id,
        logo_url,
    }))
}

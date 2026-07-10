use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::{AuthUser, require_admin};
use crate::error::ApiError;
use crate::media;
use crate::portal::resolve_public_catalog_tenant;
use crate::state::AppState;

pub mod payments;

pub use payments::{
    connect_asaas, disconnect_asaas, get_payment_balance, get_payment_settings,
    list_payment_transactions, update_payment_settings,
};

#[derive(Serialize)]
pub struct SettingsResponse {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "logoFileId", skip_serializing_if = "Option::is_none")]
    pub logo_file_id: Option<Uuid>,
    #[serde(rename = "logoUrl", skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(rename = "salesContactPhone", skip_serializing_if = "Option::is_none")]
    pub sales_contact_phone: Option<String>,
    #[serde(rename = "paymentMethods", skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<payments::PaymentMethodsResponse>,
}

#[derive(Deserialize, Default)]
pub struct UpdateSettingsRequest {
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "salesContactPhone")]
    pub sales_contact_phone: Option<String>,
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

pub async fn get_public_settings(
    State(state): State<AppState>,
) -> Result<Json<SettingsResponse>, ApiError> {
    let tenant_id = resolve_public_catalog_tenant()?;
    settings_response(&state, tenant_id).await
}

pub async fn patch_settings(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<UpdateSettingsRequest>,
) -> Result<Json<SettingsResponse>, ApiError> {
    require_admin(&auth)?;

    if body.display_name.is_none() && body.sales_contact_phone.is_none() {
        return settings_response(&state, auth.tenant_id).await;
    }

    if let Some(display_name) = body.display_name {
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
    }

    if let Some(phone) = body.sales_contact_phone {
        let normalized = normalize_sales_contact_phone(&phone)?;
        let updated = infra_postgres::shared::update_tenant_sales_contact_phone(
            &state.app_pool,
            auth.tenant_id,
            normalized.as_deref(),
        )
        .await
        .map_err(|_| ApiError::internal())?;
        if !updated {
            return Err(ApiError::not_found());
        }
    }

    settings_response(&state, auth.tenant_id).await
}

pub async fn update_site_logo(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<UpdateLogoRequest>,
) -> Result<Json<SettingsResponse>, ApiError> {
    require_admin(&auth)?;
    let file =
        infra_postgres::media::find_file_by_id(&state.app_pool, auth.tenant_id, body.file_id)
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

fn normalize_sales_contact_phone(raw: &str) -> Result<Option<String>, ApiError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let digits: String = trimmed.chars().filter(|ch| ch.is_ascii_digit()).collect();
    if digits.len() < 10 || digits.len() > 15 {
        return Err(ApiError::bad_request(
            "VALIDATION_ERROR",
            "salesContactPhone must be 10–15 digits (E.164 or BR mobile)",
        ));
    }

    Ok(Some(digits))
}

/// Missing logo blob must not fail settings — branding still returns without `logoUrl`.
async fn resolve_logo_url(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
    file_id: Uuid,
) -> Result<Option<String>, ApiError> {
    let Some(file) = infra_postgres::media::find_file_by_id(&state.app_pool, tenant_id, file_id)
        .await
        .map_err(|_| ApiError::internal())?
    else {
        return Ok(None);
    };
    match media::presign_object(state, &file.bucket, &file.object_key).await {
        Ok(presigned) => Ok(Some(media::catalog_image_url(
            file_id,
            &presigned.url,
        ))),
        Err(_) => Ok(None),
    }
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
        Some(file_id) => resolve_logo_url(state, tenant_id, file_id).await?,
        None => None,
    };

    let payment_methods = payments::public_payment_methods(state, tenant_id).await?;

    Ok(Json(SettingsResponse {
        display_name: row.display_name,
        logo_file_id: row.logo_file_id,
        logo_url,
        sales_contact_phone: row.sales_contact_phone,
        payment_methods,
    }))
}

use axum::{
    Json,
    extract::{Path, State},
};
use domain_commerces::{RegistrationMode, RegistrationStatus};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::commerces::registrations::access::load_review_flag;
use crate::commerces::registrations::reload_registration;
use crate::commerces::registrations::types::{
    PatchRegistrationRequest, RegistrationResponse, RejectRegistrationRequest,
    map_commerce_domain_error, map_registration_error,
};
use crate::error::ApiError;
use crate::state::AppState;

pub async fn approve_registration(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<RegistrationResponse>, ApiError> {
    let review_flag = load_review_flag(&state, &auth).await?;
    application::ensure_can_review_commerce(auth.role, review_flag)
        .map_err(map_registration_error)?;

    let row = load_pending(&state, auth.tenant_id, id).await?;
    let commerce = restore_row(&row, auth.tenant_id)?;
    let _ = commerce
        .approve(auth.user_id)
        .map_err(map_commerce_domain_error)?;

    let ok = infra_postgres::commerces::registrations::approve_registration(
        &state.app_pool,
        auth.tenant_id,
        id,
        auth.user_id,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    if !ok {
        return Err(ApiError::invalid_registration_transition());
    }
    Ok(Json(reload_registration(&state, auth.tenant_id, id).await?))
}

pub async fn reject_registration(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<RejectRegistrationRequest>,
) -> Result<Json<RegistrationResponse>, ApiError> {
    let review_flag = load_review_flag(&state, &auth).await?;
    application::ensure_can_review_commerce(auth.role, review_flag)
        .map_err(map_registration_error)?;

    let row = load_pending(&state, auth.tenant_id, id).await?;
    let commerce = restore_row(&row, auth.tenant_id)?;
    let _ = commerce
        .reject(auth.user_id, &body.reason)
        .map_err(map_commerce_domain_error)?;

    let ok = infra_postgres::commerces::registrations::reject_registration(
        &state.app_pool,
        auth.tenant_id,
        id,
        auth.user_id,
        body.reason.trim(),
    )
    .await
    .map_err(|_| ApiError::internal())?;
    if !ok {
        return Err(ApiError::invalid_registration_transition());
    }
    Ok(Json(reload_registration(&state, auth.tenant_id, id).await?))
}

pub async fn patch_registration(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<PatchRegistrationRequest>,
) -> Result<Json<RegistrationResponse>, ApiError> {
    use super::access::ensure_registration_submitter;

    let row = load_pending(&state, auth.tenant_id, id).await?;
    ensure_registration_submitter(&state, &auth, &row).await?;

    let legal_name = body
        .legal_name
        .as_deref()
        .unwrap_or(row.legal_name.as_str());
    let trade_name = body.trade_name.as_deref().or(Some(row.trade_name.as_str()));
    let commerce = restore_row(&row, auth.tenant_id)?;
    let updated = commerce
        .update_pending_fields(legal_name.to_owned(), trade_name.map(str::to_owned))
        .map_err(map_commerce_domain_error)?;

    let mut legacy = row
        .lookup_snapshot
        .clone()
        .unwrap_or_else(|| serde_json::json!({}));
    if let Some(contact) = body.contact {
        legacy["contact"] = serde_json::json!({
            "phone": contact.phone,
            "email": contact.email,
        });
    }

    let ok = infra_postgres::commerces::registrations::update_registration_fields(
        &state.app_pool,
        auth.tenant_id,
        id,
        updated.legal_name(),
        updated.trade_name(),
        legacy,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    if !ok {
        return Err(ApiError::registration_not_editable());
    }
    Ok(Json(reload_registration(&state, auth.tenant_id, id).await?))
}

async fn load_pending(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
    id: Uuid,
) -> Result<infra_postgres::commerces::CommerceRow, ApiError> {
    let row = infra_postgres::commerces::registrations::find_commerce_registration_by_id(
        &state.app_pool,
        tenant_id,
        id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::commerce_not_found)?;
    if row.registration_status != RegistrationStatus::PendingReview.as_str() {
        return Err(ApiError::registration_not_editable());
    }
    Ok(row)
}

fn restore_row(
    row: &infra_postgres::commerces::CommerceRow,
    tenant_id: domain_shared::TenantId,
) -> Result<domain_commerces::Commerce, ApiError> {
    let mode = row
        .registration_mode
        .as_deref()
        .map(RegistrationMode::parse)
        .transpose()
        .map_err(map_commerce_domain_error)?;
    application::restore_commerce_with_status(
        row.id,
        &row.cnpj,
        &row.legal_name,
        &row.trade_name,
        tenant_id,
        row.active,
        RegistrationStatus::parse(&row.registration_status).map_err(map_commerce_domain_error)?,
        row.submitted_by_user_id,
        mode,
    )
    .map_err(map_commerce_domain_error)
}

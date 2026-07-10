use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use domain_domains::{DomainError, TenantDomain, is_reserved_hostname, normalize_hostname};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::{AuthUser, require_admin};
use crate::domains::support::{
    challenge_expires_at, ensure_custom_domain_plan, persist_domain, reserved_platform_hosts,
    row_to_domain, txt_challenge, verification_token,
};
use crate::error::ApiError;
use crate::state::AppState;

#[derive(Serialize)]
pub struct DomainResponse {
    pub id: Uuid,
    pub hostname: String,
    pub status: String,
    #[serde(rename = "isPrimary")]
    pub is_primary: bool,
    #[serde(rename = "verifiedAt", skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct DomainListResponse {
    pub data: Vec<DomainResponse>,
}

#[derive(Deserialize)]
pub struct CreateDomainRequest {
    pub hostname: String,
}

#[derive(Serialize)]
pub struct CreateDomainResponse {
    #[serde(flatten)]
    pub domain: DomainResponse,
    #[serde(rename = "txtRecord")]
    pub txt_record: String,
    #[serde(rename = "txtValue")]
    pub txt_value: String,
}

#[derive(Serialize)]
pub struct VerifyDomainResponse {
    pub status: String,
    #[serde(rename = "txtRecord")]
    pub txt_record: String,
    #[serde(rename = "txtValue")]
    pub txt_value: String,
    #[serde(rename = "verifiedAt", skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<DateTime<Utc>>,
}

pub async fn list_domains(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<DomainListResponse>, ApiError> {
    require_admin(&auth)?;
    let rows = infra_postgres::domains::list_domains_tenant(&state.app_pool, auth.tenant_id)
        .await
        .map_err(|_| ApiError::internal())?;
    Ok(Json(DomainListResponse {
        data: rows.iter().map(domain_response_row).collect(),
    }))
}

pub async fn create_domain(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<CreateDomainRequest>,
) -> Result<(StatusCode, Json<CreateDomainResponse>), ApiError> {
    require_admin(&auth)?;
    ensure_custom_domain_plan(&state, auth.tenant_id).await?;
    let hostname = normalize_hostname(&body.hostname).map_err(map_domain_err)?;
    if is_reserved_hostname(&hostname, &reserved_platform_hosts()) {
        return Err(ApiError::bad_request(
            "RESERVED_HOSTNAME",
            "Hostname is reserved for platform use",
        ));
    }
    if infra_postgres::domains::find_domain_by_hostname(&state.admin_pool, &hostname)
        .await
        .map_err(|_| ApiError::internal())?
        .is_some()
    {
        return Err(ApiError::bad_request(
            "HOSTNAME_TAKEN",
            "Hostname is already registered",
        ));
    }

    let id = Uuid::now_v7();
    let token = verification_token();
    let now = Utc::now();
    let mut domain = TenantDomain::add(
        id,
        auth.tenant_id,
        &body.hostname,
        token.clone(),
        now,
    )
    .map_err(map_domain_err)?;
    domain.start_verifying(now).map_err(map_domain_err)?;

    infra_postgres::domains::insert_tenant_domain(
        &state.app_pool,
        infra_postgres::domains::NewDomainRow {
            id: domain.id,
            tenant_id: domain.tenant_id,
            hostname: domain.hostname.clone(),
            status: domain.status.as_str().into(),
            verification_token: domain.verification_token.clone(),
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;
    infra_postgres::domains::insert_challenge(
        &state.app_pool,
        auth.tenant_id,
        Uuid::now_v7(),
        domain.id,
        &token,
        challenge_expires_at(),
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let (txt_record, txt_value) = txt_challenge(&domain);
    Ok((
        StatusCode::CREATED,
        Json(CreateDomainResponse {
            domain: domain_response_entity(&domain),
            txt_record,
            txt_value,
        }),
    ))
}

pub async fn get_domain_verify(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<VerifyDomainResponse>, ApiError> {
    require_admin(&auth)?;
    let row = infra_postgres::domains::find_domain_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;
    let domain = row_to_domain(&row);
    let (txt_record, txt_value) = txt_challenge(&domain);
    Ok(Json(VerifyDomainResponse {
        status: domain.status.as_str().into(),
        txt_record,
        txt_value,
        verified_at: domain.verified_at,
    }))
}

pub async fn delete_domain(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    require_admin(&auth)?;
    let row = infra_postgres::domains::find_domain_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;
    let mut domain = row_to_domain(&row);
    domain.detach(Utc::now()).map_err(map_domain_err)?;
    persist_domain(&state.app_pool, &domain, false).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn set_primary_domain(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<DomainResponse>, ApiError> {
    require_admin(&auth)?;
    let row = infra_postgres::domains::find_domain_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;
    let mut domain = row_to_domain(&row);
    let now = Utc::now();

    let existing = infra_postgres::domains::list_domains_tenant(&state.app_pool, auth.tenant_id)
        .await
        .map_err(|_| ApiError::internal())?;
    for other in existing {
        if other.id == domain.id || !other.is_primary {
            continue;
        }
        let mut prev = row_to_domain(&other);
        prev.detach(now).map_err(map_domain_err)?;
        persist_domain(&state.app_pool, &prev, false).await?;
    }

    infra_postgres::domains::clear_primary_for_tenant(&state.app_pool, auth.tenant_id, Some(domain.id))
        .await
        .map_err(|_| ApiError::internal())?;
    domain.set_primary(now).map_err(map_domain_err)?;
    persist_domain(&state.app_pool, &domain, false).await?;
    Ok(Json(domain_response_entity(&domain)))
}

fn domain_response_entity(domain: &TenantDomain) -> DomainResponse {
    DomainResponse {
        id: domain.id,
        hostname: domain.hostname.clone(),
        status: domain.status.as_str().into(),
        is_primary: domain.is_primary,
        verified_at: domain.verified_at,
        created_at: domain.created_at,
    }
}

fn domain_response_row(row: &infra_postgres::domains::DomainRow) -> DomainResponse {
    domain_response_entity(&row_to_domain(row))
}

fn map_domain_err(err: DomainError) -> ApiError {
    match err {
        DomainError::InvalidHostname | DomainError::ReservedHostname => {
            ApiError::bad_request("VALIDATION_ERROR", "Invalid hostname")
        }
        DomainError::NotVerified => {
            ApiError::bad_request("INVALID_TRANSITION", "Domain must be verified before activation")
        }
        DomainError::CannotSetPrimary => {
            ApiError::bad_request("INVALID_TRANSITION", "Primary domain must be verified or active")
        }
        DomainError::InvalidTransition { .. } => {
            ApiError::bad_request("INVALID_TRANSITION", "Invalid domain status transition")
        }
    }
}

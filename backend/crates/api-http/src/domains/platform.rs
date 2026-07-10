use axum::{
    Json,
    extract::{Path, Query, State},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domains::support::{persist_domain, row_to_domain};
use crate::domains::verification::force_verify_domain;
use crate::error::ApiError;
use crate::list_query::build_cursor_page;
use crate::platform::auth::PlatformAuthUser;
use crate::platform_audit::record_platform_audit_stub;
use crate::state::AppState;

const DEFAULT_LIMIT: u32 = 20;
const MAX_LIMIT: u32 = 100;

#[derive(Deserialize)]
pub struct PlatformDomainsQuery {
    pub limit: Option<u32>,
    pub cursor: Option<Uuid>,
    #[serde(rename = "filter[status]")]
    pub filter_status: Option<String>,
    #[serde(rename = "filter[tenant_id]")]
    pub filter_tenant_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct PlatformDomainResponse {
    pub id: Uuid,
    #[serde(rename = "tenantId")]
    pub tenant_id: Uuid,
    pub hostname: String,
    pub status: String,
    #[serde(rename = "isPrimary")]
    pub is_primary: bool,
    #[serde(rename = "verifiedAt", skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct PatchPlatformDomainRequest {
    pub status: Option<String>,
    #[serde(rename = "isPrimary")]
    pub is_primary: Option<bool>,
}

pub async fn list_platform_domains(
    State(state): State<AppState>,
    _auth: PlatformAuthUser,
    Query(query): Query<PlatformDomainsQuery>,
) -> Result<Json<crate::list_query::CursorListResponse<PlatformDomainResponse>>, ApiError> {
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let rows = infra_postgres::domains::list_domains_platform(
        &state.admin_pool,
        query.filter_status.as_deref(),
        query.filter_tenant_id,
        query.cursor,
        i64::from(limit) + 1,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    let items: Vec<PlatformDomainResponse> = rows.iter().map(platform_response).collect();
    Ok(Json(build_cursor_page(items, limit, |item| item.id)))
}

pub async fn force_verify_platform_domain(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<PlatformDomainResponse>, ApiError> {
    force_verify_domain(&state, id).await?;
    let row = infra_postgres::domains::find_domain_by_id_admin(&state.admin_pool, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;
    record_platform_audit_stub(
        &state,
        auth.user_id,
        "domain.force_verify",
        Some(row.tenant_id.as_uuid()),
    )
    .await;
    Ok(Json(platform_response(&row)))
}

pub async fn patch_platform_domain(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<PatchPlatformDomainRequest>,
) -> Result<Json<PlatformDomainResponse>, ApiError> {
    let row = infra_postgres::domains::find_domain_by_id_admin(&state.admin_pool, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;
    let mut domain = row_to_domain(&row);
    let now = chrono::Utc::now();

    if let Some(status) = body.status.as_deref() {
        match status {
            "Active" => {
                domain.activate(now).map_err(|_| ApiError::bad_request("INVALID_TRANSITION", "Cannot activate"))?;
            }
            "Detached" => {
                domain.detach(now).map_err(|_| ApiError::bad_request("INVALID_TRANSITION", "Cannot detach"))?;
            }
            _ => {
                return Err(ApiError::bad_request("INVALID_INPUT", "Unsupported status"));
            }
        }
    }
    if body.is_primary == Some(true) {
        infra_postgres::domains::clear_primary_for_tenant_admin(
            &state.admin_pool,
            domain.tenant_id,
            Some(domain.id),
        )
        .await
        .map_err(|_| ApiError::internal())?;
        domain.set_primary(now).map_err(|_| ApiError::bad_request("INVALID_TRANSITION", "Cannot set primary"))?;
    }

    persist_domain(&state.admin_pool, &domain, true).await?;
    record_platform_audit_stub(
        &state,
        auth.user_id,
        "domain.patch",
        Some(domain.tenant_id.as_uuid()),
    )
    .await;
    Ok(Json(platform_from_domain(&domain)))
}

pub async fn run_domain_verification_job_handler(
    State(state): State<AppState>,
    _auth: PlatformAuthUser,
) -> Result<Json<serde_json::Value>, ApiError> {
    Ok(Json(super::verification::run_domain_verification_job(&state).await?))
}

fn platform_response(row: &infra_postgres::domains::DomainRow) -> PlatformDomainResponse {
    PlatformDomainResponse {
        id: row.id,
        tenant_id: row.tenant_id.as_uuid(),
        hostname: row.hostname.clone(),
        status: row.status.clone(),
        is_primary: row.is_primary,
        verified_at: row.verified_at,
        created_at: row.created_at,
    }
}

fn platform_from_domain(domain: &domain_domains::TenantDomain) -> PlatformDomainResponse {
    PlatformDomainResponse {
        id: domain.id,
        tenant_id: domain.tenant_id.as_uuid(),
        hostname: domain.hostname.clone(),
        status: domain.status.as_str().into(),
        is_primary: domain.is_primary,
        verified_at: domain.verified_at,
        created_at: domain.created_at,
    }
}

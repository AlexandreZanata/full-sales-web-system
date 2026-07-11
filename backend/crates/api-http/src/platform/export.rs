use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use domain_shared::TenantId;
use serde::Serialize;
use uuid::Uuid;

use crate::audit_context::AuditRequestContext;
use crate::auth::{AuthUser, require_admin};
use crate::error::ApiError;
use crate::platform::auth::PlatformAuthUser;
use crate::platform_audit::record_platform_audit;
use crate::state::AppState;

use super::export_job;

#[derive(Serialize)]
pub struct ExportJobResponse {
    pub id: Uuid,
    pub status: String,
    #[serde(rename = "downloadUrl", skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "completedAt", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
}

pub async fn start_tenant_export(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    ctx: AuditRequestContext,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<ExportJobResponse>), ApiError> {
    start_export(
        &state,
        &ctx,
        auth.user_id,
        "PlatformAdmin",
        TenantId::from_uuid(id),
    )
    .await
}

pub async fn get_tenant_export(
    State(state): State<AppState>,
    _auth: PlatformAuthUser,
    Path((tenant_id, job_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ExportJobResponse>, ApiError> {
    get_export(&state, TenantId::from_uuid(tenant_id), job_id).await
}

pub async fn start_settings_data_export(
    State(state): State<AppState>,
    auth: AuthUser,
    ctx: AuditRequestContext,
) -> Result<(StatusCode, Json<ExportJobResponse>), ApiError> {
    require_admin(&auth)?;
    start_export(&state, &ctx, auth.user_id, "User", auth.tenant_id).await
}

pub async fn get_settings_data_export(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(job_id): Path<Uuid>,
) -> Result<Json<ExportJobResponse>, ApiError> {
    require_admin(&auth)?;
    get_export(&state, auth.tenant_id, job_id).await
}

async fn start_export(
    state: &AppState,
    ctx: &AuditRequestContext,
    requested_by: Uuid,
    actor_type: &'static str,
    tenant_id: TenantId,
) -> Result<(StatusCode, Json<ExportJobResponse>), ApiError> {
    if !tenant_exists(state, tenant_id).await? {
        return Err(ApiError::not_found());
    }
    let job_id = Uuid::now_v7();
    infra_postgres::ops::insert_export_job(
        &state.admin_pool,
        infra_postgres::ops::NewDataExportJob {
            id: job_id,
            tenant_id,
            requested_by,
            actor_type,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;
    if actor_type == "PlatformAdmin" {
        record_platform_audit(
            state,
            ctx,
            requested_by,
            "tenant.export.requested",
            Some(tenant_id),
            "Tenant",
            tenant_id.as_uuid(),
            Some(serde_json::json!({ "jobId": job_id })),
        )
        .await?;
    }
    let worker_state = state.clone();
    tokio::spawn(async move {
        if let Err(err) = export_job::run_export_job(&worker_state, tenant_id, job_id).await {
            let _ = infra_postgres::ops::mark_export_failed(&worker_state.admin_pool, job_id, &err)
                .await;
        }
    });
    get_export(state, tenant_id, job_id)
        .await
        .map(|body| (StatusCode::ACCEPTED, body))
}

async fn get_export(
    state: &AppState,
    tenant_id: TenantId,
    job_id: Uuid,
) -> Result<Json<ExportJobResponse>, ApiError> {
    let row = infra_postgres::ops::find_export_job(&state.admin_pool, tenant_id, job_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;
    let download_url = if row.status == "completed" {
        presigned_export_url(state, &row).await.ok()
    } else {
        None
    };
    Ok(Json(ExportJobResponse {
        id: row.id,
        status: row.status,
        download_url,
        error_message: row.error_message,
        created_at: row.created_at,
        completed_at: row.completed_at,
    }))
}

async fn tenant_exists(state: &AppState, tenant_id: TenantId) -> Result<bool, ApiError> {
    Ok(
        infra_postgres::shared::find_tenant_lifecycle(&state.admin_pool, tenant_id)
            .await
            .map_err(|_| ApiError::internal())?
            .is_some(),
    )
}

async fn presigned_export_url(
    state: &AppState,
    row: &infra_postgres::ops::DataExportJobRow,
) -> Result<String, ApiError> {
    let bucket = row
        .storage_bucket
        .as_deref()
        .ok_or_else(ApiError::internal)?;
    let key = row.storage_key.as_deref().ok_or_else(ApiError::internal)?;
    let url = state
        .storage
        .presigned_get(bucket, key, std::time::Duration::from_secs(900))
        .await
        .map_err(|_| ApiError::internal())?;
    Ok(url.url)
}

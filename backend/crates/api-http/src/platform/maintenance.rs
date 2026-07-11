use axum::{Json, extract::State, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use domain_shared::TenantId;

use crate::audit_context::AuditRequestContext;
use crate::error::ApiError;
use crate::platform::auth::PlatformAuthUser;
use crate::platform_audit::record_platform_audit;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ScheduleMaintenanceRequest {
    #[serde(rename = "tenantId")]
    pub tenant_id: Option<Uuid>,
    pub message: String,
    #[serde(rename = "startsAt")]
    pub starts_at: DateTime<Utc>,
    #[serde(rename = "endsAt")]
    pub ends_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct MaintenanceWindowResponse {
    pub id: Uuid,
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<Uuid>,
    pub message: String,
    #[serde(rename = "startsAt")]
    pub starts_at: DateTime<Utc>,
    #[serde(rename = "endsAt")]
    pub ends_at: DateTime<Utc>,
}

pub async fn schedule_maintenance(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    ctx: AuditRequestContext,
    Json(body): Json<ScheduleMaintenanceRequest>,
) -> Result<(StatusCode, Json<MaintenanceWindowResponse>), ApiError> {
    if body.message.trim().len() < 3 {
        return Err(ApiError::bad_request(
            "VALIDATION_ERROR",
            "Message too short",
        ));
    }
    if body.ends_at <= body.starts_at {
        return Err(ApiError::bad_request(
            "VALIDATION_ERROR",
            "endsAt must be after startsAt",
        ));
    }
    if let Some(tid) = body.tenant_id {
        let exists = infra_postgres::identity::tenant_exists(&state.admin_pool, tid)
            .await
            .map_err(|_| ApiError::internal())?;
        if !exists {
            return Err(ApiError::not_found());
        }
    }
    let id = Uuid::now_v7();
    let tenant_id = body.tenant_id.map(domain_shared::TenantId::from_uuid);
    infra_postgres::ops::insert_maintenance_window(
        &state.admin_pool,
        infra_postgres::ops::MaintenanceInsert {
            id,
            tenant_id,
            message: body.message.trim().to_owned(),
            starts_at: body.starts_at,
            ends_at: body.ends_at,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;
    record_platform_audit(
        &state,
        &ctx,
        auth.user_id,
        "maintenance.schedule",
        body.tenant_id.map(TenantId::from_uuid),
        "MaintenanceWindow",
        id,
        None,
    )
    .await?;
    Ok((
        StatusCode::CREATED,
        Json(MaintenanceWindowResponse {
            id,
            tenant_id: body.tenant_id,
            message: body.message.trim().to_owned(),
            starts_at: body.starts_at,
            ends_at: body.ends_at,
        }),
    ))
}

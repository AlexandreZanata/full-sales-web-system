use std::time::Duration;

use axum::{Json, extract::State};
use chrono::{Duration as ChronoDuration, Utc};
use domain_shared::TenantId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::audit_context::AuditRequestContext;
use crate::error::ApiError;
use crate::platform::auth::PlatformAuthUser;
use crate::platform_audit::record_platform_audit;
use crate::state::AppState;

const IMPERSONATION_TTL: Duration = Duration::from_secs(15 * 60);

#[derive(Deserialize)]
pub struct ImpersonateRequest {
    #[serde(rename = "tenantId")]
    pub tenant_id: Uuid,
    #[serde(rename = "userId")]
    pub user_id: Option<Uuid>,
    pub reason: String,
}

#[derive(Serialize)]
pub struct ImpersonateResponse {
    #[serde(rename = "impersonationToken")]
    pub impersonation_token: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: Uuid,
}

#[derive(Deserialize)]
pub struct EndImpersonationRequest {
    #[serde(rename = "grantId")]
    pub grant_id: Uuid,
}

pub async fn start_impersonation(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    ctx: AuditRequestContext,
    Json(body): Json<ImpersonateRequest>,
) -> Result<Json<ImpersonateResponse>, ApiError> {
    if body.reason.trim().len() < 3 {
        return Err(ApiError::bad_request(
            "INVALID_INPUT",
            "reason must be at least 3 characters",
        ));
    }

    if !infra_postgres::identity::tenant_exists(&state.admin_pool, body.tenant_id)
        .await
        .map_err(|_| ApiError::internal())?
    {
        return Err(ApiError::not_found_with_code(
            "TENANT_NOT_FOUND",
            "Tenant not found",
        ));
    }

    let tenant_id = TenantId::from_uuid(body.tenant_id);
    let acting_user_id =
        infra_postgres::identity::find_tenant_admin_user(&state.app_pool, tenant_id, body.user_id)
            .await
            .map_err(|_| ApiError::internal())?
            .ok_or_else(|| {
                ApiError::not_found_with_code("USER_NOT_FOUND", "Tenant admin user not found")
            })?;

    let grant_id = Uuid::now_v7();
    let expires_at = Utc::now() + ChronoDuration::seconds(IMPERSONATION_TTL.as_secs() as i64);

    infra_postgres::identity::insert_impersonation_grant(
        &state.admin_pool,
        infra_postgres::identity::ImpersonationGrantInsert {
            id: grant_id,
            platform_user_id: auth.user_id,
            target_tenant_id: body.tenant_id,
            target_user_id: Some(acting_user_id),
            reason: body.reason.trim(),
            expires_at,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;

    record_platform_audit(
        &state,
        &ctx,
        auth.user_id,
        "impersonation.start",
        Some(tenant_id),
        "ImpersonationGrant",
        grant_id,
        Some(serde_json::json!({ "reason": body.reason.trim() })),
    )
    .await?;

    let token = state
        .jwt
        .issue_impersonation_token(
            auth.user_id,
            body.tenant_id,
            acting_user_id,
            grant_id,
            IMPERSONATION_TTL,
        )
        .map_err(|_| ApiError::internal())?;

    Ok(Json(ImpersonateResponse {
        impersonation_token: token,
        expires_at: expires_at.to_rfc3339(),
        tenant_id: body.tenant_id,
    }))
}

pub async fn end_impersonation(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    ctx: AuditRequestContext,
    Json(body): Json<EndImpersonationRequest>,
) -> Result<http::StatusCode, ApiError> {
    let revoked =
        infra_postgres::identity::revoke_impersonation_grant(&state.admin_pool, body.grant_id)
            .await
            .map_err(|_| ApiError::internal())?;

    if !revoked {
        return Err(ApiError::not_found());
    }

    record_platform_audit(
        &state,
        &ctx,
        auth.user_id,
        "impersonation.end",
        None,
        "ImpersonationGrant",
        body.grant_id,
        None,
    )
    .await?;
    Ok(http::StatusCode::NO_CONTENT)
}

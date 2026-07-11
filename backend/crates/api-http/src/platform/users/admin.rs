use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use infra_crypto::PasswordHasher;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use domain_shared::TenantId;

use crate::audit_context::AuditRequestContext;
use crate::error::ApiError;
use crate::platform::auth::PlatformAuthUser;
use crate::platform::users::{PlatformUserItem, to_item};
use crate::platform_audit::record_platform_audit;
use crate::state::AppState;

#[derive(Serialize)]
pub struct UserActiveResponse {
    pub active: bool,
}

#[derive(Serialize)]
pub struct ResetPasswordResponse {
    pub queued: bool,
    #[serde(rename = "temporaryPassword")]
    pub temporary_password: String,
}

#[derive(Deserialize)]
pub struct PatchPlatformUserRequest {
    pub role: Option<String>,
}

pub async fn get_platform_user(
    State(state): State<AppState>,
    _auth: PlatformAuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<PlatformUserItem>, ApiError> {
    let row = infra_postgres::identity::find_user_cross_tenant(&state.app_pool, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;
    Ok(Json(to_item(row)))
}

pub async fn disable_platform_user(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    ctx: AuditRequestContext,
    Path(id): Path<Uuid>,
) -> Result<Json<UserActiveResponse>, ApiError> {
    mutate_active(&state, &ctx, &auth, id, false).await
}

pub async fn enable_platform_user(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    ctx: AuditRequestContext,
    Path(id): Path<Uuid>,
) -> Result<Json<UserActiveResponse>, ApiError> {
    mutate_active(&state, &ctx, &auth, id, true).await
}

async fn mutate_active(
    state: &AppState,
    ctx: &AuditRequestContext,
    auth: &PlatformAuthUser,
    id: Uuid,
    active: bool,
) -> Result<Json<UserActiveResponse>, ApiError> {
    let row = infra_postgres::identity::find_user_cross_tenant(&state.app_pool, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;
    if !active && row.role == "Admin" {
        ensure_not_last_admin(state, row.tenant_id, Some(id)).await?;
    }
    infra_postgres::identity::set_user_active(&state.app_pool, id, active)
        .await
        .map_err(|_| ApiError::internal())?;
    record_platform_audit(
        state,
        ctx,
        auth.user_id,
        if active {
            "user.enable"
        } else {
            "user.disable"
        },
        Some(TenantId::from_uuid(row.tenant_id)),
        "User",
        id,
        None,
    )
    .await?;
    Ok(Json(UserActiveResponse { active }))
}

pub async fn reset_platform_user_password(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    ctx: AuditRequestContext,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<ResetPasswordResponse>), ApiError> {
    let row = infra_postgres::identity::find_user_cross_tenant(&state.app_pool, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;
    let temp_password = Uuid::now_v7().to_string();
    let hash = PasswordHasher::hash(&temp_password).map_err(|_| ApiError::internal())?;
    infra_postgres::identity::update_user_password(&state.app_pool, id, &hash)
        .await
        .map_err(|_| ApiError::internal())?;
    record_platform_audit(
        &state,
        &ctx,
        auth.user_id,
        "user.reset_password",
        Some(TenantId::from_uuid(row.tenant_id)),
        "User",
        id,
        None,
    )
    .await?;
    Ok((
        StatusCode::ACCEPTED,
        Json(ResetPasswordResponse {
            queued: true,
            temporary_password: temp_password,
        }),
    ))
}

pub async fn patch_platform_user(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    ctx: AuditRequestContext,
    Path(id): Path<Uuid>,
    Json(body): Json<PatchPlatformUserRequest>,
) -> Result<Json<PlatformUserItem>, ApiError> {
    let Some(role) = body.role else {
        return get_platform_user(State(state), auth, Path(id)).await;
    };
    if !matches!(role.as_str(), "Admin" | "Driver" | "Seller") {
        return Err(ApiError::bad_request("VALIDATION_ERROR", "Invalid role"));
    }
    let row = infra_postgres::identity::find_user_cross_tenant(&state.app_pool, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;
    if row.role == "Admin" && role != "Admin" {
        ensure_not_last_admin(&state, row.tenant_id, Some(id)).await?;
    }
    infra_postgres::identity::update_user_role(&state.app_pool, id, &role)
        .await
        .map_err(|_| ApiError::internal())?;
    record_platform_audit(
        &state,
        &ctx,
        auth.user_id,
        "user.patch",
        Some(TenantId::from_uuid(row.tenant_id)),
        "User",
        id,
        Some(serde_json::json!({ "role": role })),
    )
    .await?;
    get_platform_user(State(state), auth, Path(id)).await
}

async fn ensure_not_last_admin(
    state: &AppState,
    tenant_id: Uuid,
    except: Option<Uuid>,
) -> Result<(), ApiError> {
    let count = infra_postgres::identity::count_active_admins(
        &state.app_pool,
        domain_shared::TenantId::from_uuid(tenant_id),
        except,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    if count == 0 {
        return Err(ApiError::bad_request(
            "LAST_ADMIN_REQUIRED",
            "Cannot demote or disable the last Admin without a replacement",
        ));
    }
    Ok(())
}

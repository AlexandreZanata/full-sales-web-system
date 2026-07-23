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

#[derive(Deserialize)]
pub struct CreatePlatformTenantUserRequest {
    pub name: String,
    pub email: String,
    pub role: String,
    #[serde(rename = "commerceId")]
    pub commerce_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct CreatePlatformTenantUserResponse {
    pub user: PlatformUserItem,
    #[serde(rename = "temporaryPassword")]
    pub temporary_password: String,
}

pub async fn create_platform_tenant_user(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    ctx: AuditRequestContext,
    Path(tenant_id): Path<Uuid>,
    Json(body): Json<CreatePlatformTenantUserRequest>,
) -> Result<(StatusCode, Json<CreatePlatformTenantUserResponse>), ApiError> {
    let tenant = TenantId::from_uuid(tenant_id);
    let exists = infra_postgres::shared::find_tenant_by_id(&state.admin_pool, tenant)
        .await
        .map_err(|_| ApiError::internal())?
        .is_some();
    if !exists {
        return Err(ApiError::not_found());
    }

    let user = application::parse_register_user(
        &body.name,
        &body.email,
        &body.role,
        tenant,
        body.commerce_id,
    )
    .map_err(|err| match err {
        application::AppError::Identity(domain_identity::IdentityError::InvalidRole) => {
            ApiError::bad_request("INVALID_ROLE", "Invalid role")
        }
        application::AppError::Identity(domain_identity::IdentityError::InvalidEmail) => {
            ApiError::bad_request("INVALID_EMAIL", "Invalid email address")
        }
        application::AppError::Identity(domain_identity::IdentityError::InvalidFullName) => {
            ApiError::bad_request("INVALID_NAME", "Name must contain at least two parts")
        }
        application::AppError::Identity(domain_identity::IdentityError::CommerceRequired) => {
            ApiError::bad_request(
                "COMMERCE_REQUIRED",
                "commerceId is required for CommerceContact",
            )
        }
        _ => ApiError::bad_request("INVALID_INPUT", "Invalid user request"),
    })?;

    let temporary_password = Uuid::now_v7().to_string();
    let password_hash =
        PasswordHasher::hash(&temporary_password).map_err(|_| ApiError::internal())?;

    infra_postgres::identity::insert_user(
        &state.app_pool,
        tenant,
        infra_postgres::identity::InsertUserParams {
            id: user.id().as_uuid(),
            email: user.email().as_str(),
            name: user.name().as_str(),
            role: user.role().as_str(),
            password_hash: &password_hash,
            commerce_id: user.commerce_id(),
            profile_file_id: user.profile_file_id(),
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let row =
        infra_postgres::identity::find_user_cross_tenant(&state.app_pool, user.id().as_uuid())
            .await
            .map_err(|_| ApiError::internal())?
            .ok_or_else(ApiError::internal)?;

    record_platform_audit(
        &state,
        &ctx,
        auth.user_id,
        "user.create",
        Some(tenant),
        "User",
        user.id().as_uuid(),
        Some(serde_json::json!({ "role": user.role().as_str() })),
    )
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(CreatePlatformTenantUserResponse {
            user: to_item(row),
            temporary_password,
        }),
    ))
}

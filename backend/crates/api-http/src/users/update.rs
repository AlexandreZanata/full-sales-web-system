//! PATCH /v1/users/{id} — update name / email / optional password.

use axum::{Json, extract::Path, extract::State};
use domain_identity::{Email, FullName};
use infra_crypto::PasswordHasher;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::{AuthUser, require_admin};
use crate::error::ApiError;
use crate::state::AppState;
use crate::users::types::{UserResponse, map_user_app_error, user_response_from_detail};

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub name: String,
    pub email: String,
    #[serde(default)]
    pub password: Option<String>,
}

pub async fn update_user(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    require_admin(&auth)?;
    let _existing = infra_postgres::identity::find_user_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::user_not_found)?;

    let name = FullName::parse(&body.name)
        .map_err(|err| map_user_app_error(application::AppError::Identity(err)))?;
    let email = Email::parse(&body.email)
        .map_err(|err| map_user_app_error(application::AppError::Identity(err)))?;

    let password_hash = match body.password.as_deref().map(str::trim).filter(|p| !p.is_empty()) {
        Some(raw) if raw.len() < 8 => {
            return Err(ApiError::bad_request(
                "INVALID_INPUT",
                "password must be at least 8 characters",
            ));
        }
        Some(raw) => Some(PasswordHasher::hash(raw).map_err(|_| ApiError::internal())?),
        None => None,
    };

    let updated = infra_postgres::identity::update_user_profile_tenant(
        &state.app_pool,
        auth.tenant_id,
        id,
        name.as_str(),
        email.as_str(),
        password_hash.as_deref(),
    )
    .await
    .map_err(|err| {
        let msg = err.to_string();
        if msg.contains("unique") || msg.contains("duplicate") {
            return ApiError::bad_request("CONFLICT", "email already in use");
        }
        ApiError::internal()
    })?;

    if !updated {
        return Err(ApiError::user_not_found());
    }

    let row = infra_postgres::identity::find_user_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::user_not_found)?;
    Ok(Json(user_response_from_detail(&row)))
}

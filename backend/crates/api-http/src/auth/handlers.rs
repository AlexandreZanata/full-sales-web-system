use application::{ACCESS_TOKEN_TTL, AppError, AuthenticatedUser};
use axum::{Json, extract::State, http::HeaderMap};
use infra_crypto::PasswordHasher;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::middleware::AuthUser;
use crate::client_ip::client_ip;
use crate::error::ApiError;
use crate::fraud::on_login_failure;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    #[serde(rename = "expiresIn")]
    pub expires_in: u64,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

pub async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, ApiError> {
    let rate_key = format!("ratelimit:login:{}", client_ip(&headers));
    if state
        .rate_limiter
        .is_blocked(&rate_key, state.login_rate_limit)
    {
        return Err(ApiError::rate_limited());
    }

    let email = body.email.trim().to_lowercase();
    let ip = client_ip(&headers);
    let record = match infra_postgres::identity::find_user_for_login(&state.admin_pool, &email)
        .await
        .map_err(|_| ApiError::internal())?
    {
        Some(record) => record,
        None => {
            state
                .rate_limiter
                .record_failure(&rate_key, state.login_rate_limit);
            record_login_fraud(&state, &ip, &email).await?;
            return Err(ApiError::invalid_credentials());
        }
    };

    let password_ok = PasswordHasher::verify(&body.password, &record.password_hash)
        .map_err(|_| ApiError::internal())?;

    if !password_ok {
        state
            .rate_limiter
            .record_failure(&rate_key, state.login_rate_limit);
        record_login_fraud(&state, &ip, &email).await?;
        return Err(ApiError::invalid_credentials());
    }

    let auth = match application::auth::authenticate_login(&to_app_record(record), &body.password, true)
    {
        Ok(auth) => auth,
        Err(AppError::Identity(domain_identity::IdentityError::InactiveUser)) => {
            state
                .rate_limiter
                .record_failure(&rate_key, state.login_rate_limit);
            record_login_fraud(&state, &ip, &email).await?;
            return Err(ApiError::invalid_credentials());
        }
        Err(err) => return Err(map_app_error(err)),
    };

    issue_tokens(&state, auth).await
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(body): Json<RefreshRequest>,
) -> Result<Json<TokenResponse>, ApiError> {
    let user_id = state
        .refresh_store
        .resolve_user(&body.refresh_token)
        .await
        .map_err(|_| ApiError::invalid_credentials())?;

    let record = infra_postgres::identity::find_login_record_by_id(&state.admin_pool, user_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::invalid_credentials)?;

    let auth = application::auth::authenticate_login(&to_app_record(record), "", true)
        .map_err(map_app_error)?;

    issue_tokens(&state, auth).await
}

pub async fn logout(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<http::StatusCode, ApiError> {
    state
        .refresh_store
        .revoke(auth.user_id)
        .await
        .map_err(|_| ApiError::internal())?;
    Ok(http::StatusCode::NO_CONTENT)
}

async fn issue_tokens(
    state: &AppState,
    auth: AuthenticatedUser,
) -> Result<Json<TokenResponse>, ApiError> {
    let access = state
        .jwt
        .issue_access_token(
            auth.user_id,
            auth.tenant_id.as_uuid(),
            auth.role.as_str(),
            auth.commerce_id,
        )
        .map_err(|_| ApiError::internal())?;

    let refresh = Uuid::now_v7().to_string();
    state
        .refresh_store
        .store(auth.user_id, &refresh, state.refresh_ttl)
        .await
        .map_err(|_| ApiError::internal())?;

    Ok(Json(TokenResponse {
        access_token: access,
        refresh_token: refresh,
        expires_in: ACCESS_TOKEN_TTL.as_secs(),
    }))
}

fn map_app_error(err: AppError) -> ApiError {
    match err {
        AppError::InvalidCredentials
        | AppError::Identity(domain_identity::IdentityError::InactiveUser) => {
            ApiError::invalid_credentials()
        }
        AppError::Forbidden => ApiError::forbidden(),
        AppError::TenantSuspended => ApiError::tenant_suspended(),
        AppError::Platform(_) => ApiError::bad_request("INVALID_INPUT", "Invalid platform request"),
        AppError::Identity(_) => ApiError::bad_request("INVALID_INPUT", "Invalid request"),
        AppError::Commerce(_) => ApiError::invalid_cnpj(),
        AppError::Billing(_) => ApiError::bad_request("BILLING_ERROR", "Billing operation failed"),
    }
}

fn to_app_record(
    record: infra_postgres::identity::LoginUserRecord,
) -> application::auth::LoginUserRecord {
    application::auth::LoginUserRecord {
        id: record.id,
        tenant_id: record.tenant_id,
        role: record.role,
        password_hash: record.password_hash,
        active: record.active,
        commerce_id: record.commerce_id,
    }
}

async fn record_login_fraud(state: &AppState, ip: &str, email: &str) -> Result<(), ApiError> {
    if let Err(err) = on_login_failure(state, ip, email).await
        && err.code == "FRAUD_BLOCKED"
    {
        return Err(err);
    }
    Ok(())
}

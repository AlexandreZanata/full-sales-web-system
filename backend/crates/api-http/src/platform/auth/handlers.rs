use application::ACCESS_TOKEN_TTL;
use axum::{Json, extract::State, http::HeaderMap};
use infra_crypto::{PasswordHasher, TotpVerifier};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::client_ip::client_ip;
use crate::error::ApiError;
use super::middleware::PlatformAuthUser;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct PlatformLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum PlatformLoginResponse {
    MfaRequired {
        #[serde(rename = "mfaRequired")]
        mfa_required: bool,
        #[serde(rename = "mfaToken")]
        mfa_token: String,
    },
    Tokens(TokenResponse),
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
pub struct PlatformMfaVerifyRequest {
    pub code: String,
    #[serde(rename = "mfaToken")]
    pub mfa_token: String,
}

#[derive(Deserialize)]
pub struct PlatformRefreshRequest {
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

pub async fn platform_login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<PlatformLoginRequest>,
) -> Result<Json<PlatformLoginResponse>, ApiError> {
    let rate_key = format!("ratelimit:platform-login:{}", client_ip(&headers));
    if state
        .rate_limiter
        .is_blocked(&rate_key, state.login_rate_limit)
    {
        return Err(ApiError::rate_limited());
    }

    let email = body.email.trim().to_lowercase();
    let record = infra_postgres::identity::find_platform_user_for_login(&state.admin_pool, &email)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(|| record_login_failure(&state, &rate_key))?;

    let password_ok = PasswordHasher::verify(&body.password, &record.password_hash)
        .map_err(|_| ApiError::internal())?;
    if !password_ok || !record.active {
        return Err(record_login_failure(&state, &rate_key));
    }

    if record.mfa_enrolled && record.mfa_secret.is_some() {
        let mfa_token = state
            .jwt
            .issue_mfa_pending_token(record.id)
            .map_err(|_| ApiError::internal())?;
        return Ok(Json(PlatformLoginResponse::MfaRequired {
            mfa_required: true,
            mfa_token,
        }));
    }

    let tokens = issue_platform_tokens(&state, record.id).await?;
    Ok(Json(PlatformLoginResponse::Tokens(tokens)))
}

pub async fn platform_mfa_verify(
    State(state): State<AppState>,
    Json(body): Json<PlatformMfaVerifyRequest>,
) -> Result<Json<TokenResponse>, ApiError> {
    let claims = state
        .jwt
        .verify_mfa_pending_token(&body.mfa_token)
        .map_err(|_| ApiError::unauthorized())?;

    let record = infra_postgres::identity::find_platform_user_by_id(&state.admin_pool, claims.sub)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::unauthorized)?;

    let secret = record
        .mfa_secret
        .as_deref()
        .ok_or_else(ApiError::unauthorized)?;
    TotpVerifier::from_base32_secret(secret)
        .map_err(|_| ApiError::internal())?
        .verify(body.code.trim())
        .map_err(|_| ApiError::invalid_credentials())?;

    issue_platform_tokens(&state, record.id)
        .await
        .map(Json)
}

pub async fn platform_refresh(
    State(state): State<AppState>,
    Json(body): Json<PlatformRefreshRequest>,
) -> Result<Json<TokenResponse>, ApiError> {
    let user_id = state
        .platform_refresh_store
        .resolve_user(&body.refresh_token)
        .await
        .map_err(|_| ApiError::invalid_credentials())?;

    let record = infra_postgres::identity::find_platform_user_by_id(&state.admin_pool, user_id)
        .await
        .map_err(|_| ApiError::internal())?
        .filter(|r| r.active)
        .ok_or_else(ApiError::invalid_credentials)?;

    issue_platform_tokens(&state, record.id).await.map(Json)
}

pub async fn platform_logout(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
) -> Result<http::StatusCode, ApiError> {
    state
        .platform_refresh_store
        .revoke(auth.user_id)
        .await
        .map_err(|_| ApiError::internal())?;
    Ok(http::StatusCode::NO_CONTENT)
}

async fn issue_platform_tokens(state: &AppState, user_id: Uuid) -> Result<TokenResponse, ApiError> {
    let access = state
        .jwt
        .issue_platform_access_token(user_id, false, None, None, None, None)
        .map_err(|_| ApiError::internal())?;

    let refresh = Uuid::now_v7().to_string();
    state
        .platform_refresh_store
        .store(user_id, &refresh, state.refresh_ttl)
        .await
        .map_err(|_| ApiError::internal())?;

    Ok(TokenResponse {
        access_token: access,
        refresh_token: refresh,
        expires_in: ACCESS_TOKEN_TTL.as_secs(),
    })
}

fn record_login_failure(state: &AppState, rate_key: &str) -> ApiError {
    state
        .rate_limiter
        .record_failure(rate_key, state.login_rate_limit);
    ApiError::invalid_credentials()
}

use axum::extract::FromRequestParts;
use axum::extract::State;
use axum::http::request::Parts;
use axum::response::Response;
use infra_crypto::PLATFORM_ROLE;

use crate::error::ApiError;
use crate::state::AppState;

#[derive(Debug, Clone)]
pub struct PlatformAuthUser {
    pub user_id: uuid::Uuid,
    #[allow(dead_code)] // reserved for PlatformAdmin RLS bypass policy
    pub bypass_rls: bool,
}

impl<S> FromRequestParts<S> for PlatformAuthUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<PlatformAuthUser>()
            .cloned()
            .ok_or_else(ApiError::unauthorized)
    }
}

pub async fn platform_auth_middleware(
    State(state): State<AppState>,
    mut request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<Response, ApiError> {
    let token = bearer_token(request.headers())?;
    let claims = state
        .jwt
        .verify_platform_access_token(token)
        .map_err(|_| ApiError::unauthorized())?;

    if claims.role != PLATFORM_ROLE || claims.impersonating {
        return Err(ApiError::unauthorized());
    }

    request.extensions_mut().insert(PlatformAuthUser {
        user_id: claims.sub,
        bypass_rls: true,
    });

    Ok(next.run(request).await)
}

fn bearer_token(headers: &http::HeaderMap) -> Result<&str, ApiError> {
    let auth_header = headers
        .get(http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(ApiError::unauthorized)?;
    auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(ApiError::unauthorized)
}

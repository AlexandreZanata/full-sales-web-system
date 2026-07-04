use axum::extract::FromRequestParts;
use axum::extract::State;
use axum::http::request::Parts;
use axum::response::Response;
use domain_identity::Role;
use domain_shared::TenantId;

use crate::error::ApiError;
use crate::state::AppState;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: uuid::Uuid,
    pub tenant_id: TenantId,
    pub role: Role,
    pub commerce_id: Option<uuid::Uuid>,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthUser>()
            .cloned()
            .ok_or_else(ApiError::unauthorized)
    }
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<Response, ApiError> {
    let auth_header = request
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(ApiError::unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(ApiError::unauthorized)?;

    let claims = state
        .jwt
        .verify_access_token(token)
        .map_err(|_| ApiError::unauthorized())?;

    let role = Role::parse(&claims.role).map_err(|_| ApiError::unauthorized())?;

    request.extensions_mut().insert(AuthUser {
        user_id: claims.sub,
        tenant_id: TenantId::from_uuid(claims.tenant_id),
        role,
        commerce_id: claims.commerce_id,
    });

    Ok(next.run(request).await)
}

pub fn require_admin(user: &AuthUser) -> Result<(), ApiError> {
    if user.role.can_register_commerce() {
        Ok(())
    } else {
        Err(ApiError::forbidden())
    }
}

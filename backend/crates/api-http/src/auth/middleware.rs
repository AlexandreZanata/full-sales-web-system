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
    pub impersonating: bool,
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
    let token = bearer_token(request.headers())?;

    if let Ok(claims) = state.jwt.verify_access_token(token) {
        let role = Role::parse(&claims.role).map_err(|_| ApiError::unauthorized())?;
        request.extensions_mut().insert(AuthUser {
            user_id: claims.sub,
            tenant_id: TenantId::from_uuid(claims.tenant_id),
            role,
            commerce_id: claims.commerce_id,
            impersonating: false,
        });
        return Ok(next.run(request).await);
    }

    let claims = state
        .jwt
        .verify_platform_access_token(token)
        .map_err(|_| ApiError::unauthorized())?;

    if !claims.impersonating {
        return Err(ApiError::unauthorized());
    }

    let tenant_id = claims.acting_tenant_id.ok_or_else(ApiError::unauthorized)?;
    let acting_role = claims.acting_role.as_deref().unwrap_or("Admin");
    let role = Role::parse(acting_role).map_err(|_| ApiError::unauthorized())?;
    let user_id = claims.acting_user_id.unwrap_or(claims.sub);

    request.extensions_mut().insert(AuthUser {
        user_id,
        tenant_id: TenantId::from_uuid(tenant_id),
        role,
        commerce_id: None,
        impersonating: true,
    });

    Ok(next.run(request).await)
}

pub async fn reject_platform_token_middleware(
    State(state): State<AppState>,
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<Response, ApiError> {
    if let Ok(token) = bearer_token(request.headers())
        && state.jwt.verify_platform_access_token(token).is_ok()
    {
        return Err(ApiError::forbidden());
    }
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

pub fn require_admin(user: &AuthUser) -> Result<(), ApiError> {
    if user.role.can_register_commerce() {
        Ok(())
    } else {
        Err(ApiError::forbidden())
    }
}

pub fn require_roles(user: &AuthUser, allowed: &[Role]) -> Result<(), ApiError> {
    if allowed.contains(&user.role) {
        Ok(())
    } else {
        Err(ApiError::forbidden())
    }
}

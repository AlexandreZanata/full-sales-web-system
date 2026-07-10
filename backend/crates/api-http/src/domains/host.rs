use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::{HeaderMap, Request};
use axum::middleware::Next;
use axum::response::Response;

use crate::error::ApiError;
use crate::portal::resolve_public_catalog_tenant;
use crate::state::AppState;

use super::support::HostTenant;

pub struct PublicTenantId(pub domain_shared::TenantId);

impl<S> FromRequestParts<S> for PublicTenantId
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(host) = parts.extensions.get::<HostTenant>() {
            return Ok(PublicTenantId(host.0));
        }
        Ok(PublicTenantId(resolve_public_catalog_tenant()?))
    }
}

pub async fn host_tenant_middleware(
    State(state): State<AppState>,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, ApiError> {
    if let Some(host) = parse_host(request.headers()) {
        let tenant_id =
            infra_postgres::domains::find_tenant_by_active_hostname(&state.admin_pool, &host)
                .await
                .map_err(|_| ApiError::internal())?;
        if let Some(tenant_id) = tenant_id {
            request.extensions_mut().insert(HostTenant(tenant_id));
        }
    }
    Ok(next.run(request).await)
}

fn parse_host(headers: &HeaderMap) -> Option<String> {
    headers
        .get(http::header::HOST)
        .and_then(|v| v.to_str().ok())
        .map(|host| host.split(':').next().unwrap_or(host).trim().to_lowercase())
        .filter(|host| !host.is_empty())
}

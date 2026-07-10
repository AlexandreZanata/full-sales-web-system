use axum::extract::State;
use axum::http::{Method, Uri};
use axum::response::Response;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;

pub async fn tenant_gate_middleware(
    State(state): State<AppState>,
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<Response, ApiError> {
    if !requires_active_tenant(request.method(), request.uri()) {
        return Ok(next.run(request).await);
    }

    let auth = request
        .extensions()
        .get::<AuthUser>()
        .cloned()
        .ok_or_else(ApiError::unauthorized)?;

    let status = infra_postgres::shared::find_tenant_status(&state.app_pool, auth.tenant_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::unauthorized)?;

    application::tenants::ensure_tenant_active(status).map_err(map_app_error)?;

    Ok(next.run(request).await)
}

fn requires_active_tenant(method: &Method, uri: &Uri) -> bool {
    if matches!(method, &Method::GET | &Method::HEAD | &Method::OPTIONS) {
        return false;
    }
    let path = uri.path();
    if path.starts_with("/v1/billing") {
        return false;
    }
    path.starts_with("/v1/")
}

fn map_app_error(err: application::AppError) -> ApiError {
    match err {
        application::AppError::TenantSuspended => ApiError::tenant_suspended(),
        _ => ApiError::internal(),
    }
}

use axum::extract::State;
use axum::http::{Request, StatusCode, Uri};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

use crate::auth::AuthUser;
use crate::domains::HostTenant;
use crate::error::ApiError;
use crate::state::AppState;

#[derive(Serialize)]
struct MaintenanceErrorBody {
    error: MaintenanceErrorDetail,
}

#[derive(Serialize)]
struct MaintenanceErrorDetail {
    code: &'static str,
    message: String,
    #[serde(rename = "maintenanceEndsAt")]
    maintenance_ends_at: Option<String>,
}

pub async fn maintenance_middleware(
    State(state): State<AppState>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, ApiError> {
    if skip_maintenance_check(request.uri()) {
        return Ok(next.run(request).await);
    }

    if allows_settings_read(request.method(), request.uri()) {
        return Ok(next.run(request).await);
    }

    if let Some(window) = infra_postgres::ops::find_active_global(&state.admin_pool)
        .await
        .map_err(|_| ApiError::internal())?
    {
        return Ok(maintenance_response(&window.message, Some(window.ends_at)));
    }

    if let Some(tenant_id) = tenant_from_request(&request) {
        if let Some(window) =
            infra_postgres::ops::find_active_for_tenant(&state.admin_pool, tenant_id)
                .await
                .map_err(|_| ApiError::internal())?
        {
            if !allows_settings_read(request.method(), request.uri()) {
                return Ok(maintenance_response(&window.message, Some(window.ends_at)));
            }
        }
    }

    Ok(next.run(request).await)
}

fn skip_maintenance_check(uri: &Uri) -> bool {
    let path = uri.path();
    path == "/health" || path.starts_with("/v1/platform")
}

fn allows_settings_read(method: &http::Method, uri: &Uri) -> bool {
    if !method.eq(&http::Method::GET) {
        return false;
    }
    matches!(uri.path(), "/v1/public/settings" | "/v1/settings")
}

fn tenant_from_request(request: &Request<axum::body::Body>) -> Option<domain_shared::TenantId> {
    if let Some(auth) = request.extensions().get::<AuthUser>() {
        return Some(auth.tenant_id);
    }
    request
        .extensions()
        .get::<HostTenant>()
        .map(|host| host.0)
}

fn maintenance_response(message: &str, ends_at: Option<chrono::DateTime<chrono::Utc>>) -> Response {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(MaintenanceErrorBody {
            error: MaintenanceErrorDetail {
                code: "MAINTENANCE",
                message: message.to_owned(),
                maintenance_ends_at: ends_at.map(|t| t.to_rfc3339()),
            },
        }),
    )
        .into_response()
}

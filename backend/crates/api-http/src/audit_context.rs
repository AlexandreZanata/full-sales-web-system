use axum::extract::FromRequestParts;
use axum::http::{HeaderMap, request::Parts};
use uuid::Uuid;

use crate::client_ip::client_ip;
use crate::error::ApiError;

#[derive(Debug, Clone)]
pub struct AuditRequestContext {
    pub ip: String,
    pub correlation_id: Option<Uuid>,
}

impl AuditRequestContext {
    pub fn from_headers(headers: &HeaderMap) -> Self {
        let correlation_id = headers
            .get("x-request-id")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| Uuid::parse_str(s).ok());
        Self {
            ip: client_ip(headers),
            correlation_id,
        }
    }
}

impl<S> FromRequestParts<S> for AuditRequestContext
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuditRequestContext>()
            .cloned()
            .ok_or_else(ApiError::internal)
    }
}

pub async fn audit_context_middleware(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let ctx = AuditRequestContext::from_headers(request.headers());
    let mut request = request;
    request.extensions_mut().insert(ctx);
    next.run(request).await
}

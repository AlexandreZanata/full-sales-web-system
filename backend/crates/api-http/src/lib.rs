use axum::{Json, Router, routing::get};
use http::{HeaderName, Request};
use serde::Serialize;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing::Level;

/// API contract: `GET /health` → `{ "status": "ok" }`.
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

/// Builds the HTTP router with request-id propagation and tracing.
pub fn app() -> Router {
    let request_id = HeaderName::from_static("x-request-id");

    Router::new().route("/health", get(health)).layer(
        ServiceBuilder::new()
            .layer(SetRequestIdLayer::new(request_id.clone(), MakeRequestUuid))
            .layer(PropagateRequestIdLayer::new(request_id))
            .layer(
                TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                    let request_id = request
                        .headers()
                        .get("x-request-id")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("-");
                    tracing::span!(
                        Level::INFO,
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                        request_id = %request_id,
                    )
                }),
            ),
    )
}

/// Initializes structured tracing from `RUST_LOG` (default: `api_http=info,tower_http=info`).
pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api_http=info,tower_http=info".into()),
        )
        .init();
}

/// Binds address from `API_HOST` / `API_PORT` env vars.
pub fn listen_addr() -> String {
    let host = std::env::var("API_HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("API_PORT").unwrap_or_else(|_| "8080".into());
    format!("{host}:{port}")
}

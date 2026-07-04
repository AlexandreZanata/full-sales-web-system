use axum::{Json, Router, routing::get};
use serde::Serialize;

/// API contract: `GET /health` → `{ "status": "ok" }`.
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

/// `GET /v1/` version stub (reserved for future version metadata).
#[derive(Serialize)]
pub struct V1RootResponse {
    pub version: &'static str,
    pub status: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn v1_root() -> Json<V1RootResponse> {
    Json(V1RootResponse {
        version: "1",
        status: "ok",
    })
}

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/v1/", get(v1_root))
}

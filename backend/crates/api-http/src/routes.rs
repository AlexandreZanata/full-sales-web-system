use axum::{Json, Router, routing::get, routing::post};
use serde::Serialize;

use crate::auth::{auth_middleware, login, logout, refresh};
use crate::commerces::create_commerce;
use crate::state::AppState;

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

pub fn health_router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/v1/", get(v1_root))
}

pub fn v1_router(state: AppState) -> Router {
    let public = Router::new()
        .route("/v1/auth/login", post(login))
        .route("/v1/auth/refresh", post(refresh))
        .with_state(state.clone());

    let protected = Router::new()
        .route("/v1/auth/logout", post(logout))
        .route("/v1/commerces", post(create_commerce))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state);

    public.merge(protected)
}

pub fn router() -> Router {
    health_router()
}

pub fn app_with_state(state: AppState) -> Router {
    health_router().merge(v1_router(state))
}

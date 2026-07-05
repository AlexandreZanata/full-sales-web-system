mod admin_orders;
mod audit;
mod auth;
pub mod catalog_events;
mod client_ip;
mod commerces;
mod deliveries;
mod error;
mod inventory;
mod media;
mod pagination;
mod portal;
mod products;
mod reports;
mod routes;
mod sales;
mod session;
mod settings;
mod state;
mod users;
mod validation;

use axum::Router;
use http::HeaderName;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing::Level;

pub use error::ApiError;
pub use routes::{app_with_state, health_router, router};
pub use state::AppState;

/// Builds the HTTP router with request-id propagation and tracing (health only).
pub fn app() -> Router {
    wrap_layers(router())
}

/// Full API router with identity/commerces routes and shared state.
pub fn full_app(state: AppState) -> Router {
    wrap_layers(routes::app_with_state(state))
}

fn wrap_layers(router: Router) -> Router {
    let request_id = HeaderName::from_static("x-request-id");

    router.fallback(error::not_found_handler).layer(
        ServiceBuilder::new()
            .layer(SetRequestIdLayer::new(request_id.clone(), MakeRequestUuid))
            .layer(PropagateRequestIdLayer::new(request_id))
            .layer(
                TraceLayer::new_for_http().make_span_with(|request: &http::Request<_>| {
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

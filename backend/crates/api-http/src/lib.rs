mod admin_orders;
mod audit;
mod audit_context;
mod auth;
mod billing;
pub mod catalog_events;
mod categories;
mod client_ip;
pub mod cnpj_lookup;
mod commerces;
mod deliveries;
mod domains;
mod error;
mod fraud;
pub mod health;
mod health_config;
mod inventory;
mod list_query;
mod maintenance;
mod media;
mod pagination;
mod platform;
mod platform_audit;
mod portal;
mod portal_content;
mod products;
mod reports;
mod routes;
mod sales;
mod session;
mod settings;
mod state;
mod status;
mod tenant_gate;
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

pub use domains::{EmptyDnsTxtResolver, MockDnsTxtResolver, PublicTenantId};
pub use error::ApiError;
pub use health::{AlertConfig, run_health_worker, run_probe_cycle};
pub use list_query::{
    CursorListResponse, CursorPaginationMeta, ListQueryApiError, RouteListConfig,
    build_cursor_page, parse_list_query,
};
pub use routes::{app_with_state, health_router, health_router_liveness_only, router};
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

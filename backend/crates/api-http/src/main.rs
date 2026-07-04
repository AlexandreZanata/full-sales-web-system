use std::net::SocketAddr;
use std::sync::Arc;

use api_http::{AppState, app, full_app, init_tracing, listen_addr};
use application::REFRESH_TOKEN_TTL;
use infra_redis::{InMemoryRefreshTokenStore, RedisRefreshTokenStore, RefreshTokenStore};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    init_tracing();

    let addr: SocketAddr = listen_addr().parse().unwrap_or_else(|err| {
        eprintln!("invalid listen address: {err}");
        std::process::exit(1);
    });

    let listener = TcpListener::bind(addr).await.unwrap_or_else(|err| {
        eprintln!("failed to bind {addr}: {err}");
        std::process::exit(1);
    });

    let router = match build_app().await {
        Ok(router) => router,
        Err(err) => {
            eprintln!("failed to build API: {err}");
            std::process::exit(1);
        }
    };

    info!(%addr, "api-http listening");
    axum::serve(listener, router).await.unwrap_or_else(|err| {
        eprintln!("server error: {err}");
        std::process::exit(1);
    });
}

async fn build_app() -> Result<axum::Router, Box<dyn std::error::Error>> {
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => return Ok(app()),
    };

    let admin_url = std::env::var("DATABASE_ADMIN_URL").unwrap_or(database_url.clone());
    let admin_pool = infra_postgres::connect(&admin_url).await?;
    infra_postgres::migrate(&admin_pool).await?;
    let app_pool = infra_postgres::connect(&database_url).await?;

    let refresh_store: Arc<dyn RefreshTokenStore> =
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            Arc::new(RedisRefreshTokenStore::connect(&redis_url).await?)
        } else {
            Arc::new(InMemoryRefreshTokenStore::new())
        };

    let state = AppState {
        admin_pool,
        app_pool,
        refresh_store,
        idempotency_store: AppState::in_memory_idempotency(),
        jwt: AppState::jwt_from_env(),
        refresh_ttl: REFRESH_TOKEN_TTL,
    };

    Ok(full_app(state))
}

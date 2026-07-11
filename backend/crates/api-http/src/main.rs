use std::net::SocketAddr;
use std::sync::Arc;

use api_http::{AppState, app, full_app, init_tracing, listen_addr};
use application::REFRESH_TOKEN_TTL;
use infra_redis::{
    CnpjMissCache, InMemoryCnpjMissCache, InMemoryVelocityCounter, RedisCnpjMissCache,
    RedisVelocityCounter, VelocityCounter,
};
use infra_redis::{InMemoryRefreshTokenStore, RedisRefreshTokenStore, RefreshTokenStore};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
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
        Err(_) => {
            eprintln!(
                "warning: DATABASE_URL not set — only GET /health is available; \
                 ensure backend/.env exists (see backend/.env.example)"
            );
            return Ok(app());
        }
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

    let platform_refresh_store: Arc<dyn RefreshTokenStore> =
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            Arc::new(RedisRefreshTokenStore::connect(&redis_url).await?)
        } else {
            Arc::new(InMemoryRefreshTokenStore::new())
        };

    let cnpj_miss_cache: Arc<dyn CnpjMissCache> = if let Ok(redis_url) = std::env::var("REDIS_URL")
    {
        Arc::new(RedisCnpjMissCache::connect(&redis_url).await?)
    } else {
        Arc::new(InMemoryCnpjMissCache::new())
    };

    let velocity_counter: Arc<dyn VelocityCounter> =
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            Arc::new(RedisVelocityCounter::connect(&redis_url).await?)
        } else {
            Arc::new(InMemoryVelocityCounter::new())
        };

    let state = AppState {
        admin_pool,
        app_pool,
        refresh_store,
        platform_refresh_store,
        idempotency_store: AppState::in_memory_idempotency(),
        rate_limiter: AppState::in_memory_rate_limiter(),
        login_rate_limit: AppState::default_login_rate_limit(),
        verify_rate_limit: AppState::default_verify_rate_limit(),
        cnpj_lookup_rate_limit: AppState::default_cnpj_lookup_rate_limit(),
        jwt: AppState::jwt_from_env(),
        refresh_ttl: REFRESH_TOKEN_TTL,
        storage: AppState::dev_storage(),
        report_signing_key: AppState::report_signing_key_from_env(),
        catalog_events: AppState::default_catalog_events(),
        cnpj_lookup: api_http::cnpj_lookup::default_cnpj_lookup_provider(),
        cnpj_miss_cache,
        payment_gateway: AppState::payment_gateway_from_env(),
        asaas_webhook_token: AppState::asaas_webhook_token_from_env(),
        credential_encryptor: AppState::credential_encryptor_from_env(),
        settlement_cache: AppState::test_settlement_cache(),
        settlement_rate_limit: AppState::default_settlement_rate_limit(),
        velocity_counter,
        dns_resolver: AppState::empty_dns_resolver(),
        health_config: AppState::health_config_from_env(),
        tenant_asaas_base_url: None,
    };

    tokio::spawn(api_http::health::run_health_worker(state.clone()));

    Ok(full_app(state))
}

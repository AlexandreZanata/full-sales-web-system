use std::sync::Arc;
use std::time::Duration;

use infra_crypto::JwtService;
use infra_postgres::PgPool;
use infra_redis::RefreshTokenStore;

pub const JWT_SECRET_ENV: &str = "JWT_SECRET";

/// Shared application state for HTTP handlers.
#[derive(Clone)]
pub struct AppState {
    pub admin_pool: PgPool,
    pub app_pool: PgPool,
    pub refresh_store: Arc<dyn RefreshTokenStore>,
    pub jwt: JwtService,
    pub refresh_ttl: Duration,
}

impl AppState {
    pub fn jwt_from_env() -> JwtService {
        let secret = std::env::var(JWT_SECRET_ENV).unwrap_or_else(|_| "dev-only-secret".into());
        JwtService::new(secret, Duration::from_secs(15 * 60))
    }
}

use std::sync::Arc;
use std::time::Duration;

use ed25519_dalek::SigningKey;
use infra_crypto::JwtService;
use infra_postgres::PgPool;
use infra_redis::{
    IdempotencyStore, InMemoryIdempotencyStore, InMemoryRateLimiter, RateLimitPolicy, RateLimiter,
    RefreshTokenStore,
};
use infra_storage::{InMemoryObjectStorage, LocalFsObjectStorage, ObjectStorage};

use crate::catalog_events::CatalogEventHub;

pub const JWT_SECRET_ENV: &str = "JWT_SECRET";

/// Shared application state for HTTP handlers.
#[derive(Clone)]
pub struct AppState {
    pub admin_pool: PgPool,
    pub app_pool: PgPool,
    pub refresh_store: Arc<dyn RefreshTokenStore>,
    pub idempotency_store: Arc<dyn IdempotencyStore>,
    pub rate_limiter: Arc<dyn RateLimiter>,
    pub login_rate_limit: RateLimitPolicy,
    pub verify_rate_limit: RateLimitPolicy,
    pub jwt: JwtService,
    pub refresh_ttl: Duration,
    pub storage: Arc<dyn ObjectStorage>,
    pub report_signing_key: Option<SigningKey>,
    pub catalog_events: Arc<CatalogEventHub>,
}

impl AppState {
    pub fn jwt_from_env() -> JwtService {
        let secret = std::env::var(JWT_SECRET_ENV).unwrap_or_else(|_| "dev-only-secret".into());
        JwtService::new(secret, Duration::from_secs(15 * 60))
    }

    pub fn in_memory_idempotency() -> Arc<dyn IdempotencyStore> {
        Arc::new(InMemoryIdempotencyStore::new())
    }

    pub fn in_memory_rate_limiter() -> Arc<dyn RateLimiter> {
        Arc::new(InMemoryRateLimiter::new())
    }

    pub fn default_login_rate_limit() -> RateLimitPolicy {
        RateLimitPolicy {
            max: 5,
            window: Duration::from_secs(60),
        }
    }

    pub fn default_verify_rate_limit() -> RateLimitPolicy {
        RateLimitPolicy {
            max: 60,
            window: Duration::from_secs(60),
        }
    }

    pub fn default_catalog_events() -> Arc<CatalogEventHub> {
        CatalogEventHub::new()
    }

    pub fn in_memory_storage() -> Arc<dyn ObjectStorage> {
        Arc::new(InMemoryObjectStorage::new())
    }

    pub fn dev_storage() -> Arc<dyn ObjectStorage> {
        if let Ok(path) = std::env::var("MEDIA_LOCAL_PATH") {
            if let Ok(storage) = LocalFsObjectStorage::new(path) {
                return Arc::new(storage);
            }
        }
        if let Ok(storage) = LocalFsObjectStorage::new(".local/object-storage") {
            return Arc::new(storage);
        }
        Arc::new(InMemoryObjectStorage::new())
    }

    /// Deterministic Ed25519 key for integration tests (do not use in production).
    pub fn test_signing_key() -> SigningKey {
        SigningKey::from_bytes(&[7u8; 32])
    }

    pub fn report_signing_key_from_env() -> Option<SigningKey> {
        let hex_key = std::env::var("REPORT_SIGNING_KEY_HEX").ok()?;
        let bytes = hex::decode(hex_key.trim()).ok()?;
        let array: [u8; 32] = bytes.try_into().ok()?;
        Some(SigningKey::from_bytes(&array))
    }
}

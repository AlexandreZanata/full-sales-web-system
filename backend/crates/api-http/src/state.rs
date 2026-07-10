use std::sync::Arc;
use std::time::Duration;

use application::billing::PaymentGateway;
use application::domains::DnsTxtResolver;
use ed25519_dalek::SigningKey;
use infra_crypto::{CredentialEncryptor, JwtService};
use infra_postgres::PgPool;
use infra_redis::{
    IdempotencyStore, InMemoryIdempotencyStore, InMemoryRateLimiter, InMemoryVelocityCounter,
    RateLimitPolicy, RateLimiter, RefreshTokenStore, VelocityCounter, CnpjMissCache,
    InMemoryCnpjMissCache,
};
use infra_storage::{InMemoryObjectStorage, LocalFsObjectStorage, ObjectStorage};

use crate::catalog_events::CatalogEventHub;
use crate::cnpj_lookup::CnpjLookupProvider;
use crate::settings::payments::SettlementCache;

pub const JWT_SECRET_ENV: &str = "JWT_SECRET";

/// Shared application state for HTTP handlers.
#[derive(Clone)]
pub struct AppState {
    pub admin_pool: PgPool,
    pub app_pool: PgPool,
    pub refresh_store: Arc<dyn RefreshTokenStore>,
    pub platform_refresh_store: Arc<dyn RefreshTokenStore>,
    pub idempotency_store: Arc<dyn IdempotencyStore>,
    pub rate_limiter: Arc<dyn RateLimiter>,
    pub login_rate_limit: RateLimitPolicy,
    pub verify_rate_limit: RateLimitPolicy,
    pub cnpj_lookup_rate_limit: RateLimitPolicy,
    pub jwt: JwtService,
    pub refresh_ttl: Duration,
    pub storage: Arc<dyn ObjectStorage>,
    pub report_signing_key: Option<SigningKey>,
    pub catalog_events: Arc<CatalogEventHub>,
    pub cnpj_lookup: Arc<dyn CnpjLookupProvider>,
    pub cnpj_miss_cache: Arc<dyn CnpjMissCache>,
    pub payment_gateway: Arc<dyn PaymentGateway>,
    pub asaas_webhook_token: Option<String>,
    pub credential_encryptor: Option<Arc<CredentialEncryptor>>,
    pub settlement_cache: Arc<SettlementCache>,
    pub settlement_rate_limit: RateLimitPolicy,
    pub velocity_counter: Arc<dyn VelocityCounter>,
    pub dns_resolver: Arc<dyn DnsTxtResolver>,
    /// ponytail: integration tests point tenant Asaas client at wiremock without env mutation.
    pub tenant_asaas_base_url: Option<String>,
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

    pub fn default_cnpj_lookup_rate_limit() -> RateLimitPolicy {
        RateLimitPolicy {
            max: 30,
            window: Duration::from_secs(60),
        }
    }

    pub fn default_settlement_rate_limit() -> RateLimitPolicy {
        RateLimitPolicy {
            max: 30,
            window: Duration::from_secs(60),
        }
    }

    pub fn credential_encryptor_from_env() -> Option<Arc<CredentialEncryptor>> {
        CredentialEncryptor::from_env().ok().map(Arc::new)
    }

    pub fn mock_cnpj_lookup() -> Arc<dyn CnpjLookupProvider> {
        Arc::new(crate::cnpj_lookup::MockCnpjLookup)
    }

    pub fn mock_payment_gateway() -> Arc<dyn PaymentGateway> {
        Arc::new(infra_asaas::MockPaymentGateway)
    }

    pub fn payment_gateway_from_env() -> Arc<dyn PaymentGateway> {
        infra_asaas::gateway_from_env()
    }

    pub fn asaas_webhook_token_from_env() -> Option<String> {
        crate::billing::webhook_token_from_env()
    }

    pub fn in_memory_cnpj_miss_cache() -> Arc<dyn CnpjMissCache> {
        Arc::new(InMemoryCnpjMissCache::new())
    }

    pub fn default_catalog_events() -> Arc<CatalogEventHub> {
        CatalogEventHub::new()
    }

    pub fn in_memory_storage() -> Arc<dyn ObjectStorage> {
        Arc::new(InMemoryObjectStorage::new())
    }

    pub fn dev_storage() -> Arc<dyn ObjectStorage> {
        if let Ok(path) = std::env::var("MEDIA_LOCAL_PATH")
            && let Ok(storage) = LocalFsObjectStorage::new(path)
        {
            return Arc::new(storage);
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

    pub fn in_memory_refresh_stores() -> (
        Arc<dyn RefreshTokenStore>,
        Arc<dyn RefreshTokenStore>,
    ) {
        let store = Arc::new(infra_redis::InMemoryRefreshTokenStore::new());
        (store.clone(), store)
    }

    pub fn test_credential_encryptor() -> Arc<CredentialEncryptor> {
        use base64::{Engine, engine::general_purpose::STANDARD};
        let key = STANDARD.encode([9u8; 32]);
        Arc::new(
            CredentialEncryptor::from_master_key_b64(&key, 1).expect("test credential encryptor"),
        )
    }

    pub fn test_settlement_cache() -> Arc<SettlementCache> {
        Arc::new(SettlementCache::new(Duration::from_secs(60)))
    }

    pub fn in_memory_velocity_counter() -> Arc<dyn VelocityCounter> {
        Arc::new(InMemoryVelocityCounter::new())
    }

    pub fn empty_dns_resolver() -> Arc<dyn DnsTxtResolver> {
        Arc::new(crate::domains::EmptyDnsTxtResolver)
    }
}

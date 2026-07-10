use infra_storage::StorageConfig;

/// Runtime configuration for dependency health probes.
#[derive(Debug, Clone, Default)]
pub struct HealthConfig {
    pub redis_url: Option<String>,
    pub storage_config: Option<StorageConfig>,
}

impl HealthConfig {
    pub fn from_env() -> Self {
        Self {
            redis_url: std::env::var("REDIS_URL").ok(),
            storage_config: StorageConfig::from_env().ok(),
        }
    }
}

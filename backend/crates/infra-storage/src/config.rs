use std::env;

use crate::error::StorageError;

/// S3-compatible storage configuration (MinIO or cloud bucket).
#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    pub region: String,
}

impl StorageConfig {
    pub fn from_env() -> Result<Self, StorageError> {
        let endpoint = env_var("STORAGE_ENDPOINT")?;
        let access_key = env_var("STORAGE_ACCESS_KEY")?;
        let secret_key = env_var("STORAGE_SECRET_KEY")?;
        let bucket = env_var("STORAGE_BUCKET")?;
        let region = env::var("STORAGE_REGION").unwrap_or_else(|_| "us-east-1".to_owned());
        Ok(Self {
            endpoint,
            access_key,
            secret_key,
            bucket,
            region,
        })
    }
}

fn env_var(name: &str) -> Result<String, StorageError> {
    env::var(name).map_err(|_| StorageError::Config(format!("missing {name}")))
}

use std::time::Instant;

use crate::config::StorageConfig;
use crate::error::StorageError;
use crate::s3::S3ObjectStorage;

/// Head-bucket check for S3-compatible storage (MinIO).
pub async fn head_bucket(config: &StorageConfig) -> Result<u64, StorageError> {
    let storage = S3ObjectStorage::from_config(config).await;
    let started = Instant::now();
    storage
        .head_bucket(&config.bucket)
        .await
        .map(|_| started.elapsed().as_millis() as u64)
}

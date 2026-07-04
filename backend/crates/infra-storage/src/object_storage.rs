use std::time::Duration;

use async_trait::async_trait;

/// Default presigned GET TTL — ~15 minutes (ENTITY-SPEC / ADR-011).
pub const DEFAULT_PRESIGN_TTL_SECS: u64 = 900;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresignedUrl {
    pub url: String,
    pub expires_in_secs: u64,
}

#[async_trait]
pub trait ObjectStorage: Send + Sync {
    async fn put_object(
        &self,
        bucket: &str,
        key: &str,
        bytes: &[u8],
        content_type: &str,
    ) -> Result<(), crate::error::StorageError>;

    async fn delete_object(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<(), crate::error::StorageError>;

    async fn presigned_get(
        &self,
        bucket: &str,
        key: &str,
        ttl: Duration,
    ) -> Result<PresignedUrl, crate::error::StorageError>;
}

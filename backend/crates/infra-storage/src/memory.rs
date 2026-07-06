use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::error::StorageError;
use crate::object_storage::{ObjectStorage, PresignedUrl};

#[derive(Debug, Clone)]
struct StoredObject {
    bytes: Vec<u8>,
    content_type: String,
}

/// In-memory object storage for tests — presigned URLs use the `memory://` scheme.
#[derive(Debug, Default)]
pub struct InMemoryObjectStorage {
    objects: Arc<RwLock<HashMap<(String, String), StoredObject>>>,
}

impl InMemoryObjectStorage {
    pub fn new() -> Self {
        Self::default()
    }

    /// Resolves a presigned URL produced by this store.
    pub async fn resolve_presigned(&self, url: &str) -> Result<(Vec<u8>, String), StorageError> {
        let (bucket, key) = parse_memory_url(url)?;
        let guard = self.objects.read().await;
        let obj = guard.get(&(bucket, key)).ok_or(StorageError::NotFound)?;
        Ok((obj.bytes.clone(), obj.content_type.clone()))
    }
}

#[async_trait]
impl ObjectStorage for InMemoryObjectStorage {
    async fn put_object(
        &self,
        bucket: &str,
        key: &str,
        bytes: &[u8],
        content_type: &str,
    ) -> Result<(), StorageError> {
        let mut guard = self.objects.write().await;
        guard.insert(
            (bucket.to_owned(), key.to_owned()),
            StoredObject {
                bytes: bytes.to_vec(),
                content_type: content_type.to_owned(),
            },
        );
        Ok(())
    }

    async fn delete_object(&self, bucket: &str, key: &str) -> Result<(), StorageError> {
        let mut guard = self.objects.write().await;
        guard
            .remove(&(bucket.to_owned(), key.to_owned()))
            .ok_or(StorageError::NotFound)?;
        Ok(())
    }

    async fn presigned_get(
        &self,
        bucket: &str,
        key: &str,
        ttl: Duration,
    ) -> Result<PresignedUrl, StorageError> {
        let guard = self.objects.read().await;
        if !guard.contains_key(&(bucket.to_owned(), key.to_owned())) {
            return Err(StorageError::NotFound);
        }
        let expires = ttl.as_secs().max(1);
        Ok(PresignedUrl {
            url: format!("memory://{bucket}/{key}?ttl={expires}"),
            expires_in_secs: expires,
        })
    }

    async fn get_object(&self, bucket: &str, key: &str) -> Result<(Vec<u8>, String), StorageError> {
        let guard = self.objects.read().await;
        let obj = guard
            .get(&(bucket.to_owned(), key.to_owned()))
            .ok_or(StorageError::NotFound)?;
        Ok((obj.bytes.clone(), obj.content_type.clone()))
    }
}

fn parse_memory_url(url: &str) -> Result<(String, String), StorageError> {
    let rest = url
        .strip_prefix("memory://")
        .ok_or(StorageError::InvalidPresignedUrl)?;
    let path = rest.split('?').next().unwrap_or(rest);
    let mut parts = path.splitn(2, '/');
    let bucket = parts
        .next()
        .filter(|s| !s.is_empty())
        .ok_or(StorageError::InvalidPresignedUrl)?
        .to_owned();
    let key = parts
        .next()
        .filter(|s| !s.is_empty())
        .ok_or(StorageError::InvalidPresignedUrl)?
        .to_owned();
    Ok((bucket, key))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object_storage::DEFAULT_PRESIGN_TTL_SECS;

    #[tokio::test]
    async fn default_presign_ttl_is_fifteen_minutes() {
        assert_eq!(DEFAULT_PRESIGN_TTL_SECS, 900);
    }
}

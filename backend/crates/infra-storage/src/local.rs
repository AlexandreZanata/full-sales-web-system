use std::path::{Path, PathBuf};
use std::time::Duration;

use async_trait::async_trait;
use tokio::fs;

use crate::error::StorageError;
use crate::object_storage::{ObjectStorage, PresignedUrl};

/// Dev-friendly object storage — persists bytes under a local directory.
pub struct LocalFsObjectStorage {
    base: PathBuf,
}

impl LocalFsObjectStorage {
    pub fn new(base: impl AsRef<Path>) -> Result<Self, StorageError> {
        let base = base.as_ref().to_path_buf();
        std::fs::create_dir_all(&base).map_err(|err| StorageError::Operation(err.to_string()))?;
        Ok(Self { base })
    }

    fn data_path(&self, bucket: &str, key: &str) -> PathBuf {
        self.base.join(bucket).join(key)
    }

    fn meta_path(&self, bucket: &str, key: &str) -> PathBuf {
        self.base.join(bucket).join(format!("{key}.meta"))
    }

    async fn ensure_parent(path: &Path) -> Result<(), StorageError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|err| StorageError::Operation(err.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl ObjectStorage for LocalFsObjectStorage {
    async fn put_object(
        &self,
        bucket: &str,
        key: &str,
        bytes: &[u8],
        content_type: &str,
    ) -> Result<(), StorageError> {
        let data_path = self.data_path(bucket, key);
        let meta_path = self.meta_path(bucket, key);
        Self::ensure_parent(&data_path).await?;
        fs::write(&data_path, bytes)
            .await
            .map_err(|err| StorageError::Operation(err.to_string()))?;
        fs::write(&meta_path, content_type)
            .await
            .map_err(|err| StorageError::Operation(err.to_string()))?;
        Ok(())
    }

    async fn delete_object(&self, bucket: &str, key: &str) -> Result<(), StorageError> {
        let data_path = self.data_path(bucket, key);
        let meta_path = self.meta_path(bucket, key);
        let removed_data = fs::remove_file(&data_path).await.is_ok();
        let _ = fs::remove_file(&meta_path).await;
        if removed_data {
            Ok(())
        } else {
            Err(StorageError::NotFound)
        }
    }

    async fn presigned_get(
        &self,
        bucket: &str,
        key: &str,
        ttl: Duration,
    ) -> Result<PresignedUrl, StorageError> {
        if !self.data_path(bucket, key).is_file() {
            return Err(StorageError::NotFound);
        }
        let expires = ttl.as_secs().max(1);
        Ok(PresignedUrl {
            url: format!("memory://{bucket}/{key}?ttl={expires}"),
            expires_in_secs: expires,
        })
    }

    async fn get_object(&self, bucket: &str, key: &str) -> Result<(Vec<u8>, String), StorageError> {
        let data_path = self.data_path(bucket, key);
        let meta_path = self.meta_path(bucket, key);
        let bytes = fs::read(&data_path)
            .await
            .map_err(|_| StorageError::NotFound)?;
        let content_type = fs::read_to_string(&meta_path)
            .await
            .unwrap_or_else(|_| "application/octet-stream".to_owned());
        Ok((bytes, content_type))
    }
}

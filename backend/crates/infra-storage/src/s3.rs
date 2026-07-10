use std::time::Duration;

use async_trait::async_trait;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::config::Region;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::primitives::ByteStream;

use crate::config::StorageConfig;
use crate::error::StorageError;
use crate::object_storage::{ObjectStorage, PresignedUrl};

/// S3-compatible client — works with MinIO via custom endpoint (ADR-011).
pub struct S3ObjectStorage {
    client: Client,
}

impl S3ObjectStorage {
    pub async fn from_config(config: &StorageConfig) -> Self {
        let credentials = Credentials::new(
            &config.access_key,
            &config.secret_key,
            None,
            None,
            "infra-storage",
        );
        let sdk_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(Region::new(config.region.clone()))
            .credentials_provider(credentials)
            .endpoint_url(&config.endpoint)
            .load()
            .await;
        let s3_config = aws_sdk_s3::Config::from(&sdk_config)
            .to_builder()
            .force_path_style(true)
            .build();
        Self {
            client: Client::from_conf(s3_config),
        }
    }
}

#[async_trait]
impl ObjectStorage for S3ObjectStorage {
    async fn put_object(
        &self,
        bucket: &str,
        key: &str,
        bytes: &[u8],
        content_type: &str,
    ) -> Result<(), StorageError> {
        self.client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(ByteStream::from(bytes.to_vec()))
            .content_type(content_type)
            .send()
            .await
            .map_err(|err| StorageError::Operation(err.to_string()))?;
        Ok(())
    }

    async fn delete_object(&self, bucket: &str, key: &str) -> Result<(), StorageError> {
        self.client
            .delete_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|err| StorageError::Operation(err.to_string()))?;
        Ok(())
    }

    async fn presigned_get(
        &self,
        bucket: &str,
        key: &str,
        ttl: Duration,
    ) -> Result<PresignedUrl, StorageError> {
        let presigning = PresigningConfig::expires_in(ttl)
            .map_err(|err| StorageError::Presign(err.to_string()))?;
        let presigned = self
            .client
            .get_object()
            .bucket(bucket)
            .key(key)
            .presigned(presigning)
            .await
            .map_err(|err| StorageError::Presign(err.to_string()))?;
        Ok(PresignedUrl {
            url: presigned.uri().to_string(),
            expires_in_secs: ttl.as_secs(),
        })
    }

    async fn get_object(&self, bucket: &str, key: &str) -> Result<(Vec<u8>, String), StorageError> {
        let response = self
            .client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|err| StorageError::Operation(err.to_string()))?;
        let content_type = response
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_owned();
        let bytes = response
            .body
            .collect()
            .await
            .map_err(|err| StorageError::Operation(err.to_string()))?
            .into_bytes()
            .to_vec();
        Ok((bytes, content_type))
    }
}

impl S3ObjectStorage {
    pub async fn head_bucket(&self, bucket: &str) -> Result<(), StorageError> {
        self.client
            .head_bucket()
            .bucket(bucket)
            .send()
            .await
            .map_err(|err| StorageError::Operation(err.to_string()))?;
        Ok(())
    }
}

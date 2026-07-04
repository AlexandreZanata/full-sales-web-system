//! Object storage adapters — S3-compatible (MinIO) and in-memory test double.

pub mod config;
pub mod error;
pub mod memory;
pub mod object_storage;
pub mod s3;

pub use config::StorageConfig;
pub use error::StorageError;
pub use memory::InMemoryObjectStorage;
pub use object_storage::{ObjectStorage, PresignedUrl, DEFAULT_PRESIGN_TTL_SECS};
pub use s3::S3ObjectStorage;

//! Object storage adapters — S3-compatible (MinIO) and in-memory test double.

pub mod config;
pub mod error;
pub mod local;
pub mod memory;
pub mod object_storage;
pub mod s3;

pub use config::StorageConfig;
pub use error::StorageError;
pub use local::LocalFsObjectStorage;
pub use memory::InMemoryObjectStorage;
pub use object_storage::{DEFAULT_PRESIGN_TTL_SECS, ObjectStorage, PresignedUrl};
pub use s3::S3ObjectStorage;

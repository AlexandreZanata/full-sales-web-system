use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("storage configuration error: {0}")]
    Config(String),

    #[error("object not found")]
    NotFound,

    #[error("storage operation failed: {0}")]
    Operation(String),

    #[error("presign failed")]
    Presign(String),

    #[error("invalid presigned url")]
    InvalidPresignedUrl,
}

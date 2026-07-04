use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MediaError {
    #[error("invalid file id")]
    InvalidFileId,

    #[error("invalid entity type")]
    InvalidEntityType,

    #[error("invalid mime type")]
    InvalidMimeType,

    #[error("file too large")]
    FileTooLarge,

    #[error("integrity mismatch")]
    IntegrityMismatch,
}

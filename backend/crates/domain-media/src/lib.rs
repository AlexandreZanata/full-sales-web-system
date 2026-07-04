//! Media domain — file metadata validation (RN7) and upload integrity.

pub mod error;
pub mod file;
pub mod file_entity_type;
pub mod file_id;
pub mod upload;

pub use error::MediaError;
pub use file::{File, FileCreateInput};
pub use file_entity_type::FileEntityType;
pub use file_id::FileId;
pub use upload::{MAX_FILE_SIZE_BYTES, compute_sha256, validate_upload};

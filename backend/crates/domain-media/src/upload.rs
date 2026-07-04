use sha2::{Digest, Sha256};

use crate::error::MediaError;

/// Maximum upload size — 5 MiB (RN7 / ENTITY-SPEC-file).
pub const MAX_FILE_SIZE_BYTES: u64 = 5_242_880;

const ALLOWED_MIMES: &[&str] = &["image/jpeg", "image/png", "image/webp"];

/// Computes lowercase hex SHA-256 digest of `bytes`.
pub fn compute_sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    hex::encode(digest)
}

/// Validates mime whitelist, size cap, and SHA-256 integrity before storage write (RN7).
pub fn validate_upload(
    mime_type: &str,
    size_bytes: u64,
    bytes: &[u8],
    claimed_sha256: &str,
) -> Result<(), MediaError> {
    if !ALLOWED_MIMES.contains(&mime_type) {
        return Err(MediaError::InvalidMimeType);
    }
    if size_bytes == 0 || size_bytes > MAX_FILE_SIZE_BYTES {
        return Err(MediaError::FileTooLarge);
    }
    if bytes.len() as u64 != size_bytes {
        return Err(MediaError::FileTooLarge);
    }
    let actual = compute_sha256(bytes);
    if actual != claimed_sha256.to_ascii_lowercase() {
        return Err(MediaError::IntegrityMismatch);
    }
    Ok(())
}

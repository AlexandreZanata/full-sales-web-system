//! RN7 upload validation — contract from ENTITY-SPEC-file T-01..T-03.

use domain_identity::UserId;
use domain_media::{
    File, FileCreateInput, FileEntityType, FileId, MAX_FILE_SIZE_BYTES, MediaError, compute_sha256,
    validate_upload,
};
use domain_shared::TenantId;
use uuid::Uuid;

fn minimal_webp_bytes() -> Vec<u8> {
    vec![
        0x52, 0x49, 0x46, 0x46, 0x24, 0x00, 0x00, 0x00, 0x57, 0x45, 0x42, 0x50, 0x56, 0x50, 0x20,
        0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]
}

#[test]
fn given_exe_mime_when_validate_upload_then_invalid_mime_type() {
    let bytes = b"MZ executable";
    let sha = compute_sha256(bytes);
    let err = validate_upload("application/x-msdownload", bytes.len() as u64, bytes, &sha)
        .expect_err("exe mime must be rejected");
    assert_eq!(err, MediaError::InvalidMimeType);
}

#[test]
fn given_over_max_payload_when_validate_upload_then_file_too_large() {
    let bytes = vec![0u8; (MAX_FILE_SIZE_BYTES + 1) as usize];
    let sha = compute_sha256(&bytes);
    let err = validate_upload("image/png", bytes.len() as u64, &bytes, &sha)
        .expect_err("oversized file must be rejected");
    assert_eq!(err, MediaError::FileTooLarge);
}

#[test]
fn given_valid_webp_when_file_create_then_succeeds() {
    let bytes = minimal_webp_bytes();
    let sha = compute_sha256(&bytes);
    validate_upload("image/webp", bytes.len() as u64, &bytes, &sha).expect("valid webp");

    let file = File::create(FileCreateInput {
        id: FileId::generate(),
        tenant_id: TenantId::generate(),
        entity_type: FileEntityType::Product,
        entity_id: Uuid::now_v7(),
        bucket: "media".to_owned(),
        object_key: "products/test.webp".to_owned(),
        mime_type: "image/webp".to_owned(),
        size_bytes: bytes.len() as u64,
        sha256: sha,
        uploaded_by_user_id: UserId::generate(),
        bytes,
    })
    .expect("file create");
    assert_eq!(file.mime_type(), "image/webp");
}

#[test]
fn given_wrong_sha256_when_validate_upload_then_integrity_mismatch() {
    let bytes = minimal_webp_bytes();
    let err = validate_upload("image/webp", bytes.len() as u64, &bytes, "deadbeef")
        .expect_err("wrong hash must fail");
    assert_eq!(err, MediaError::IntegrityMismatch);
}

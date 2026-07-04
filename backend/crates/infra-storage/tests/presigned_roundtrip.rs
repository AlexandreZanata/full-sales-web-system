//! Presigned URL roundtrip — contract: put → presigned_get → resolve returns same bytes.

use std::time::Duration;

use infra_storage::{InMemoryObjectStorage, ObjectStorage, DEFAULT_PRESIGN_TTL_SECS};

#[tokio::test]
async fn given_stored_object_when_presigned_get_then_roundtrip_bytes_match() {
    let storage = InMemoryObjectStorage::new();
    let bucket = "tenant-media";
    let key = "proofs/delivery-1.webp";
    let payload = b"proof-photo-bytes";

    storage
        .put_object(bucket, key, payload, "image/webp")
        .await
        .expect("put object");

    let presigned = storage
        .presigned_get(
            bucket,
            key,
            Duration::from_secs(DEFAULT_PRESIGN_TTL_SECS),
        )
        .await
        .expect("presigned get");

    assert_eq!(presigned.expires_in_secs, DEFAULT_PRESIGN_TTL_SECS);

    let (bytes, content_type) = storage
        .resolve_presigned(&presigned.url)
        .await
        .expect("resolve presigned url");

    assert_eq!(bytes, payload);
    assert_eq!(content_type, "image/webp");
}

#[tokio::test]
async fn given_missing_object_when_presigned_get_then_not_found() {
    let storage = InMemoryObjectStorage::new();
    let err = storage
        .presigned_get("b", "missing", Duration::from_secs(60))
        .await
        .expect_err("missing key must fail");
    assert!(matches!(err, infra_storage::StorageError::NotFound));
}

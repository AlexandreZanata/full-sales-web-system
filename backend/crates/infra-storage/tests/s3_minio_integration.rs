//! Manual MinIO integration — run with Docker MinIO:
//! `docker run -d --rm --name minio-p7 -p 9000:9000 \
//!   -e MINIO_ROOT_USER=testkey -e MINIO_ROOT_PASSWORD=testsecret \
//!   quay.io/minio/minio server /data`
//! Then: `cargo test -p infra-storage --test s3_minio_integration -- --ignored --nocapture`

use std::time::Duration;

use aws_sdk_s3::Client;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::config::Region;
use infra_storage::{ObjectStorage, S3ObjectStorage, StorageConfig};

async fn minio_client() -> (Client, StorageConfig) {
    let config = StorageConfig {
        endpoint: "http://127.0.0.1:9000".to_owned(),
        access_key: "testkey".to_owned(),
        secret_key: "testsecret".to_owned(),
        bucket: "media-test".to_owned(),
        region: "us-east-1".to_owned(),
    };
    let credentials = Credentials::new(
        &config.access_key,
        &config.secret_key,
        None,
        None,
        "minio-integration-test",
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
    (Client::from_conf(s3_config), config)
}

#[tokio::test]
#[ignore = "requires MinIO on localhost:9000"]
async fn given_minio_when_put_and_presigned_get_then_bytes_match() {
    let (client, config) = minio_client().await;
    let _ = client.create_bucket().bucket(&config.bucket).send().await;

    let storage = S3ObjectStorage::from_config(&config).await;
    let key = format!("manual/{}", uuid::Uuid::now_v7());
    let payload = b"minio-manual-test-bytes";

    storage
        .put_object(&config.bucket, &key, payload, "image/webp")
        .await
        .expect("put to minio");

    let presigned = storage
        .presigned_get(&config.bucket, &key, Duration::from_secs(900))
        .await
        .expect("presign get");

    assert!(presigned.url.starts_with("http"));

    let response = reqwest::get(&presigned.url)
        .await
        .expect("http get presigned");
    assert!(response.status().is_success());
    let body = response.bytes().await.expect("read body");
    assert_eq!(body.as_ref(), payload);

    storage
        .delete_object(&config.bucket, &key)
        .await
        .expect("cleanup object");
}

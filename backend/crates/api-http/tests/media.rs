//! Phase 23 — Media upload contract tests.

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;

use support::{minimal_webp_bytes, request, request_bytes, seed_admin, setup, upload_multipart};

// Contract: invalid mime → 400 before DB row
// T-17-093
#[tokio::test]
async fn contract_upload_when_invalid_mime_then_400() {
    let env = setup().await;
    let (admin_id, admin_token) = seed_admin(&env).await;

    let (status, body) = upload_multipart(
        &env,
        &admin_token,
        "malware.exe",
        "application/x-msdownload",
        b"MZ executable",
        "User",
        admin_id,
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "INVALID_MIME");
}

// Contract: presigned URL non-empty
#[tokio::test]
async fn contract_upload_when_valid_then_presigned_url_non_empty() {
    let env = setup().await;
    let (admin_id, admin_token) = seed_admin(&env).await;
    let webp = minimal_webp_bytes();

    let (upload_status, upload_body) = upload_multipart(
        &env,
        &admin_token,
        "photo.webp",
        "image/webp",
        &webp,
        "User",
        admin_id,
    )
    .await;
    assert_eq!(upload_status, StatusCode::OK);
    let file_id = upload_body["id"].as_str().expect("file id");

    let (url_status, url_body) = request(
        &env,
        "GET",
        &format!("/v1/media/{file_id}/url"),
        Some(&admin_token),
        None,
    )
    .await;

    assert_eq!(url_status, StatusCode::OK);
    let url = url_body["url"].as_str().unwrap_or("");
    assert!(!url.is_empty(), "presigned URL must be non-empty");
    assert!(url_body["expiresAt"].is_string());
}

// Contract: authenticated content download returns image bytes
// T-17-163
#[tokio::test]
async fn contract_upload_when_valid_then_content_endpoint_returns_bytes() {
    let env = setup().await;
    let (admin_id, admin_token) = seed_admin(&env).await;
    let webp = minimal_webp_bytes();

    let (upload_status, upload_body) = upload_multipart(
        &env,
        &admin_token,
        "photo.webp",
        "image/webp",
        &webp,
        "User",
        admin_id,
    )
    .await;
    assert_eq!(upload_status, StatusCode::OK);
    let file_id = upload_body["id"].as_str().expect("file id");

    let (content_status, content_bytes) = request_bytes(
        &env,
        "GET",
        &format!("/v1/media/{file_id}/content"),
        Some(&admin_token),
    )
    .await;

    assert_eq!(content_status, StatusCode::OK);
    assert_eq!(content_bytes, webp);
}

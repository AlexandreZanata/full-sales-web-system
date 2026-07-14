//! Phase 17H — Media MIME/size/url errors (T-17-093..094, T-17-163).
#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use uuid::Uuid;

use support::{minimal_webp_bytes, request, seed_admin, seed_driver, setup, upload_multipart};

// T-17-093 errors
#[tokio::test]
async fn given_upload_when_invalid_mime_or_empty_then_400() {
    let env = setup().await;
    let (admin_id, admin) = seed_admin(&env).await;

    let (mime_st, mime_body) = upload_multipart(
        &env,
        &admin,
        "x.exe",
        "application/x-msdownload",
        b"MZ",
        "User",
        admin_id,
    )
    .await;
    assert_eq!(mime_st, StatusCode::BAD_REQUEST);
    assert_eq!(mime_body["error"]["code"], "INVALID_MIME");

    let (empty_st, empty_body) = upload_multipart(
        &env,
        &admin,
        "empty.webp",
        "image/webp",
        b"",
        "User",
        admin_id,
    )
    .await;
    assert_eq!(empty_st, StatusCode::BAD_REQUEST);
    assert_eq!(empty_body["error"]["code"], "FILE_TOO_LARGE");
}

// T-17-093 / T-17-094 / T-17-163 happy + MEDIA_NOT_FOUND
#[tokio::test]
async fn given_valid_upload_when_url_and_content_then_ok() {
    let env = setup().await;
    let (admin_id, admin) = seed_admin(&env).await;
    let webp = minimal_webp_bytes();
    let (up_st, up) = upload_multipart(
        &env,
        &admin,
        "ok.webp",
        "image/webp",
        &webp,
        "User",
        admin_id,
    )
    .await;
    assert!(up_st.is_success());
    let file_id = up["id"].as_str().expect("id");

    let (url_st, url) = request(
        &env,
        "GET",
        &format!("/v1/media/{file_id}/url"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(url_st, StatusCode::OK);
    assert!(!url["url"].as_str().unwrap_or("").is_empty());

    let missing = Uuid::now_v7();
    let (nf_st, nf) = request(
        &env,
        "GET",
        &format!("/v1/media/{missing}/url"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(nf_st, StatusCode::NOT_FOUND);
    assert_eq!(nf["error"]["code"], "MEDIA_NOT_FOUND");
}

// T-17-093 authz — unauthenticated denied
#[tokio::test]
async fn given_no_token_when_upload_then_401() {
    let env = setup().await;
    let (_, driver) = seed_driver(&env, "media-drv@test.com").await;
    let _ = driver;
    let (st, body) = request(&env, "POST", "/v1/media/upload", None, None).await;
    assert_eq!(st, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["code"], "UNAUTHORIZED");
}

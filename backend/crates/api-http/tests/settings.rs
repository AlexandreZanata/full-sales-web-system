//! Phase 41 — Site settings contract tests.

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{minimal_webp_bytes, request, seed_admin, setup, setup_with_tenant, upload_multipart};

const DEV_SEED_TENANT_ID: &str = "01900001-0000-7000-8000-000000000001";

#[tokio::test]
async fn contract_patch_settings_when_valid_then_display_name_updated() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (patch_status, patch_body) = request(
        &env,
        "PATCH",
        "/v1/settings",
        Some(&admin_token),
        Some(json!({ "displayName": "Branded Tenant" }).to_string()),
    )
    .await;
    assert_eq!(patch_status, StatusCode::OK);
    assert_eq!(patch_body["displayName"], "Branded Tenant");

    let (get_status, get_body) = request(&env, "GET", "/v1/settings", Some(&admin_token), None).await;
    assert_eq!(get_status, StatusCode::OK);
    assert_eq!(get_body["displayName"], "Branded Tenant");
}

#[tokio::test]
async fn contract_patch_settings_when_sales_contact_phone_then_persisted() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (patch_status, patch_body) = request(
        &env,
        "PATCH",
        "/v1/settings",
        Some(&admin_token),
        Some(json!({ "salesContactPhone": "+55 11 98765-4321" }).to_string()),
    )
    .await;
    assert_eq!(patch_status, StatusCode::OK);
    assert_eq!(patch_body["salesContactPhone"], "5511987654321");

    let (get_status, get_body) = request(&env, "GET", "/v1/settings", Some(&admin_token), None).await;
    assert_eq!(get_status, StatusCode::OK);
    assert_eq!(get_body["salesContactPhone"], "5511987654321");
}

#[tokio::test]
async fn contract_get_public_settings_when_seeded_then_sales_contact_phone() {
    let tenant_id = domain_shared::TenantId::parse(DEV_SEED_TENANT_ID).expect("tenant id");
    let env = setup_with_tenant(tenant_id).await;
    let (_, admin_token) = seed_admin(&env).await;

    let (patch_status, _) = request(
        &env,
        "PATCH",
        "/v1/settings",
        Some(&admin_token),
        Some(json!({ "salesContactPhone": "5511987654321" }).to_string()),
    )
    .await;
    assert_eq!(patch_status, StatusCode::OK);

    let (status, body) = request(&env, "GET", "/v1/public/settings", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["displayName"].is_string());
    assert_eq!(body["salesContactPhone"], "5511987654321");
}

#[tokio::test]
async fn contract_patch_settings_when_invalid_phone_then_400() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (status, _) = request(
        &env,
        "PATCH",
        "/v1/settings",
        Some(&admin_token),
        Some(json!({ "salesContactPhone": "123" }).to_string()),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn contract_put_site_logo_when_valid_file_then_logo_file_id_set() {
    let env = setup().await;
    let (admin_id, admin_token) = seed_admin(&env).await;
    let tenant_id = env.tenant_id.as_uuid();
    let webp = minimal_webp_bytes();

    let (upload_status, upload_body) = upload_multipart(
        &env,
        &admin_token,
        "site-logo.webp",
        "image/webp",
        &webp,
        "Tenant",
        tenant_id,
    )
    .await;
    assert_eq!(
        upload_status,
        StatusCode::OK,
        "upload failed: {upload_body:?}"
    );
    let file_id = upload_body["id"].as_str().expect("file id");

    let (put_status, put_body) = request(
        &env,
        "PUT",
        "/v1/settings/logo",
        Some(&admin_token),
        Some(json!({ "fileId": file_id }).to_string()),
    )
    .await;
    assert_eq!(put_status, StatusCode::OK);
    assert_eq!(put_body["logoFileId"].as_str(), Some(file_id));
    assert!(
        put_body["logoUrl"]
            .as_str()
            .is_some_and(|url| url.starts_with("/v1/media/")),
        "expected browser-loadable logo URL, got {:?}",
        put_body["logoUrl"]
    );

    let _ = admin_id;
}

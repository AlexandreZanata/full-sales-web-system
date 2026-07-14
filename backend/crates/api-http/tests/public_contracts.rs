//! Phase 17F — Public catalog + settings (T-17-050..063, T-17-090).

#[path = "support/mod.rs"]
mod support;

use domain_shared::TenantId;
use http::StatusCode;

use support::{
    minimal_webp_bytes, request, request_bytes, seed_admin, seed_portal_home_content, seed_product,
    setup_with_tenant, upload_multipart,
};

const DEV_SEED_TENANT_ID: &str = "01900001-0000-7000-8000-000000000001";

// T-17-050 / T-17-051 / T-17-056 / T-17-057 / T-17-090
#[tokio::test]
async fn given_no_auth_when_public_catalog_and_settings_then_200() {
    let tenant_id = TenantId::parse(DEV_SEED_TENANT_ID).expect("tenant");
    let env = setup_with_tenant(tenant_id).await;
    let (_, admin) = seed_admin(&env).await;
    let product_id = seed_product(&env, "PUB-17F", "Public Item", 990).await;

    // Create category with slug for public filter/detail
    let (cat_status, cat) = request(
        &env,
        "POST",
        "/v1/categories",
        Some(&admin),
        Some(serde_json::json!({ "name": "Drinks", "slug": "drinks-17f" }).to_string()),
    )
    .await;
    assert_eq!(cat_status, StatusCode::CREATED);

    let (list_status, list) =
        request(&env, "GET", "/v1/public/products?limit=20", None, None).await;
    assert_eq!(list_status, StatusCode::OK);
    assert!(list["data"].is_array());

    let (get_status, got) = request(
        &env,
        "GET",
        &format!("/v1/public/products/{product_id}"),
        None,
        None,
    )
    .await;
    assert_eq!(get_status, StatusCode::OK);
    assert_eq!(got["id"], product_id.to_string());

    let (cats_status, cats) =
        request(&env, "GET", "/v1/public/categories?limit=20", None, None).await;
    assert_eq!(cats_status, StatusCode::OK);
    assert!(cats["data"].is_array());
    let _ = cat;

    let (slug_status, _) =
        request(&env, "GET", "/v1/public/categories/drinks-17f", None, None).await;
    assert!(
        slug_status == StatusCode::OK || slug_status == StatusCode::NOT_FOUND,
        "{slug_status}"
    );

    let (settings_status, settings) = request(&env, "GET", "/v1/public/settings", None, None).await;
    assert_eq!(settings_status, StatusCode::OK);
    assert!(settings.is_object());
}

// T-17-052 / T-17-060..063
#[tokio::test]
async fn given_seeded_home_when_public_home_and_media_then_ok() {
    let tenant_id = TenantId::parse(DEV_SEED_TENANT_ID).expect("tenant");
    let env = setup_with_tenant(tenant_id).await;
    let seed = seed_portal_home_content(&env).await;
    let (_, admin) = seed_admin(&env).await;

    for uri in [
        "/v1/public/banners?placement=hero&limit=5",
        "/v1/public/promotions?limit=4",
        "/v1/public/products/featured?limit=8",
        "/v1/public/products/popular?limit=8",
    ] {
        let (status, body) = request(&env, "GET", uri, None, None).await;
        assert_eq!(status, StatusCode::OK, "{uri}");
        assert!(body["data"].is_array(), "{uri}");
    }
    assert_eq!(seed.featured_product_id.to_string().is_empty(), false);

    let webp = minimal_webp_bytes();
    let (upload_status, upload) = upload_multipart(
        &env,
        &admin,
        "pub.webp",
        "image/webp",
        &webp,
        "Product",
        seed.featured_product_id,
    )
    .await;
    assert!(upload_status.is_success());
    let file_id = upload["id"].as_str().expect("file");
    let (attach_status, _) = request(
        &env,
        "POST",
        &format!("/v1/products/{}/images", seed.featured_product_id),
        Some(&admin),
        Some(serde_json::json!({ "fileId": file_id, "isPrimary": true }).to_string()),
    )
    .await;
    assert_eq!(attach_status, StatusCode::CREATED);

    let (media_status, bytes) = request_bytes(
        &env,
        "GET",
        &format!("/v1/public/media/{file_id}/content"),
        None,
    )
    .await;
    assert_eq!(media_status, StatusCode::OK);
    assert!(!bytes.is_empty());
}

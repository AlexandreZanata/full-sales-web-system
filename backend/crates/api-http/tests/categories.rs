//! Product categories API contract tests — Phase 43.

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use support::{
    minimal_webp_bytes, request, request_bytes, seed_admin, setup, setup_with_tenant,
    upload_multipart,
};

#[tokio::test]
async fn contract_create_category_when_listed_then_present() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (create_status, created) = request(
        &env,
        "POST",
        "/v1/categories",
        Some(&admin_token),
        Some(
            json!({
                "name": "Beverages",
                "description": "Drinks",
                "sortOrder": 0
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let category_id = created["id"].as_str().expect("id");
    assert_eq!(created["slug"].as_str(), Some("beverages"));

    let (list_status, list_body) = request(
        &env,
        "GET",
        "/v1/categories?page=1&pageSize=20",
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(list_status, StatusCode::OK);
    assert!(
        list_body["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["id"].as_str() == Some(category_id))
    );
}

#[tokio::test]
async fn contract_assign_category_when_portal_filter_then_product_returned() {
    let tenant_id =
        domain_shared::TenantId::parse("01900001-0000-7000-8000-000000000001").expect("tenant");
    let env = setup_with_tenant(tenant_id).await;
    let (_, admin_token) = seed_admin(&env).await;

    let (cat_status, category) = request(
        &env,
        "POST",
        "/v1/categories",
        Some(&admin_token),
        Some(json!({ "name": "Widgets", "sortOrder": 1 }).to_string()),
    )
    .await;
    assert_eq!(cat_status, StatusCode::CREATED);
    let category_id = category["id"].as_str().expect("category id");
    let slug = category["slug"].as_str().expect("slug");

    let (product_status, product) = request(
        &env,
        "POST",
        "/v1/products",
        Some(&admin_token),
        Some(
            json!({
                "name": "Category Widget",
                "sku": "CAT-WGT-001",
                "priceAmount": 1500,
                "categoryId": category_id
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(product_status, StatusCode::CREATED);
    assert_eq!(product["categorySlug"].as_str(), Some(slug));

    let (category_status, category_body) = request(
        &env,
        "GET",
        &format!("/v1/categories/{category_id}"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(category_status, StatusCode::OK);
    assert!(category_body["productCount"].as_i64().unwrap_or(0) >= 1);

    let (public_status, public_body) = request(
        &env,
        "GET",
        &format!("/v1/public/categories/{slug}"),
        None,
        None,
    )
    .await;
    assert_eq!(public_status, StatusCode::OK);
    assert_eq!(public_body["slug"].as_str(), Some(slug));
    assert!(
        public_body["products"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["sku"].as_str() == Some("CAT-WGT-001"))
    );
}

#[tokio::test]
async fn contract_legacy_category_field_when_create_product_then_rejected() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (status, body) = request(
        &env,
        "POST",
        "/v1/products",
        Some(&admin_token),
        Some(
            json!({
                "name": "Legacy",
                "sku": "LEG-001",
                "priceAmount": 1000,
                "category": "Old Style"
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"].as_str(), Some("VALIDATION_ERROR"));
}

#[tokio::test]
async fn contract_reorder_categories_when_listed_then_order_persists() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (_, first) = request(
        &env,
        "POST",
        "/v1/categories",
        Some(&admin_token),
        Some(json!({ "name": "Alpha", "sortOrder": 0 }).to_string()),
    )
    .await;
    let (_, second) = request(
        &env,
        "POST",
        "/v1/categories",
        Some(&admin_token),
        Some(json!({ "name": "Beta", "sortOrder": 1 }).to_string()),
    )
    .await;
    let first_id = Uuid::parse_str(first["id"].as_str().unwrap()).unwrap();
    let second_id = Uuid::parse_str(second["id"].as_str().unwrap()).unwrap();

    let (reorder_status, _) = request(
        &env,
        "POST",
        "/v1/categories/reorder",
        Some(&admin_token),
        Some(json!({ "orderedIds": [second_id, first_id] }).to_string()),
    )
    .await;
    assert_eq!(reorder_status, StatusCode::NO_CONTENT);

    let (_, list_body) = request(
        &env,
        "GET",
        "/v1/categories?page=1&pageSize=20&active=true",
        Some(&admin_token),
        None,
    )
    .await;
    let items = list_body["items"].as_array().unwrap();
    let beta_index = items
        .iter()
        .position(|item| item["id"].as_str() == Some(second_id.to_string().as_str()))
        .expect("beta");
    let alpha_index = items
        .iter()
        .position(|item| item["id"].as_str() == Some(first_id.to_string().as_str()))
        .expect("alpha");
    assert!(beta_index < alpha_index);
}

#[tokio::test]
async fn contract_category_image_when_linked_then_public_media_content_ok() {
    let tenant_id =
        domain_shared::TenantId::parse("01900001-0000-7000-8000-000000000001").expect("tenant");
    let env = setup_with_tenant(tenant_id).await;
    let (_, admin_token) = seed_admin(&env).await;
    let webp = minimal_webp_bytes();

    let (cat_status, category) = request(
        &env,
        "POST",
        "/v1/categories",
        Some(&admin_token),
        Some(json!({ "name": "Snacks", "sortOrder": 2 }).to_string()),
    )
    .await;
    assert_eq!(cat_status, StatusCode::CREATED);
    let category_id = category["id"].as_str().expect("category id");
    let category_uuid = Uuid::parse_str(category_id).expect("uuid");

    let (upload_status, upload_body) = upload_multipart(
        &env,
        &admin_token,
        "snacks.webp",
        "image/webp",
        &webp,
        "ProductCategory",
        category_uuid,
    )
    .await;
    assert_eq!(upload_status, StatusCode::OK);
    let file_id = upload_body["id"].as_str().expect("file id");

    let (image_status, _) = request(
        &env,
        "PUT",
        &format!("/v1/categories/{category_id}/image"),
        Some(&admin_token),
        Some(json!({ "fileId": file_id }).to_string()),
    )
    .await;
    assert_eq!(image_status, StatusCode::OK);

    let (list_status, list_body) = request(
        &env,
        "GET",
        "/v1/public/categories",
        None,
        None,
    )
    .await;
    assert_eq!(list_status, StatusCode::OK);
    let snacks = list_body
        .as_array()
        .expect("array")
        .iter()
        .find(|item| item["slug"].as_str() == Some("snacks"))
        .expect("snacks category");
    let thumb_url = snacks["thumbUrl"].as_str().expect("thumbUrl");
    assert!(thumb_url.contains(file_id));

    let (media_status, media_bytes) = request_bytes(
        &env,
        "GET",
        &format!("/v1/public/media/{file_id}/content"),
        None,
    )
    .await;
    assert_eq!(media_status, StatusCode::OK);
    assert_eq!(media_bytes, webp);
}

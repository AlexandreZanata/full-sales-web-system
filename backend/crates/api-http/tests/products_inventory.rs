//! Phase 68B — cursor list contract tests for catalog and inventory routes.

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use support::{minimal_webp_bytes, request, seed_admin, setup, upload_multipart};

#[tokio::test]
async fn contract_list_products_when_cursor_envelope_then_data_array() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/products?limit=20&filter[active]=true",
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["data"].is_array());
    assert_eq!(body["pagination"]["has_more"], false);
}

#[tokio::test]
async fn contract_list_products_when_invalid_filter_then_400() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/products?filter[unknown]=x",
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "invalid_filter_field");
}

#[tokio::test]
async fn contract_list_products_when_limit_over_max_then_400() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/products?limit=200",
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "invalid_pagination");
}

#[tokio::test]
async fn contract_create_product_when_listed_then_present_in_data() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (create_status, created) = request(
        &env,
        "POST",
        "/v1/products",
        Some(&admin_token),
        Some(
            json!({
                "name": "New Widget",
                "sku": "NEW-SKU-001",
                "priceAmount": 2500,
                "priceCurrency": "BRL"
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let product_id = created["id"].as_str().expect("id");

    let (list_status, list_body) = request(
        &env,
        "GET",
        "/v1/products?limit=50",
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(list_status, StatusCode::OK);
    assert!(
        list_body["data"]
            .as_array()
            .unwrap()
            .iter()
            .any(|p| p["id"].as_str() == Some(product_id))
    );
}

#[tokio::test]
async fn contract_adjustment_when_positive_then_balance_increases() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (create_status, created) = request(
        &env,
        "POST",
        "/v1/products",
        Some(&admin_token),
        Some(
            json!({
                "name": "Stock Widget",
                "sku": "STK-SKU",
                "priceAmount": 500
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let product_id = created["id"].as_str().expect("id");

    let (before_status, before_body) = request(
        &env,
        "GET",
        &format!("/v1/inventory/products/{product_id}/balance"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(before_status, StatusCode::OK);
    let before = before_body["available"].as_i64().unwrap();

    let (adj_status, _) = request(
        &env,
        "POST",
        "/v1/inventory/movements",
        Some(&admin_token),
        Some(
            json!({
                "productId": product_id,
                "movementType": "Adjustment",
                "quantity": 5,
                "reason": "Initial stock count"
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(adj_status, StatusCode::CREATED);

    let (after_status, after_body) = request(
        &env,
        "GET",
        &format!("/v1/inventory/products/{product_id}/balance"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(after_status, StatusCode::OK);
    assert_eq!(after_body["available"].as_i64().unwrap(), before + 5);
}

#[tokio::test]
async fn contract_adjustment_when_exceeds_balance_then_422() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (create_status, created) = request(
        &env,
        "POST",
        "/v1/products",
        Some(&admin_token),
        Some(
            json!({
                "name": "Low Stock",
                "sku": "LOW-SKU",
                "priceAmount": 100
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let product_id = created["id"].as_str().expect("id");

    let (adj_status, adj_body) = request(
        &env,
        "POST",
        "/v1/inventory/movements",
        Some(&admin_token),
        Some(
            json!({
                "productId": product_id,
                "movementType": "Adjustment",
                "quantity": -10,
                "reason": "Correction"
            })
            .to_string(),
        ),
    )
    .await;

    assert_eq!(adj_status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(adj_body["error"]["code"], "INSUFFICIENT_BALANCE");
}

#[tokio::test]
async fn contract_list_product_images_when_attached_then_returns_data() {
    let env = setup().await;
    let (admin_id, admin_token) = seed_admin(&env).await;

    let (create_status, created) = request(
        &env,
        "POST",
        "/v1/products",
        Some(&admin_token),
        Some(
            json!({
                "name": "Photo Widget",
                "sku": "IMG-SKU",
                "priceAmount": 900
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let product_id = created["id"].as_str().expect("id");

    let webp = minimal_webp_bytes();
    let (upload_status, upload_body) = upload_multipart(
        &env,
        &admin_token,
        "photo.webp",
        "image/webp",
        &webp,
        "Product",
        Uuid::parse_str(product_id).expect("uuid"),
    )
    .await;
    assert_eq!(upload_status, StatusCode::OK);
    let file_id = upload_body["id"].as_str().expect("file id");

    let (attach_status, attach_body) = request(
        &env,
        "POST",
        &format!("/v1/products/{product_id}/images"),
        Some(&admin_token),
        Some(
            json!({
                "fileId": file_id,
                "isPrimary": true
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(attach_status, StatusCode::CREATED);
    let image_id = attach_body["id"].as_str().expect("image id");

    let (list_status, list_body) = request(
        &env,
        "GET",
        &format!("/v1/products/{product_id}/images"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(list_status, StatusCode::OK);
    let items = list_body["data"].as_array().expect("data");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["id"].as_str(), Some(image_id));
    assert_eq!(items[0]["fileId"].as_str(), Some(file_id));
    assert_eq!(items[0]["isPrimary"], true);

    let (products_status, products_body) = request(
        &env,
        "GET",
        "/v1/products?limit=20",
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(products_status, StatusCode::OK);
    let product = products_body["data"]
        .as_array()
        .expect("data")
        .iter()
        .find(|item| item["id"].as_str() == Some(product_id))
        .expect("product in list");
    assert_eq!(product["primaryImageFileId"].as_str(), Some(file_id));
    let image_url = product["primaryImageUrl"]
        .as_str()
        .expect("primaryImageUrl");
    assert!(
        image_url.starts_with("/v1/public/media/"),
        "expected public media url, got {image_url}"
    );

    let _ = admin_id;
}

#[tokio::test]
async fn contract_delete_product_image_when_attached_then_removed() {
    let env = setup().await;
    let admin_token = seed_admin(&env).await.1;

    let (create_status, created) = request(
        &env,
        "POST",
        "/v1/products",
        Some(&admin_token),
        Some(
            json!({
                "name": "Delete Photo Widget",
                "sku": "IMG-DEL",
                "priceAmount": 900
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let product_id = created["id"].as_str().expect("id");

    let webp = minimal_webp_bytes();
    let (upload_status, upload_body) = upload_multipart(
        &env,
        &admin_token,
        "photo.webp",
        "image/webp",
        &webp,
        "Product",
        Uuid::parse_str(product_id).expect("uuid"),
    )
    .await;
    assert_eq!(upload_status, StatusCode::OK);
    let file_id = upload_body["id"].as_str().expect("file id");

    let (attach_status, attach_body) = request(
        &env,
        "POST",
        &format!("/v1/products/{product_id}/images"),
        Some(&admin_token),
        Some(
            json!({
                "fileId": file_id,
                "isPrimary": true
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(attach_status, StatusCode::CREATED);
    let image_id = attach_body["id"].as_str().expect("image id");

    let (delete_status, _) = request(
        &env,
        "DELETE",
        &format!("/v1/products/{product_id}/images/{image_id}"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(delete_status, StatusCode::NO_CONTENT);

    let (list_status, list_body) = request(
        &env,
        "GET",
        &format!("/v1/products/{product_id}/images"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(list_status, StatusCode::OK);
    assert_eq!(list_body["data"].as_array().expect("data").len(), 0);
}

#[tokio::test]
async fn contract_list_products_when_deactivated_then_visible_with_inactive_filter() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (create_status, created) = request(
        &env,
        "POST",
        "/v1/products",
        Some(&admin_token),
        Some(
            json!({
                "name": "Inactive Widget",
                "sku": "INACT-SKU",
                "priceAmount": 100
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let product_id = created["id"].as_str().expect("id");

    let (deactivate_status, _) = request(
        &env,
        "PATCH",
        &format!("/v1/products/{product_id}"),
        Some(&admin_token),
        Some(json!({ "active": false }).to_string()),
    )
    .await;
    assert_eq!(deactivate_status, StatusCode::OK);

    let (inactive_status, inactive_body) = request(
        &env,
        "GET",
        "/v1/products?limit=50&filter[active]=false",
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(inactive_status, StatusCode::OK);
    assert!(
        inactive_body["data"]
            .as_array()
            .unwrap()
            .iter()
            .any(|p| p["id"].as_str() == Some(product_id))
    );

    let (reactivate_status, reactivated) = request(
        &env,
        "PATCH",
        &format!("/v1/products/{product_id}"),
        Some(&admin_token),
        Some(json!({ "active": true }).to_string()),
    )
    .await;
    assert_eq!(reactivate_status, StatusCode::OK);
    assert_eq!(reactivated["active"], true);
}

#[tokio::test]
async fn contract_list_stock_balances_when_products_seeded_then_returns_data() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;
    let product_id = support::seed_product(&env, "BAL-SKU", "Balance Widget", 1_000).await;
    let driver_id =
        support::seed_user(&env, "driver-bal@test.com", "secret123", "Driver", true).await;
    support::seed_driver_stock(&env, driver_id, product_id, 25).await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/inventory/balances?limit=20&filter[sku][like]=BAL-SKU",
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let item = body["data"]
        .as_array()
        .and_then(|items| items.first())
        .expect("item");
    assert_eq!(item["sku"], "BAL-SKU");
    assert_eq!(item["balanceTotal"].as_i64(), Some(25));
    assert_eq!(item["available"].as_i64(), Some(25));
}

//! Phase 17D — Products get/patch/top-selling + authz (T-17-025..029).

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use support::{request, seed_admin, seed_driver, seed_product, setup};

// T-17-025
#[tokio::test]
async fn given_admin_when_create_product_then_201() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let (status, body) = request(
        &env,
        "POST",
        "/v1/products",
        Some(&admin),
        Some(
            json!({
                "name": "Widget",
                "sku": "W-17D",
                "priceAmount": 1500,
                "priceCurrency": "BRL"
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["sku"], "W-17D");
    assert_eq!(body["name"], "Widget");
}

// T-17-027
#[tokio::test]
async fn given_admin_when_get_product_then_200() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let id = seed_product(&env, "GET-SKU", "Get Me", 900).await;
    let (status, body) = request(
        &env,
        "GET",
        &format!("/v1/products/{id}"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["id"], id.to_string());
}

// T-17-027
#[tokio::test]
async fn given_unknown_product_when_get_then_404() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let missing = Uuid::now_v7();
    let (status, body) = request(
        &env,
        "GET",
        &format!("/v1/products/{missing}"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "PRODUCT_NOT_FOUND");
}

// T-17-028
#[tokio::test]
async fn given_admin_when_top_selling_empty_then_200_envelope() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let (status, body) = request(
        &env,
        "GET",
        "/v1/products/top-selling?limit=5",
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["data"].is_array());
    assert_eq!(body["pagination"]["limit"], 5);
    assert_eq!(body["pagination"]["has_more"], false);
}

// T-17-029
#[tokio::test]
async fn given_admin_when_patch_product_then_200() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let id = seed_product(&env, "PATCH-SKU", "Old", 100).await;
    let (status, body) = request(
        &env,
        "PATCH",
        &format!("/v1/products/{id}"),
        Some(&admin),
        Some(json!({ "name": "New Name", "priceAmount": 250, "isFeatured": true }).to_string()),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["name"], "New Name");
    assert_eq!(body["priceAmount"], 250);
    assert_eq!(body["isFeatured"], true);
}

// T-17-025 / T-17-026 / T-17-029 authz
#[tokio::test]
async fn given_driver_when_mutate_or_unauth_list_then_denied() {
    let env = setup().await;
    let id = seed_product(&env, "AUTHZ-SKU", "P", 100).await;
    let (_, driver) = seed_driver(&env, "driver-cat@test.com").await;

    let (post_status, post_body) = request(
        &env,
        "POST",
        "/v1/products",
        Some(&driver),
        Some(json!({ "name": "X", "sku": "X1", "priceAmount": 1 }).to_string()),
    )
    .await;
    assert_eq!(post_status, StatusCode::FORBIDDEN);
    assert_eq!(post_body["error"]["code"], "FORBIDDEN");

    let (patch_status, _) = request(
        &env,
        "PATCH",
        &format!("/v1/products/{id}"),
        Some(&driver),
        Some(json!({ "name": "Nope" }).to_string()),
    )
    .await;
    assert_eq!(patch_status, StatusCode::FORBIDDEN);

    let (list_status, list_body) = request(&env, "GET", "/v1/products", None, None).await;
    assert_eq!(list_status, StatusCode::UNAUTHORIZED);
    assert_eq!(list_body["error"]["code"], "UNAUTHORIZED");
}

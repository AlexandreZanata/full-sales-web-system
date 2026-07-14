//! Phase 17D — Product images authz + 404 (T-17-037..039).

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use support::{request, seed_admin, seed_driver, seed_product, setup};

// T-17-037
#[tokio::test]
async fn given_missing_product_when_list_images_then_404() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let missing = Uuid::now_v7();
    let (status, body) = request(
        &env,
        "GET",
        &format!("/v1/products/{missing}/images"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "PRODUCT_NOT_FOUND");
}

// T-17-037 / T-17-038 / T-17-039 authz
#[tokio::test]
async fn given_driver_when_product_images_then_403() {
    let env = setup().await;
    let product_id = seed_product(&env, "IMG-AUTHZ", "P", 100).await;
    let (_, driver) = seed_driver(&env, "driver-img@test.com").await;

    let (list_status, list_body) = request(
        &env,
        "GET",
        &format!("/v1/products/{product_id}/images"),
        Some(&driver),
        None,
    )
    .await;
    assert_eq!(list_status, StatusCode::FORBIDDEN);
    assert_eq!(list_body["error"]["code"], "FORBIDDEN");

    let (post_status, _) = request(
        &env,
        "POST",
        &format!("/v1/products/{product_id}/images"),
        Some(&driver),
        Some(json!({ "fileId": Uuid::now_v7() }).to_string()),
    )
    .await;
    assert_eq!(post_status, StatusCode::FORBIDDEN);

    let (del_status, _) = request(
        &env,
        "DELETE",
        &format!("/v1/products/{product_id}/images/{}", Uuid::now_v7()),
        Some(&driver),
        None,
    )
    .await;
    assert_eq!(del_status, StatusCode::FORBIDDEN);
}

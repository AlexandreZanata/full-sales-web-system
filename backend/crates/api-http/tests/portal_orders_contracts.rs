//! Phase 17F — Portal catalog + orders (T-17-054..059, T-17-072..077).
#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{
    request, seed_admin, seed_commerce, seed_commerce_contact, seed_delivery_address, seed_driver,
    seed_product, setup,
};

// T-17-054 / T-17-055 / T-17-058 / T-17-059
#[tokio::test]
async fn given_commerce_contact_when_portal_catalog_then_200() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let (_, contact) = seed_commerce_contact(&env, commerce_id, "portal-cat@test.com").await;
    let product_id = seed_product(&env, "PORT-17F", "Portal Item", 1500).await;

    let (cat_status, _) = request(
        &env,
        "POST",
        "/v1/categories",
        Some(&admin),
        Some(json!({ "name": "Portal Cat", "slug": "portal-cat-17f" }).to_string()),
    )
    .await;
    assert_eq!(cat_status, StatusCode::CREATED);

    let (list_status, list) = request(
        &env,
        "GET",
        "/v1/portal/products?limit=20",
        Some(&contact),
        None,
    )
    .await;
    assert_eq!(list_status, StatusCode::OK);
    assert!(list["data"].is_array());

    let (get_status, got) = request(
        &env,
        "GET",
        &format!("/v1/portal/products/{product_id}"),
        Some(&contact),
        None,
    )
    .await;
    assert_eq!(get_status, StatusCode::OK);
    assert_eq!(got["id"], product_id.to_string());

    let (pcats_status, _) = request(
        &env,
        "GET",
        "/v1/portal/categories?limit=20",
        Some(&contact),
        None,
    )
    .await;
    assert_eq!(pcats_status, StatusCode::OK);

    let (slug_status, _) = request(
        &env,
        "GET",
        "/v1/portal/categories/portal-cat-17f",
        Some(&contact),
        None,
    )
    .await;
    assert!(slug_status.is_success() || slug_status == StatusCode::NOT_FOUND);
}

// T-17-072..077
#[tokio::test]
async fn given_commerce_contact_when_order_lifecycle_then_ok() {
    let env = setup().await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let (_, contact) = seed_commerce_contact(&env, commerce_id, "portal-ord@test.com").await;
    let product_id = seed_product(&env, "ORD-17F", "Order Item", 2500).await;
    let address_id = seed_delivery_address(&env, commerce_id).await;

    let (create_status, created) = request(
        &env,
        "POST",
        "/v1/portal/orders",
        Some(&contact),
        Some(
            json!({
                "deliveryAddressId": address_id,
                "items": [{ "productId": product_id, "quantity": 2 }]
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let order_id = created["id"].as_str().expect("id");
    assert_eq!(created["status"], "Draft");

    let (put_status, updated) = request(
        &env,
        "PUT",
        &format!("/v1/portal/orders/{order_id}"),
        Some(&contact),
        Some(
            json!({
                "deliveryAddressId": address_id,
                "items": [{ "productId": product_id, "quantity": 1 }]
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(put_status, StatusCode::OK, "put body={updated}");
    assert_eq!(updated["totalAmount"], 2500);

    let (get_status, got) = request(
        &env,
        "GET",
        &format!("/v1/portal/orders/{order_id}"),
        Some(&contact),
        None,
    )
    .await;
    assert_eq!(get_status, StatusCode::OK);
    assert_eq!(got["id"], order_id);

    let (list_status, list) = request(
        &env,
        "GET",
        "/v1/portal/orders?limit=20",
        Some(&contact),
        None,
    )
    .await;
    assert_eq!(list_status, StatusCode::OK);
    assert!(
        list["data"]
            .as_array()
            .expect("data")
            .iter()
            .any(|o| o["id"] == order_id)
    );

    let (submit_status, submitted) = request(
        &env,
        "POST",
        &format!("/v1/portal/orders/{order_id}/submit"),
        Some(&contact),
        None,
    )
    .await;
    assert_eq!(submit_status, StatusCode::OK);
    assert_eq!(submitted["status"], "PendingApproval");
}

// T-17-074 / T-17-076 — delete draft + driver denied
#[tokio::test]
async fn given_draft_order_when_delete_or_driver_access_then_expected() {
    let env = setup().await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let (_, contact) = seed_commerce_contact(&env, commerce_id, "portal-del@test.com").await;
    let (_, driver) = seed_driver(&env, "driver-portal@test.com").await;
    let product_id = seed_product(&env, "DEL-17F", "Del Item", 1000).await;
    let address_id = seed_delivery_address(&env, commerce_id).await;

    let (create_status, created) = request(
        &env,
        "POST",
        "/v1/portal/orders",
        Some(&contact),
        Some(
            json!({
                "deliveryAddressId": address_id,
                "items": [{ "productId": product_id, "quantity": 1 }]
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let order_id = created["id"].as_str().expect("id");

    let (del_status, _) = request(
        &env,
        "DELETE",
        &format!("/v1/portal/orders/{order_id}"),
        Some(&contact),
        None,
    )
    .await;
    assert_eq!(del_status, StatusCode::NO_CONTENT);

    let (driver_status, driver_body) =
        request(&env, "GET", "/v1/portal/orders", Some(&driver), None).await;
    assert_eq!(driver_status, StatusCode::FORBIDDEN);
    assert_eq!(driver_body["error"]["code"], "FORBIDDEN");
}

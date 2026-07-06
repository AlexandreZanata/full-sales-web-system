//! Phase 26 — Deliveries contract tests (RN5 partial, driver RLS, proof validation).

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use support::{
    minimal_webp_bytes, request, seed_admin, seed_commerce, seed_commerce_contact,
    seed_delivery_address, seed_driver, seed_driver_stock, seed_product, setup, upload_multipart,
};

async fn delivery_through_transit(
    env: &support::TestEnv,
    admin_token: &str,
    driver_token: &str,
    driver_id: Uuid,
    order_id: &str,
) -> String {
    let (picking_status, _) = request(
        env,
        "POST",
        &format!("/v1/orders/{order_id}/start-picking"),
        Some(admin_token),
        None,
    )
    .await;
    assert_eq!(picking_status, StatusCode::OK);

    let (assign_status, assign_body) = request(
        env,
        "POST",
        &format!("/v1/orders/{order_id}/delivery"),
        Some(admin_token),
        Some(json!({ "driverId": driver_id }).to_string()),
    )
    .await;
    assert_eq!(assign_status, StatusCode::CREATED);
    let delivery_id = assign_body["id"].as_str().expect("delivery id").to_owned();

    let (transit_status, _) = request(
        env,
        "POST",
        &format!("/v1/deliveries/{delivery_id}/start-transit"),
        Some(driver_token),
        None,
    )
    .await;
    assert_eq!(transit_status, StatusCode::OK);

    delivery_id
}

async fn portal_order_two_items(
    env: &support::TestEnv,
    contact_token: &str,
    admin_token: &str,
    address_id: Uuid,
    product_a: Uuid,
    product_b: Uuid,
) -> (String, Uuid, Uuid) {
    let (create_status, create_body) = request(
        env,
        "POST",
        "/v1/portal/orders",
        Some(contact_token),
        Some(
            json!({
                "deliveryAddressId": address_id,
                "items": [
                    { "productId": product_a, "quantity": 10 },
                    { "productId": product_b, "quantity": 5 }
                ]
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let order_id = create_body["id"].as_str().expect("order id").to_owned();
    let item_a = create_body["items"][0]["id"]
        .as_str()
        .expect("item a")
        .to_owned();
    let item_b = create_body["items"][1]["id"]
        .as_str()
        .expect("item b")
        .to_owned();

    let (submit_status, _) = request(
        env,
        "POST",
        &format!("/v1/portal/orders/{order_id}/submit"),
        Some(contact_token),
        None,
    )
    .await;
    assert_eq!(submit_status, StatusCode::OK);

    let (approve_status, _) = request(
        env,
        "POST",
        &format!("/v1/orders/{order_id}/approve"),
        Some(admin_token),
        None,
    )
    .await;
    assert_eq!(approve_status, StatusCode::OK);

    (
        order_id,
        Uuid::parse_str(&item_a).expect("uuid"),
        Uuid::parse_str(&item_b).expect("uuid"),
    )
}

// Contract: RN5 — partial delivery sets PartiallyDelivered on order
#[tokio::test]
async fn given_partial_quantities_when_confirm_delivery_then_partially_delivered() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let product_a = seed_product(&env, "DEL-A", "Product A", 1_000).await;
    let product_b = seed_product(&env, "DEL-B", "Product B", 500).await;
    let address_id = seed_delivery_address(&env, commerce_id).await;
    let (contact_id, contact_token) =
        seed_commerce_contact(&env, commerce_id, "partial@store.com").await;
    let _ = contact_id;
    let (driver_id, driver_token) = seed_driver(&env, "partial-driver@test.com").await;
    seed_driver_stock(&env, driver_id, product_a, 20).await;
    seed_driver_stock(&env, driver_id, product_b, 20).await;

    let (order_id, item_a, item_b) = portal_order_two_items(
        &env,
        &contact_token,
        &admin_token,
        address_id,
        product_a,
        product_b,
    )
    .await;

    let delivery_id =
        delivery_through_transit(&env, &admin_token, &driver_token, driver_id, &order_id).await;

    let (upload_status, upload_body) = upload_multipart(
        &env,
        &driver_token,
        "proof.webp",
        "image/webp",
        &minimal_webp_bytes(),
        "Delivery",
        Uuid::parse_str(&delivery_id).expect("delivery uuid"),
    )
    .await;
    assert_eq!(upload_status, StatusCode::OK);
    let proof_id = upload_body["id"].as_str().expect("proof id");

    let (confirm_status, confirm_body) = request(
        &env,
        "POST",
        &format!("/v1/deliveries/{delivery_id}/confirm"),
        Some(&driver_token),
        Some(
            json!({
                "proofFileId": proof_id,
                "items": [
                    { "orderItemId": item_a, "quantityDelivered": 3 },
                    { "orderItemId": item_b, "quantityDelivered": 5 }
                ]
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(confirm_status, StatusCode::OK);
    assert!(confirm_body["saleId"].is_string());

    let (order_status, order_body) = request(
        &env,
        "GET",
        &format!("/v1/orders/{order_id}"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(order_status, StatusCode::OK);
    assert_eq!(order_body["status"], "PartiallyDelivered");
}

// Contract: driver RLS — list returns only assigned deliveries
#[tokio::test]
async fn given_two_drivers_when_driver_lists_deliveries_then_own_only() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let product_id = seed_product(&env, "RLS-SKU", "RLS Product", 1_000).await;
    let address_id = seed_delivery_address(&env, commerce_id).await;
    let (_, contact_token) = seed_commerce_contact(&env, commerce_id, "rls@store.com").await;
    let (driver_a, driver_a_token) = seed_driver(&env, "driver-a@test.com").await;
    let (driver_b, driver_b_token) = seed_driver(&env, "driver-b@test.com").await;
    seed_driver_stock(&env, driver_a, product_id, 10).await;
    seed_driver_stock(&env, driver_b, product_id, 10).await;

    for (contact_token, driver_id, driver_token) in [
        (&contact_token, driver_a, &driver_a_token),
        (&contact_token, driver_b, &driver_b_token),
    ] {
        let (create_status, create_body) = request(
            &env,
            "POST",
            "/v1/portal/orders",
            Some(contact_token),
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
        let order_id = create_body["id"].as_str().expect("order id");
        let (submit_status, _) = request(
            &env,
            "POST",
            &format!("/v1/portal/orders/{order_id}/submit"),
            Some(contact_token),
            None,
        )
        .await;
        assert_eq!(submit_status, StatusCode::OK);
        let (approve_status, _) = request(
            &env,
            "POST",
            &format!("/v1/orders/{order_id}/approve"),
            Some(&admin_token),
            None,
        )
        .await;
        assert_eq!(approve_status, StatusCode::OK);
        let _ =
            delivery_through_transit(&env, &admin_token, driver_token, driver_id, order_id).await;
    }

    let (status, body) = request(&env, "GET", "/v1/deliveries?limit=50", Some(&driver_a_token), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"].as_array().map(|a| a.len()), Some(1));
    assert_eq!(body["data"][0]["driverId"], driver_a.to_string());
}

// Contract: confirm without proofFileId → validation error
#[tokio::test]
async fn given_confirm_without_proof_when_post_then_validation_error() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let product_id = seed_product(&env, "PRF-SKU", "Proof Product", 1_000).await;
    let address_id = seed_delivery_address(&env, commerce_id).await;
    let (_, contact_token) = seed_commerce_contact(&env, commerce_id, "proof@store.com").await;
    let (driver_id, driver_token) = seed_driver(&env, "proof-driver@test.com").await;
    seed_driver_stock(&env, driver_id, product_id, 5).await;

    let (create_status, create_body) = request(
        &env,
        "POST",
        "/v1/portal/orders",
        Some(&contact_token),
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
    let order_id = create_body["id"].as_str().expect("order id");
    let item_id = create_body["items"][0]["id"].as_str().expect("item id");
    let (submit_status, _) = request(
        &env,
        "POST",
        &format!("/v1/portal/orders/{order_id}/submit"),
        Some(&contact_token),
        None,
    )
    .await;
    assert_eq!(submit_status, StatusCode::OK);
    let (approve_status, _) = request(
        &env,
        "POST",
        &format!("/v1/orders/{order_id}/approve"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(approve_status, StatusCode::OK);

    let delivery_id =
        delivery_through_transit(&env, &admin_token, &driver_token, driver_id, order_id).await;

    let (status, body) = request(
        &env,
        "POST",
        &format!("/v1/deliveries/{delivery_id}/confirm"),
        Some(&driver_token),
        Some(json!({ "items": [{ "orderItemId": item_id, "quantityDelivered": 1 }] }).to_string()),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "VALIDATION_ERROR");
}

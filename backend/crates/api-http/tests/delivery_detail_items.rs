//! Contract: GET delivery detail includes order items for driver confirm flow

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{
    request, seed_admin, seed_commerce, seed_commerce_contact, seed_delivery_address, seed_driver,
    seed_driver_stock, seed_product, setup,
};

#[tokio::test]
async fn given_delivery_when_driver_gets_detail_then_order_items_included() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let product_id = seed_product(&env, "DET-ITEM-SKU", "Detail Product", 1_500).await;
    let address_id = seed_delivery_address(&env, commerce_id).await;
    let (_, contact_token) = seed_commerce_contact(&env, commerce_id, "detail-items@store.com").await;
    let (driver_id, driver_token) = seed_driver(&env, "detail-items-driver@test.com").await;
    seed_driver_stock(&env, driver_id, product_id, 10).await;

    let (create_status, create_body) = request(
        &env,
        "POST",
        "/v1/portal/orders",
        Some(&contact_token),
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

    let (assign_status, assign_body) = request(
        &env,
        "POST",
        &format!("/v1/orders/{order_id}/delivery"),
        Some(&admin_token),
        Some(json!({ "driverId": driver_id }).to_string()),
    )
    .await;
    assert_eq!(assign_status, StatusCode::CREATED);
    let delivery_id = assign_body["id"].as_str().expect("delivery id");

    let (status, body) = request(
        &env,
        "GET",
        &format!("/v1/deliveries/{delivery_id}"),
        Some(&driver_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["orderItems"][0]["id"], item_id);
    assert_eq!(body["orderItems"][0]["quantity"], 2);
}

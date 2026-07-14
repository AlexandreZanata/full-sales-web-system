//! Phase 17G — Deliveries happy path (T-17-084..088).
#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use support::{
    minimal_webp_bytes, request, seed_admin, seed_commerce, seed_commerce_contact,
    seed_delivery_address, seed_driver, seed_driver_stock, seed_product, setup, upload_multipart,
};

async fn approved_order(
    env: &support::TestEnv,
    contact: &str,
    admin: &str,
    address_id: Uuid,
    product_id: Uuid,
) -> (String, String) {
    let (st, body) = request(
        env,
        "POST",
        "/v1/portal/orders",
        Some(contact),
        Some(
            json!({
                "deliveryAddressId": address_id,
                "items": [{ "productId": product_id, "quantity": 1 }]
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(st, StatusCode::CREATED);
    let order_id = body["id"].as_str().expect("id").to_owned();
    let item_id = body["items"][0]["id"].as_str().expect("item").to_owned();
    let (sub, _) = request(
        env,
        "POST",
        &format!("/v1/portal/orders/{order_id}/submit"),
        Some(contact),
        None,
    )
    .await;
    assert_eq!(sub, StatusCode::OK);
    let (ap, _) = request(
        env,
        "POST",
        &format!("/v1/orders/{order_id}/approve"),
        Some(admin),
        None,
    )
    .await;
    assert_eq!(ap, StatusCode::OK);
    (order_id, item_id)
}

// T-17-084 / T-17-085 / T-17-086 / T-17-087 / T-17-088
#[tokio::test]
async fn given_driver_when_delivery_lifecycle_then_ok() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let product_id = seed_product(&env, "DEL-17G", "D", 1000).await;
    let address_id = seed_delivery_address(&env, commerce_id).await;
    let (_, contact) = seed_commerce_contact(&env, commerce_id, "del17g@test.com").await;
    let (driver_id, driver) = seed_driver(&env, "del-driver@test.com").await;
    seed_driver_stock(&env, driver_id, product_id, 5).await;
    let (order_id, item_id) = approved_order(&env, &contact, &admin, address_id, product_id).await;

    assert_eq!(
        request(
            &env,
            "POST",
            &format!("/v1/orders/{order_id}/start-picking"),
            Some(&admin),
            None,
        )
        .await
        .0,
        StatusCode::OK
    );

    let (assign_st, assigned) = request(
        &env,
        "POST",
        &format!("/v1/orders/{order_id}/delivery"),
        Some(&admin),
        Some(json!({ "driverId": driver_id }).to_string()),
    )
    .await;
    assert_eq!(assign_st, StatusCode::CREATED);
    assert_eq!(assigned["status"], "Waiting");
    let delivery_id = assigned["id"].as_str().expect("delivery id");

    let (list_st, list) =
        request(&env, "GET", "/v1/deliveries?limit=20", Some(&driver), None).await;
    assert_eq!(list_st, StatusCode::OK);
    assert!(
        list["data"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d["id"] == delivery_id)
    );

    let (get_st, got) = request(
        &env,
        "GET",
        &format!("/v1/deliveries/{delivery_id}"),
        Some(&driver),
        None,
    )
    .await;
    assert_eq!(get_st, StatusCode::OK);
    assert!(got["orderItems"].is_array());

    assert_eq!(
        request(
            &env,
            "POST",
            &format!("/v1/deliveries/{delivery_id}/start-transit"),
            Some(&driver),
            None,
        )
        .await
        .0,
        StatusCode::OK
    );

    let (up_st, up) = upload_multipart(
        &env,
        &driver,
        "proof.webp",
        "image/webp",
        &minimal_webp_bytes(),
        "Delivery",
        Uuid::parse_str(delivery_id).unwrap(),
    )
    .await;
    assert!(up_st.is_success());
    let proof = up["id"].as_str().expect("proof");

    let (cf_st, confirmed) = request(
        &env,
        "POST",
        &format!("/v1/deliveries/{delivery_id}/confirm"),
        Some(&driver),
        Some(
            json!({
                "proofFileId": proof,
                "items": [{ "orderItemId": item_id, "quantityDelivered": 1 }]
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(cf_st, StatusCode::OK);
    assert_eq!(confirmed["status"], "Delivered");
    assert!(confirmed["saleId"].is_string());
}

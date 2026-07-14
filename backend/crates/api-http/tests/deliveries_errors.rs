//! Phase 17G — Deliveries errors + authz (T-17-086..088).
#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use support::{
    request, seed_admin, seed_commerce, seed_commerce_contact, seed_delivery_address, seed_driver,
    seed_driver_stock, seed_product, setup,
};

// T-17-086 / T-17-088 — not found, proof required, admin denied confirm
#[tokio::test]
async fn given_delivery_when_not_found_no_proof_or_admin_then_errors() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let (_, driver) = seed_driver(&env, "err-driver@test.com").await;
    let missing = Uuid::now_v7();
    let (nf_st, nf) = request(
        &env,
        "GET",
        &format!("/v1/deliveries/{missing}"),
        Some(&driver),
        None,
    )
    .await;
    assert_eq!(nf_st, StatusCode::NOT_FOUND);
    assert_eq!(nf["error"]["code"], "DELIVERY_NOT_FOUND");

    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let product_id = seed_product(&env, "PRF-17G", "P", 500).await;
    let address_id = seed_delivery_address(&env, commerce_id).await;
    let (_, contact) = seed_commerce_contact(&env, commerce_id, "prf17g@test.com").await;
    let (driver_id, driver_tok) = seed_driver(&env, "prf-driver@test.com").await;
    seed_driver_stock(&env, driver_id, product_id, 3).await;

    let (st, body) = request(
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
    assert_eq!(st, StatusCode::CREATED);
    let order_id = body["id"].as_str().expect("id");
    let item_id = body["items"][0]["id"].as_str().expect("item");
    assert_eq!(
        request(
            &env,
            "POST",
            &format!("/v1/portal/orders/{order_id}/submit"),
            Some(&contact),
            None,
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(
            &env,
            "POST",
            &format!("/v1/orders/{order_id}/approve"),
            Some(&admin),
            None,
        )
        .await
        .0,
        StatusCode::OK
    );
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
    let delivery_id = assigned["id"].as_str().expect("id");
    assert_eq!(
        request(
            &env,
            "POST",
            &format!("/v1/deliveries/{delivery_id}/start-transit"),
            Some(&driver_tok),
            None,
        )
        .await
        .0,
        StatusCode::OK
    );

    let confirm_body =
        json!({ "items": [{ "orderItemId": item_id, "quantityDelivered": 1 }] }).to_string();
    let (proof_st, proof_body) = request(
        &env,
        "POST",
        &format!("/v1/deliveries/{delivery_id}/confirm"),
        Some(&driver_tok),
        Some(confirm_body.clone()),
    )
    .await;
    assert_eq!(proof_st, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(proof_body["error"]["code"], "PROOF_REQUIRED");

    let (adm_st, adm) = request(
        &env,
        "POST",
        &format!("/v1/deliveries/{delivery_id}/confirm"),
        Some(&admin),
        Some(confirm_body),
    )
    .await;
    assert_eq!(adm_st, StatusCode::FORBIDDEN);
    assert_eq!(adm["error"]["code"], "FORBIDDEN");
}

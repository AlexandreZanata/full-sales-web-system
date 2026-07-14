//! Phase 17G — Admin orders contracts (T-17-078..083).
#[path = "support/mod.rs"]
mod support;
use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use support::{
    request, seed_admin, seed_commerce, seed_commerce_contact, seed_delivery_address, seed_driver,
    seed_driver_stock, seed_order, seed_product, setup,
};
async fn submitted_portal_order(
    env: &support::TestEnv,
    contact: &str,
    address_id: Uuid,
    product_id: Uuid,
    qty: i32,
) -> String {
    let (status, body) = request(
        env,
        "POST",
        "/v1/portal/orders",
        Some(contact),
        Some(
            json!({
                "deliveryAddressId": address_id,
                "items": [{ "productId": product_id, "quantity": qty }]
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let id = body["id"].as_str().expect("id").to_owned();
    let (submit, _) = request(
        env,
        "POST",
        &format!("/v1/portal/orders/{id}/submit"),
        Some(contact),
        None,
    )
    .await;
    assert_eq!(submit, StatusCode::OK);
    id
}

// T-17-078 / T-17-079 / T-17-080 / T-17-083
#[tokio::test]
async fn given_admin_when_approve_and_start_picking_then_ok() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let product_id = seed_product(&env, "ORD-17G", "O", 1000).await;
    let address_id = seed_delivery_address(&env, commerce_id).await;
    let (_, contact) = seed_commerce_contact(&env, commerce_id, "o17g@test.com").await;
    let (driver_id, _) = seed_driver(&env, "stock-o17g@test.com").await;
    seed_driver_stock(&env, driver_id, product_id, 10).await;
    let order_id = submitted_portal_order(&env, &contact, address_id, product_id, 1).await;

    let (list_st, list) = request(&env, "GET", "/v1/orders?limit=20", Some(&admin), None).await;
    assert_eq!(list_st, StatusCode::OK);
    assert!(
        list["data"]
            .as_array()
            .unwrap()
            .iter()
            .any(|o| o["id"] == order_id)
    );

    let (get_st, got) = request(
        &env,
        "GET",
        &format!("/v1/orders/{order_id}"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(get_st, StatusCode::OK);
    assert_eq!(got["status"], "PendingApproval");

    let (ap_st, approved) = request(
        &env,
        "POST",
        &format!("/v1/orders/{order_id}/approve"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(ap_st, StatusCode::OK);
    assert_eq!(approved["status"], "Approved");

    let (pick_st, picking) = request(
        &env,
        "POST",
        &format!("/v1/orders/{order_id}/start-picking"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(pick_st, StatusCode::OK);
    assert_eq!(picking["status"], "Picking");
}

// T-17-079 / T-17-080 / T-17-081 errors
#[tokio::test]
async fn given_order_when_not_found_reject_or_bad_approve_then_errors() {
    let env = setup().await;
    let (admin_id, admin) = seed_admin(&env).await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let missing = Uuid::now_v7();
    let (nf_st, nf) = request(
        &env,
        "GET",
        &format!("/v1/orders/{missing}"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(nf_st, StatusCode::NOT_FOUND);
    assert_eq!(nf["error"]["code"], "ORDER_NOT_FOUND");

    let order_id = seed_order(&env, commerce_id, admin_id).await;
    let (rej_st, rej) = request(
        &env,
        "POST",
        &format!("/v1/orders/{order_id}/reject"),
        Some(&admin),
        Some(json!({ "reason": "" }).to_string()),
    )
    .await;
    assert_eq!(rej_st, StatusCode::BAD_REQUEST);
    assert_eq!(rej["error"]["code"], "REJECTION_REASON_REQUIRED");

    let (stock_st, stock) = request(
        &env,
        "POST",
        &format!("/v1/orders/{order_id}/approve"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(stock_st, StatusCode::CONFLICT);
    assert_eq!(stock["error"]["code"], "INSUFFICIENT_STOCK");
}

// T-17-081 / T-17-082 happy
#[tokio::test]
async fn given_pending_when_reject_or_approved_when_cancel_then_ok() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let product_id = seed_product(&env, "CXL-17G", "C", 800).await;
    let address_id = seed_delivery_address(&env, commerce_id).await;
    let (_, contact) = seed_commerce_contact(&env, commerce_id, "cxl17g@test.com").await;
    let (driver_id, _) = seed_driver(&env, "stock-cxl@test.com").await;
    seed_driver_stock(&env, driver_id, product_id, 5).await;

    let reject_id = submitted_portal_order(&env, &contact, address_id, product_id, 1).await;
    let (rej_st, rejected) = request(
        &env,
        "POST",
        &format!("/v1/orders/{reject_id}/reject"),
        Some(&admin),
        Some(json!({ "reason": "Out of season" }).to_string()),
    )
    .await;
    assert_eq!(rej_st, StatusCode::OK);
    assert_eq!(rejected["status"], "Rejected");

    let cancel_id = submitted_portal_order(&env, &contact, address_id, product_id, 1).await;
    let (ap_st, _) = request(
        &env,
        "POST",
        &format!("/v1/orders/{cancel_id}/approve"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(ap_st, StatusCode::OK);
    let (cx_st, cancelled) = request(
        &env,
        "POST",
        &format!("/v1/orders/{cancel_id}/cancel"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(cx_st, StatusCode::OK);
    assert_eq!(cancelled["status"], "Cancelled");
}

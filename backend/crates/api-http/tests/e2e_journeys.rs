//! Phase 25 — E2E journey tests (portal → delivery → payment → report; media proof).

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use support::{
    minimal_webp_bytes, request, seed_admin, seed_commerce, seed_commerce_contact,
    seed_delivery_address, seed_driver, seed_driver_stock, seed_product, seed_signing_key, setup,
    upload_multipart,
};

async fn portal_order_through_approve(
    env: &support::TestEnv,
    contact_token: &str,
    admin_token: &str,
    address_id: Uuid,
    product_id: Uuid,
    quantity: i32,
) -> String {
    let (create_status, create_body) = request(
        env,
        "POST",
        "/v1/portal/orders",
        Some(contact_token),
        Some(
            json!({
                "deliveryAddressId": address_id,
                "items": [{ "productId": product_id, "quantity": quantity }]
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let order_id = create_body["id"].as_str().expect("order id").to_owned();

    let (submit_status, submit_body) = request(
        env,
        "POST",
        &format!("/v1/portal/orders/{order_id}/submit"),
        Some(contact_token),
        None,
    )
    .await;
    assert_eq!(submit_status, StatusCode::OK);
    assert_eq!(submit_body["status"], "PendingApproval");

    let (approve_status, approve_body) = request(
        env,
        "POST",
        &format!("/v1/orders/{order_id}/approve"),
        Some(admin_token),
        None,
    )
    .await;
    assert_eq!(approve_status, StatusCode::OK);
    assert_eq!(approve_body["status"], "Approved");

    order_id
}

async fn assign_and_start_delivery(
    env: &support::TestEnv,
    admin_token: &str,
    driver_token: &str,
    order_id: &str,
    driver_id: Uuid,
) -> String {
    let (picking_status, picking_body) = request(
        env,
        "POST",
        &format!("/v1/orders/{order_id}/start-picking"),
        Some(admin_token),
        None,
    )
    .await;
    assert_eq!(picking_status, StatusCode::OK);
    assert_eq!(picking_body["status"], "Picking");

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

    let (transit_status, transit_body) = request(
        env,
        "POST",
        &format!("/v1/deliveries/{delivery_id}/start-transit"),
        Some(driver_token),
        None,
    )
    .await;
    assert_eq!(transit_status, StatusCode::OK);
    assert_eq!(transit_body["status"], "InTransit");

    delivery_id
}

// E2E-003 (Phase 25): portal order → delivery → declare payment → signed report
#[tokio::test]
async fn e2e_003_portal_order_delivery_declare_payment_report() {
    let env = setup().await;
    seed_signing_key(&env).await;
    let (_, admin_token) = seed_admin(&env).await;
    let (driver_id, driver_token) = seed_driver(&env, "driver-journey@test.com").await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let address_id = seed_delivery_address(&env, commerce_id).await;
    let product_id = seed_product(&env, "JOURNEY-SKU", "Journey Widget", 1_500).await;
    seed_driver_stock(&env, driver_id, product_id, 20).await;
    let (_, contact_token) =
        seed_commerce_contact(&env, commerce_id, "contact-journey@test.com").await;

    let quantity = 2;
    let order_id = portal_order_through_approve(
        &env,
        &contact_token,
        &admin_token,
        address_id,
        product_id,
        quantity,
    )
    .await;

    let delivery_id =
        assign_and_start_delivery(&env, &admin_token, &driver_token, &order_id, driver_id).await;

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
    let proof_file_id = upload_body["id"].as_str().expect("proof file id");

    let (order_status, order_body) = request(
        &env,
        "GET",
        &format!("/v1/orders/{order_id}"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(order_status, StatusCode::OK);
    let order_item_id = order_body["items"][0]["id"]
        .as_str()
        .expect("order item id");

    let (confirm_status, confirm_body) = request(
        &env,
        "POST",
        &format!("/v1/deliveries/{delivery_id}/confirm"),
        Some(&driver_token),
        Some(
            json!({
                "proofFileId": proof_file_id,
                "items": [{ "orderItemId": order_item_id, "quantityDelivered": quantity }],
                "receivedByName": "Store Manager"
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(confirm_status, StatusCode::OK);
    assert_eq!(confirm_body["status"], "Delivered");
    let sale_id = confirm_body["saleId"]
        .as_str()
        .expect("sale id from delivery confirm");

    let (declare_status, declare_body) = request(
        &env,
        "POST",
        &format!("/v1/sales/{sale_id}/declare-payment"),
        Some(&driver_token),
        Some(
            json!({
                "method": "pix",
                "received": true,
                "notes": "Paid at counter"
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(declare_status, StatusCode::OK);
    assert_eq!(declare_body["id"], sale_id);

    let (report_status, report_body) = request(
        &env,
        "POST",
        "/v1/reports",
        Some(&admin_token),
        Some(
            json!({
                "reportType": "DailyDriver",
                "periodStart": "2026-01-01T00:00:00Z",
                "periodEnd": "2026-12-31T23:59:59Z",
                "driverId": driver_id
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(report_status, StatusCode::CREATED);
    let report_id = report_body["id"].as_str().expect("report id");

    let (verify_status, verify_body) = request(
        &env,
        "GET",
        &format!("/v1/reports/{report_id}/verify"),
        None,
        None,
    )
    .await;
    assert_eq!(verify_status, StatusCode::OK);
    assert_eq!(verify_body["valid"], true);
}

// E2E-004 (Phase 25): media upload scoped to delivery → confirm with proofFileId
#[tokio::test]
async fn e2e_004_media_upload_delivery_proof_confirm() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;
    let (driver_id, driver_token) = seed_driver(&env, "driver-proof@test.com").await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let address_id = seed_delivery_address(&env, commerce_id).await;
    let product_id = seed_product(&env, "PROOF-SKU", "Proof Widget", 900).await;
    seed_driver_stock(&env, driver_id, product_id, 10).await;
    let (_, contact_token) =
        seed_commerce_contact(&env, commerce_id, "contact-proof@test.com").await;

    let order_id = portal_order_through_approve(
        &env,
        &contact_token,
        &admin_token,
        address_id,
        product_id,
        1,
    )
    .await;
    let delivery_id =
        assign_and_start_delivery(&env, &admin_token, &driver_token, &order_id, driver_id).await;
    let delivery_uuid = Uuid::parse_str(&delivery_id).expect("delivery uuid");

    let (upload_status, upload_body) = upload_multipart(
        &env,
        &driver_token,
        "proof.webp",
        "image/webp",
        &minimal_webp_bytes(),
        "Delivery",
        delivery_uuid,
    )
    .await;
    assert_eq!(upload_status, StatusCode::OK);
    let proof_file_id = upload_body["id"].as_str().expect("proof file id");

    let (order_status, order_body) = request(
        &env,
        "GET",
        &format!("/v1/orders/{order_id}"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(order_status, StatusCode::OK);
    let order_item_id = order_body["items"][0]["id"]
        .as_str()
        .expect("order item id");

    let (confirm_status, confirm_body) = request(
        &env,
        "POST",
        &format!("/v1/deliveries/{delivery_id}/confirm"),
        Some(&driver_token),
        Some(
            json!({
                "proofFileId": proof_file_id,
                "items": [{ "orderItemId": order_item_id, "quantityDelivered": 1 }]
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(confirm_status, StatusCode::OK);
    assert!(confirm_body["saleId"].is_string());
}

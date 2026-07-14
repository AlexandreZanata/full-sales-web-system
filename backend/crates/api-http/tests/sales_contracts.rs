//! Phase 17E — Sales route contracts (T-17-044..049).

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use support::{request, seed_commerce, seed_driver, seed_driver_stock, seed_product, setup};

async fn create_pending_sale(
    env: &support::TestEnv,
    driver_token: &str,
    commerce_id: Uuid,
    product_id: Uuid,
) -> Uuid {
    let (status, body) = request(
        env,
        "POST",
        "/v1/sales",
        Some(driver_token),
        Some(
            json!({
                "commerceId": commerce_id,
                "items": [{ "productId": product_id, "quantity": 1 }],
                "paymentMethod": "cash"
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["status"], "Pending");
    Uuid::parse_str(body["id"].as_str().expect("id")).expect("uuid")
}

// T-17-044 / T-17-045 / T-17-047 / T-17-048
#[tokio::test]
async fn given_driver_when_sale_lifecycle_confirm_then_ok() {
    let env = setup().await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let (driver_id, driver) = seed_driver(&env, "driver-sale@test.com").await;
    let product_id = seed_product(&env, "SALE-17E", "Sale Item", 1000).await;
    seed_driver_stock(&env, driver_id, product_id, 5).await;

    let sale_id = create_pending_sale(&env, &driver, commerce_id, product_id).await;

    let (get_status, got) = request(
        &env,
        "GET",
        &format!("/v1/sales/{sale_id}"),
        Some(&driver),
        None,
    )
    .await;
    assert_eq!(get_status, StatusCode::OK);
    assert_eq!(got["id"], sale_id.to_string());

    let (list_status, list) = request(&env, "GET", "/v1/sales?limit=20", Some(&driver), None).await;
    assert_eq!(list_status, StatusCode::OK);
    assert!(
        list["data"]
            .as_array()
            .expect("data")
            .iter()
            .any(|s| s["id"] == sale_id.to_string())
    );

    let (confirm_status, confirmed) = request(
        &env,
        "POST",
        &format!("/v1/sales/{sale_id}/confirm"),
        Some(&driver),
        None,
    )
    .await;
    assert_eq!(confirm_status, StatusCode::OK);
    assert_eq!(confirmed["status"], "Confirmed");

    let (again_status, again_body) = request(
        &env,
        "POST",
        &format!("/v1/sales/{sale_id}/confirm"),
        Some(&driver),
        None,
    )
    .await;
    assert_eq!(again_status, StatusCode::CONFLICT);
    assert_eq!(again_body["error"]["code"], "INVALID_SALE_TRANSITION");
}

// T-17-046
#[tokio::test]
async fn given_pending_sale_when_cancel_then_200() {
    let env = setup().await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let (driver_id, driver) = seed_driver(&env, "driver-cancel@test.com").await;
    let product_id = seed_product(&env, "CXL-17E", "Cancel Item", 1000).await;
    seed_driver_stock(&env, driver_id, product_id, 3).await;
    let sale_id = create_pending_sale(&env, &driver, commerce_id, product_id).await;

    let (status, body) = request(
        &env,
        "POST",
        &format!("/v1/sales/{sale_id}/cancel"),
        Some(&driver),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "Cancelled");
}

// T-17-045 errors / T-17-049 authz
#[tokio::test]
async fn given_sale_when_confirm_without_stock_or_other_driver_declare_then_errors() {
    let env = setup().await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let (_, driver_a) = seed_driver(&env, "driver-a-sale@test.com").await;
    let (_, driver_b) = seed_driver(&env, "driver-b-sale@test.com").await;
    let product_id = seed_product(&env, "NOS-17E", "No Stock", 1000).await;
    // No stock seeded → confirm fails
    let sale_id = create_pending_sale(&env, &driver_a, commerce_id, product_id).await;

    let (confirm_status, confirm_body) = request(
        &env,
        "POST",
        &format!("/v1/sales/{sale_id}/confirm"),
        Some(&driver_a),
        None,
    )
    .await;
    assert_eq!(confirm_status, StatusCode::CONFLICT);
    assert_eq!(confirm_body["error"]["code"], "INSUFFICIENT_STOCK");

    let (decl_status, decl_body) = request(
        &env,
        "POST",
        &format!("/v1/sales/{sale_id}/declare-payment"),
        Some(&driver_b),
        Some(json!({ "method": "cash", "received": true }).to_string()),
    )
    .await;
    assert_eq!(decl_status, StatusCode::FORBIDDEN);
    assert_eq!(
        decl_body["error"]["code"],
        "UNAUTHORIZED_PAYMENT_DECLARATION"
    );
}

// T-17-049 happy path after confirm
#[tokio::test]
async fn given_own_sale_when_declare_payment_then_200() {
    let env = setup().await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let (driver_id, driver) = seed_driver(&env, "driver-decl@test.com").await;
    let product_id = seed_product(&env, "DECL-17E", "Decl Item", 1000).await;
    seed_driver_stock(&env, driver_id, product_id, 5).await;
    let sale_id = create_pending_sale(&env, &driver, commerce_id, product_id).await;
    let (confirm_status, _) = request(
        &env,
        "POST",
        &format!("/v1/sales/{sale_id}/confirm"),
        Some(&driver),
        None,
    )
    .await;
    assert_eq!(confirm_status, StatusCode::OK);

    let (status, body) = request(
        &env,
        "POST",
        &format!("/v1/sales/{sale_id}/declare-payment"),
        Some(&driver),
        Some(json!({ "method": "cash", "received": true }).to_string()),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["declaredPaymentReceived"], true);
}

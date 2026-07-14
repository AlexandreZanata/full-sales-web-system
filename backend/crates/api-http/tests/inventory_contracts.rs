//! Phase 17E — Inventory contracts (T-17-040..043).

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{request, seed_admin, seed_driver, seed_product, setup};

// T-17-040 / T-17-041 / T-17-042 / T-17-043
#[tokio::test]
async fn given_admin_when_inventory_flow_then_balances_and_movements() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let product_id = seed_product(&env, "INV-17E", "Inv Item", 500).await;

    let (bal_list_status, bal_list) = request(
        &env,
        "GET",
        "/v1/inventory/balances?limit=20",
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(bal_list_status, StatusCode::OK);
    assert!(bal_list["data"].is_array());

    let (mv_status, _mv) = request(
        &env,
        "POST",
        "/v1/inventory/movements",
        Some(&admin),
        Some(
            json!({
                "productId": product_id,
                "movementType": "Adjustment",
                "quantity": 7,
                "reason": "Cycle count"
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(mv_status, StatusCode::CREATED);

    let (bal_status, bal) = request(
        &env,
        "GET",
        &format!("/v1/inventory/products/{product_id}/balance"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(bal_status, StatusCode::OK);
    assert_eq!(bal["productId"], product_id.to_string());
    assert!(bal["available"].as_i64().unwrap() >= 7);

    let (hist_status, hist) = request(
        &env,
        "GET",
        &format!("/v1/inventory/products/{product_id}/movements?limit=20"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(hist_status, StatusCode::OK);
    assert!(!hist["data"].as_array().expect("data").is_empty());
}

// T-17-040 / T-17-042 authz
#[tokio::test]
async fn given_driver_or_no_token_when_admin_inventory_then_denied() {
    let env = setup().await;
    let product_id = seed_product(&env, "INV-AUTHZ", "P", 100).await;
    let (_, driver) = seed_driver(&env, "driver-inv@test.com").await;

    let (list_status, list_body) =
        request(&env, "GET", "/v1/inventory/balances", Some(&driver), None).await;
    assert_eq!(list_status, StatusCode::FORBIDDEN);
    assert_eq!(list_body["error"]["code"], "FORBIDDEN");

    let (mv_status, _) = request(
        &env,
        "POST",
        "/v1/inventory/movements",
        Some(&driver),
        Some(
            json!({
                "productId": product_id,
                "movementType": "Adjustment",
                "quantity": 1,
                "reason": "x"
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(mv_status, StatusCode::FORBIDDEN);

    let (unauth, unauth_body) = request(&env, "GET", "/v1/inventory/balances", None, None).await;
    assert_eq!(unauth, StatusCode::UNAUTHORIZED);
    assert_eq!(unauth_body["error"]["code"], "UNAUTHORIZED");
}

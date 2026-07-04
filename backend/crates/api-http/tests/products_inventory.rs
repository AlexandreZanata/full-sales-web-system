//! Phase 19 — Products & inventory contract tests.

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{request, seed_admin, setup};

// Contract: create product → appears in GET list
#[tokio::test]
async fn contract_create_product_when_listed_then_present() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (create_status, created) = request(
        &env,
        "POST",
        "/v1/products",
        Some(&admin_token),
        Some(
            json!({
                "name": "New Widget",
                "sku": "NEW-SKU-001",
                "priceAmount": 2500,
                "priceCurrency": "BRL"
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let product_id = created["id"].as_str().expect("id");

    let (list_status, list_body) = request(
        &env,
        "GET",
        "/v1/products?page=1&pageSize=50",
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(list_status, StatusCode::OK);
    assert!(list_body["items"]
        .as_array()
        .unwrap()
        .iter()
        .any(|p| p["id"].as_str() == Some(product_id)));
}

// Contract: adjustment increases balance
#[tokio::test]
async fn contract_adjustment_when_positive_then_balance_increases() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (create_status, created) = request(
        &env,
        "POST",
        "/v1/products",
        Some(&admin_token),
        Some(
            json!({
                "name": "Stock Widget",
                "sku": "STK-SKU",
                "priceAmount": 500
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let product_id = created["id"].as_str().expect("id");

    let (before_status, before_body) = request(
        &env,
        "GET",
        &format!("/v1/inventory/products/{product_id}/balance"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(before_status, StatusCode::OK);
    let before = before_body["available"].as_i64().unwrap();

    let (adj_status, _) = request(
        &env,
        "POST",
        "/v1/inventory/movements",
        Some(&admin_token),
        Some(
            json!({
                "productId": product_id,
                "movementType": "Adjustment",
                "quantity": 5,
                "reason": "Initial stock count"
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(adj_status, StatusCode::CREATED);

    let (after_status, after_body) = request(
        &env,
        "GET",
        &format!("/v1/inventory/products/{product_id}/balance"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(after_status, StatusCode::OK);
    assert_eq!(after_body["available"].as_i64().unwrap(), before + 5);
}

// Contract: negative adjustment beyond balance → 422
#[tokio::test]
async fn contract_adjustment_when_exceeds_balance_then_422() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (create_status, created) = request(
        &env,
        "POST",
        "/v1/products",
        Some(&admin_token),
        Some(
            json!({
                "name": "Low Stock",
                "sku": "LOW-SKU",
                "priceAmount": 100
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let product_id = created["id"].as_str().expect("id");

    let (adj_status, adj_body) = request(
        &env,
        "POST",
        "/v1/inventory/movements",
        Some(&admin_token),
        Some(
            json!({
                "productId": product_id,
                "movementType": "Adjustment",
                "quantity": -10,
                "reason": "Correction"
            })
            .to_string(),
        ),
    )
    .await;

    assert_eq!(adj_status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(adj_body["error"]["code"], "INSUFFICIENT_BALANCE");
}

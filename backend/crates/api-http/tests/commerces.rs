//! Phase 18 — Commerces & addresses contract tests.

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{request, seed_admin, seed_commerce, seed_driver, seed_product, setup};

// Contract: driver can GET commerce list; cannot POST address
#[tokio::test]
async fn contract_driver_when_list_commerces_then_ok_but_post_address_forbidden() {
    let env = setup().await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let (_, driver_token) = seed_driver(&env, "driver@test.com").await;

    let (list_status, list_body) = request(
        &env,
        "GET",
        "/v1/commerces",
        Some(&driver_token),
        None,
    )
    .await;
    assert_eq!(list_status, StatusCode::OK);
    assert!(list_body["items"].is_array());
    assert!(list_body["items"]
        .as_array()
        .unwrap()
        .iter()
        .any(|c| c["id"] == commerce_id.to_string()));

    let (post_status, post_body) = request(
        &env,
        "POST",
        &format!("/v1/commerces/{commerce_id}/addresses"),
        Some(&driver_token),
        Some(
            json!({
                "addressType": "Delivery",
                "street": "Rua Nova",
                "number": "100",
                "city": "SP",
                "state": "SP",
                "postalCode": "01310100",
                "isPrimary": false
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(post_status, StatusCode::FORBIDDEN);
    assert_eq!(post_body["error"]["code"], "FORBIDDEN");
}

// Contract: deactivate commerce → subsequent sale 422
#[tokio::test]
async fn contract_deactivate_commerce_when_new_sale_then_422() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let (driver_id, driver_token) = seed_driver(&env, "driver@test.com").await;
    let product_id = seed_product(&env, "SKU-001", "Widget", 1_000).await;

    infra_postgres::inventory::upsert_stock_balance(
        &env.app_pool,
        env.tenant_id,
        driver_id,
        product_id,
        10,
    )
    .await
    .expect("stock");

    let (deactivate_status, _) = request(
        &env,
        "PATCH",
        &format!("/v1/commerces/{commerce_id}/deactivate"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(deactivate_status, StatusCode::OK);

    let (sale_status, sale_body) = request(
        &env,
        "POST",
        "/v1/sales",
        Some(&driver_token),
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

    assert_eq!(sale_status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(sale_body["error"]["code"], "COMMERCE_INACTIVE");
}

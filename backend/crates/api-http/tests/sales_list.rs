//! Phase 20 — Sales list & declare-payment contract tests.

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{request, seed_commerce, seed_driver, seed_product, setup};

async fn create_sale(
    env: &support::TestEnv,
    driver_token: &str,
    commerce_id: uuid::Uuid,
    product_id: uuid::Uuid,
) -> String {
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
    body["id"].as_str().expect("sale id").to_owned()
}

// Contract: list returns pagination meta
#[tokio::test]
async fn contract_list_sales_when_driver_then_pagination_meta() {
    let env = setup().await;
    let (driver_id, driver_token) = seed_driver(&env, "driver@test.com").await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
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

    create_sale(&env, &driver_token, commerce_id, product_id).await;

    let (status, body) = request(&env, "GET", "/v1/sales", Some(&driver_token), None).await;

    assert_eq!(status, StatusCode::OK);
    assert!(body["items"].is_array());
    assert_eq!(body["page"], 1);
    assert_eq!(body["pageSize"], 20);
    assert!(body["total"].as_u64().is_some());
}

// Contract: driver B cannot declare driver A sale → 403
#[tokio::test]
async fn contract_declare_payment_when_other_driver_then_forbidden() {
    let env = setup().await;
    let (driver_a_id, driver_a_token) = seed_driver(&env, "driver-a@test.com").await;
    let (_, driver_b_token) = seed_driver(&env, "driver-b@test.com").await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let product_id = seed_product(&env, "SKU-002", "Gadget", 2_000).await;

    infra_postgres::inventory::upsert_stock_balance(
        &env.app_pool,
        env.tenant_id,
        driver_a_id,
        product_id,
        10,
    )
    .await
    .expect("stock");

    let sale_id = create_sale(&env, &driver_a_token, commerce_id, product_id).await;

    let (status, body) = request(
        &env,
        "POST",
        &format!("/v1/sales/{sale_id}/declare-payment"),
        Some(&driver_b_token),
        Some(json!({ "method": "cash", "received": true }).to_string()),
    )
    .await;

    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body["error"]["code"], "UNAUTHORIZED_PAYMENT_DECLARATION");
}

//! Phase 20 — Sales list & declare-payment contract tests.

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{request, seed_commerce, seed_driver, seed_product, setup};

async fn create_sale(
    env: &support::TestEnv,
    token: &str,
    commerce_id: uuid::Uuid,
    product_id: uuid::Uuid,
) -> String {
    let (status, body) = request(
        env,
        "POST",
        "/v1/sales",
        Some(token),
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

async fn seed_stock(
    env: &support::TestEnv,
    user_id: uuid::Uuid,
    product_id: uuid::Uuid,
    quantity: i32,
) {
    infra_postgres::inventory::upsert_stock_balance(
        &env.app_pool,
        env.tenant_id,
        user_id,
        product_id,
        quantity,
    )
    .await
    .expect("stock");
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

// Contract: seller lists own sale after create
#[tokio::test]
async fn contract_list_sales_when_seller_creates_then_returns_own_sale() {
    let env = setup().await;
    let (seller_id, seller_token) = (
        support::seed_user(&env, "seller@test.com", "secret123", "Seller", true).await,
        support::login_token(&env, "seller@test.com", "secret123").await,
    );
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let product_id = seed_product(&env, "SKU-SELLER", "Seller Widget", 1_500).await;
    seed_stock(&env, seller_id, product_id, 10).await;

    let sale_id = create_sale(&env, &seller_token, commerce_id, product_id).await;

    let (status, body) = request(&env, "GET", "/v1/sales", Some(&seller_token), None).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["total"], 1);
    assert_eq!(body["items"][0]["id"], sale_id);
    assert_eq!(body["items"][0]["driverId"], seller_id.to_string());
}

// Contract: seller list excludes another user's sale
#[tokio::test]
async fn contract_list_sales_when_seller_then_excludes_other_user_sale() {
    let env = setup().await;
    let (seller_id, seller_token) = (
        support::seed_user(&env, "seller-a@test.com", "secret123", "Seller", true).await,
        support::login_token(&env, "seller-a@test.com", "secret123").await,
    );
    let (driver_id, driver_token) = seed_driver(&env, "driver-b@test.com").await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let product_id = seed_product(&env, "SKU-ISO", "Isolated", 2_000).await;
    seed_stock(&env, seller_id, product_id, 10).await;
    seed_stock(&env, driver_id, product_id, 10).await;

    let seller_sale_id = create_sale(&env, &seller_token, commerce_id, product_id).await;
    create_sale(&env, &driver_token, commerce_id, product_id).await;

    let (status, body) = request(&env, "GET", "/v1/sales", Some(&seller_token), None).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["total"], 1);
    assert_eq!(body["items"][0]["id"], seller_sale_id);
}

// Contract: seller driverId query is ignored (Admin-only filter)
#[tokio::test]
async fn contract_list_sales_when_seller_driver_id_query_then_ignored() {
    let env = setup().await;
    let (seller_id, seller_token) = (
        support::seed_user(&env, "seller-filter@test.com", "secret123", "Seller", true).await,
        support::login_token(&env, "seller-filter@test.com", "secret123").await,
    );
    let (driver_id, driver_token) = seed_driver(&env, "driver-filter@test.com").await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let product_id = seed_product(&env, "SKU-FILTER", "Filter", 2_500).await;
    seed_stock(&env, seller_id, product_id, 10).await;
    seed_stock(&env, driver_id, product_id, 10).await;

    let seller_sale_id = create_sale(&env, &seller_token, commerce_id, product_id).await;
    create_sale(&env, &driver_token, commerce_id, product_id).await;

    let path = format!("/v1/sales?driverId={driver_id}");
    let (status, body) = request(&env, "GET", &path, Some(&seller_token), None).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["total"], 1);
    assert_eq!(body["items"][0]["id"], seller_sale_id);
    assert_eq!(body["items"][0]["driverId"], seller_id.to_string());
}

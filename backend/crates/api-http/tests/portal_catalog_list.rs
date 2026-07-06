//! Phase 68E — Portal/public catalog cursor list contract tests.

#[path = "support/mod.rs"]
mod support;

use domain_shared::TenantId;
use http::StatusCode;

use support::{request, seed_admin, seed_product, setup_with_tenant};

const DEV_SEED_TENANT_ID: &str = "01900001-0000-7000-8000-000000000001";

#[tokio::test]
async fn contract_public_products_when_cursor_envelope_then_data_array() {
    let tenant_id = TenantId::parse(DEV_SEED_TENANT_ID).expect("dev seed tenant");
    let env = setup_with_tenant(tenant_id).await;
    let _product_id = seed_product(&env, "PUB-SKU", "Public Widget", 990).await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/public/products?limit=20",
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["data"].is_array());
    assert!(!body["data"].as_array().expect("data").is_empty());
    assert_eq!(body["pagination"]["limit"], 20);
}

#[tokio::test]
async fn contract_public_products_when_invalid_filter_then_400() {
    let tenant_id = TenantId::parse(DEV_SEED_TENANT_ID).expect("dev seed tenant");
    let env = setup_with_tenant(tenant_id).await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/public/products?filter[unknown]=x",
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "invalid_filter_field");
}

#[tokio::test]
async fn contract_public_categories_when_cursor_envelope_then_data_array() {
    let tenant_id = TenantId::parse(DEV_SEED_TENANT_ID).expect("dev seed tenant");
    let env = setup_with_tenant(tenant_id).await;
    let (_, _admin_token) = support::seed_admin(&env).await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/public/categories?limit=20",
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["data"].is_array());
    assert_eq!(body["pagination"]["has_more"], false);
}

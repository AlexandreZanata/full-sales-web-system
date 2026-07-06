//! Public catalog — browse without login (portal guest mode).

#[path = "support/mod.rs"]
mod support;

use domain_shared::TenantId;
use http::StatusCode;

use support::{request, seed_product, setup_with_tenant};

const DEV_SEED_TENANT_ID: &str = "01900001-0000-7000-8000-000000000001";

#[tokio::test]
async fn given_no_auth_when_get_public_products_then_200_with_data() {
    let tenant_id = TenantId::parse(DEV_SEED_TENANT_ID).expect("dev seed tenant");
    let env = setup_with_tenant(tenant_id).await;
    let _product_id = seed_product(&env, "PUB-SKU", "Public Widget", 990).await;

    let (status, body) = request(&env, "GET", "/v1/public/products?limit=20", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(!body["data"].as_array().expect("data").is_empty());
    assert_eq!(body["pagination"]["limit"], 20);
}

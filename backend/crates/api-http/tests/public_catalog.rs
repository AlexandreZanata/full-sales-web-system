//! Public catalog — browse without login (portal guest mode).

#[path = "support/mod.rs"]
mod support;

use domain_shared::TenantId;
use http::StatusCode;

use support::{request, seed_portal_home_content, seed_product, setup_with_tenant};

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

#[tokio::test]
async fn given_seeded_portal_content_when_public_home_endpoints_then_200_with_data() {
    let tenant_id = TenantId::parse(DEV_SEED_TENANT_ID).expect("dev seed tenant");
    let env = setup_with_tenant(tenant_id).await;
    let seed = seed_portal_home_content(&env).await;

    let (banner_status, banner_body) =
        request(&env, "GET", "/v1/public/banners?placement=hero&limit=5", None, None).await;
    assert_eq!(banner_status, StatusCode::OK);
    let banners = banner_body["data"].as_array().expect("banner data");
    assert!(!banners.is_empty());
    assert!(banners[0]["imageUrl"].as_str().is_some());

    let (promo_status, promo_body) =
        request(&env, "GET", "/v1/public/promotions?limit=4", None, None).await;
    assert_eq!(promo_status, StatusCode::OK);
    let promotions = promo_body["data"].as_array().expect("promotion data");
    assert!(!promotions.is_empty());
    assert_eq!(promotions[0]["headline"], "Tasty Burger");

    let (featured_status, featured_body) =
        request(&env, "GET", "/v1/public/products/featured?limit=8", None, None).await;
    assert_eq!(featured_status, StatusCode::OK);
    let featured = featured_body["data"].as_array().expect("featured data");
    assert!(!featured.is_empty());
    assert!(
        featured
            .iter()
            .any(|row| row["id"] == seed.featured_product_id.to_string())
    );

    let (popular_status, popular_body) =
        request(&env, "GET", "/v1/public/products/popular?limit=8", None, None).await;
    assert_eq!(popular_status, StatusCode::OK);
    let popular = popular_body["data"].as_array().expect("popular data");
    assert!(!popular.is_empty());
    assert_eq!(popular[0]["id"], seed.popular_product_id.to_string());
}

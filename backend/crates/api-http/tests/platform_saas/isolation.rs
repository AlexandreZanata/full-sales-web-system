//! Phase 13B — cross-tenant RLS regression (TS-E2E-004 extended).

use http::StatusCode;

use crate::isolation_seed::{seed_other_tenant_commerce, seed_other_tenant_product};
use crate::support::{
    platform_access_token, request, seed_admin, seed_commerce, seed_product, setup,
};

#[tokio::test]
async fn contract_tenant_a_admin_cannot_read_tenant_b_commerce() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;
    let (_, other_commerce) = seed_other_tenant_commerce(&env).await;
    let own_commerce = seed_commerce(&env, "11222333000181").await;

    let (status, _) = request(
        &env,
        "GET",
        &format!("/v1/commerces/{own_commerce}"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let (status, body) = request(
        &env,
        "GET",
        &format!("/v1/commerces/{other_commerce}"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "COMMERCE_NOT_FOUND");
}

#[tokio::test]
async fn contract_tenant_a_admin_cannot_read_tenant_b_product() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;
    let other_product = seed_other_tenant_product(&env).await;
    let own_product = seed_product(&env, "OWN-SKU", "Own Product", 500).await;

    let (status, _) = request(
        &env,
        "GET",
        &format!("/v1/products/{own_product}"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let (status, body) = request(
        &env,
        "GET",
        &format!("/v1/products/{other_product}"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "PRODUCT_NOT_FOUND");
}

#[tokio::test]
async fn contract_platform_admin_can_list_both_tenants() {
    let env = setup().await;
    let _ = seed_admin(&env).await;
    let (other_tenant, _) = seed_other_tenant_commerce(&env).await;
    let token = platform_access_token(&env).await;

    let (status, body) = request(&env, "GET", "/v1/platform/tenants", Some(&token), None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let ids: Vec<String> = body["data"]
        .as_array()
        .expect("data")
        .iter()
        .filter_map(|row| row["id"].as_str().map(str::to_owned))
        .collect();
    assert!(ids.contains(&env.tenant_id.as_uuid().to_string()));
    assert!(ids.contains(&other_tenant.as_uuid().to_string()));
}

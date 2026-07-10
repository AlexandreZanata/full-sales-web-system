//! Phase 2 — tenant lifecycle and suspension gate contract tests.

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{
    platform_access_token, request, seed_admin, seed_commerce, seed_driver, seed_driver_stock,
    seed_platform_admin, seed_product, setup,
};

#[tokio::test]
async fn contract_provision_tenant_when_valid_then_created_with_admin() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let token = platform_access_token(&env).await;

    let body = json!({
        "legalName": "Beta Distribuidora LTDA",
        "displayName": "Beta Store",
        "adminEmail": "beta-admin@test.com",
        "planId": "01900002-0001-7000-8000-000000000001",
        "trial": true
    })
    .to_string();

    let (status, resp) = request(
        &env,
        "POST",
        "/v1/platform/tenants",
        Some(&token),
        Some(body),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED, "provision: {resp}");
    assert_eq!(resp["status"], "Trial");
    assert!(resp["adminTemporaryPassword"].is_string());
    assert!(resp["trialEndsAt"].is_string());
}

#[tokio::test]
async fn contract_suspend_tenant_when_active_then_blocks_sale_creation() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let product_id = seed_product(&env, "SUS-1", "Suspend Product", 500).await;
    let (_, admin_token) = seed_admin(&env).await;
    let platform_token = platform_access_token(&env).await;

    let suspend_body = json!({ "reason": "billing fraud review" }).to_string();
    let (status, resp) = request(
        &env,
        "POST",
        &format!("/v1/platform/tenants/{}/suspend", env.tenant_id.as_uuid()),
        Some(&platform_token),
        Some(suspend_body),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "suspend failed: {resp}");

    let sale_body = json!({
        "commerceId": commerce_id,
        "items": [{ "productId": product_id, "quantity": 1 }],
        "paymentMethod": "cash"
    })
    .to_string();
    let (status, resp) = request(
        &env,
        "POST",
        "/v1/sales",
        Some(&admin_token),
        Some(sale_body),
    )
    .await;

    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(resp["error"]["code"], "TENANT_SUSPENDED");
}

#[tokio::test]
async fn contract_reactivate_tenant_when_suspended_then_sale_allowed() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let product_id = seed_product(&env, "REA-1", "Reactivate Product", 700).await;
    let (driver_id, _) = seed_driver(&env, "driver-reactivate@test.com").await;
    seed_driver_stock(&env, driver_id, product_id, 10).await;
    let (_, admin_token) = seed_admin(&env).await;
    let platform_token = platform_access_token(&env).await;
    let tenant_path = format!("/v1/platform/tenants/{}", env.tenant_id.as_uuid());

    let _ = request(
        &env,
        "POST",
        &format!("{tenant_path}/suspend"),
        Some(&platform_token),
        Some(json!({ "reason": "manual hold" }).to_string()),
    )
    .await;

    let (status, _) = request(
        &env,
        "POST",
        &format!("{tenant_path}/reactivate"),
        Some(&platform_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let sale_body = json!({
        "commerceId": commerce_id,
        "driverId": driver_id,
        "items": [{ "productId": product_id, "quantity": 1 }],
        "paymentMethod": "pix"
    })
    .to_string();
    let (status, resp) = request(
        &env,
        "POST",
        "/v1/sales",
        Some(&admin_token),
        Some(sale_body),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "sale after reactivate: {resp}");
}

#[tokio::test]
async fn contract_list_platform_tenants_includes_seeded_tenant() {
    let env = setup().await;
    let token = platform_access_token(&env).await;
    let (status, resp) = request(&env, "GET", "/v1/platform/tenants", Some(&token), None).await;
    assert_eq!(status, StatusCode::OK);
    let ids: Vec<String> = resp["data"]
        .as_array()
        .expect("data")
        .iter()
        .map(|row| row["id"].as_str().unwrap().to_owned())
        .collect();
    assert!(ids.contains(&env.tenant_id.as_uuid().to_string()));
}

//! Phase 8 — platform operations API contract tests.

#[path = "support/mod.rs"]
mod support;

use chrono::{Duration, Utc};
use http::StatusCode;
use serde_json::json;

use support::{
    platform_access_token, request, seed_admin, seed_commerce, seed_driver, seed_order,
    seed_platform_admin, setup,
};

#[tokio::test]
async fn contract_platform_users_when_filter_tenant_then_scoped() {
    let env = setup().await;
    let _ = seed_admin(&env).await;
    let _ = seed_driver(&env, "driver@test.com").await;

    let other_tenant = domain_shared::TenantId::generate();
    infra_postgres::shared::insert_tenant(&env.admin_pool, other_tenant, "Other")
        .await
        .expect("tenant");
    infra_postgres::identity::insert_user(
        &env.app_pool,
        other_tenant,
        infra_postgres::identity::InsertUserParams {
            id: uuid::Uuid::now_v7(),
            email: "other@test.com",
            name: "Other User",
            role: "Admin",
            password_hash: "$2a$12$aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            commerce_id: None,
            profile_file_id: None,
        },
    )
    .await
    .expect("user");

    let token = platform_access_token(&env).await;
    let path = format!(
        "/v1/platform/users?filter[tenant_id]={}",
        env.tenant_id.as_uuid()
    );
    let (status, body) = request(&env, "GET", &path, Some(&token), None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let emails: Vec<&str> = body["data"]
        .as_array()
        .expect("data")
        .iter()
        .filter_map(|row| row["email"].as_str())
        .collect();
    assert!(emails.contains(&"driver@test.com"));
    assert!(!emails.contains(&"other@test.com"));
}

#[tokio::test]
async fn contract_platform_disable_user_when_driver_then_inactive() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let (driver_id, _) = seed_driver(&env, "disable-me@test.com").await;
    let token = platform_access_token(&env).await;

    let (status, body) = request(
        &env,
        "POST",
        &format!("/v1/platform/users/{driver_id}/disable"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["active"], false);

    let (status, detail) = request(
        &env,
        "GET",
        &format!("/v1/platform/users/{driver_id}"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{detail}");
    assert_eq!(detail["active"], false);
}

#[tokio::test]
async fn contract_platform_disable_when_last_admin_then_rejected() {
    let env = setup().await;
    let (admin_id, _) = seed_admin(&env).await;
    let token = platform_access_token(&env).await;

    let (status, body) = request(
        &env,
        "POST",
        &format!("/v1/platform/users/{admin_id}/disable"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    assert_eq!(body["error"]["code"], "LAST_ADMIN_REQUIRED");
}

#[tokio::test]
async fn contract_platform_support_when_orders_exist_then_lists() {
    let env = setup().await;
    let (admin_id, _) = seed_admin(&env).await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let order_id = seed_order(&env, commerce_id, admin_id).await;
    let token = platform_access_token(&env).await;

    let path = format!("/v1/platform/tenants/{}/orders", env.tenant_id.as_uuid());
    let (status, body) = request(&env, "GET", &path, Some(&token), None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let ids: Vec<String> = body["data"]
        .as_array()
        .expect("data")
        .iter()
        .filter_map(|row| row["id"].as_str().map(str::to_owned))
        .collect();
    assert!(ids.iter().any(|id| id == &order_id.to_string()), "{body}");
}

#[tokio::test]
async fn contract_maintenance_when_global_active_then_public_503() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let token = platform_access_token(&env).await;
    let now = Utc::now();
    let schedule = json!({
        "message": "Scheduled maintenance",
        "startsAt": (now - Duration::minutes(1)).to_rfc3339(),
        "endsAt": (now + Duration::hours(1)).to_rfc3339(),
    })
    .to_string();
    let (status, _) = request(
        &env,
        "POST",
        "/v1/platform/maintenance",
        Some(&token),
        Some(schedule),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);

    let (status, body) = request(&env, "GET", "/v1/public/products", None, None).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE, "{body}");
    assert_eq!(body["error"]["code"], "MAINTENANCE");
}

#[tokio::test]
async fn contract_maintenance_when_tenant_active_then_settings_banner() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let (_, admin_token) = seed_admin(&env).await;
    let platform_token = platform_access_token(&env).await;
    let now = Utc::now();
    let schedule = json!({
        "tenantId": env.tenant_id.as_uuid(),
        "message": "Tenant maintenance window",
        "startsAt": (now - Duration::minutes(1)).to_rfc3339(),
        "endsAt": (now + Duration::hours(1)).to_rfc3339(),
    })
    .to_string();
    let (status, _) = request(
        &env,
        "POST",
        "/v1/platform/maintenance",
        Some(&platform_token),
        Some(schedule),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);

    let (status, settings) = request(
        &env,
        "GET",
        "/v1/settings",
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{settings}");
    assert_eq!(settings["maintenanceBanner"]["message"], "Tenant maintenance window");
}

#[tokio::test]
async fn contract_feature_flags_when_patch_then_resolved() {
    let env = setup().await;
    let token = platform_access_token(&env).await;
    let path = format!("/v1/platform/tenants/{}/features", env.tenant_id.as_uuid());
    let body = json!({ "onlinePayments": false, "apiRateTier": "enterprise" }).to_string();
    let (status, resp) = request(&env, "PATCH", &path, Some(&token), Some(body)).await;
    assert_eq!(status, StatusCode::OK, "{resp}");
    assert_eq!(resp["onlinePayments"], false);
    assert_eq!(resp["apiRateTier"], "enterprise");
}

//! Phase 6 — anti-fraud and abuse contract tests (BR-FR-001, BR-FR-002).

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{platform_access_token, request, seed_admin, seed_user, setup};

const PRO_PLAN: &str = "01900002-0001-7000-8000-000000000002";

async fn set_fraud_threshold(env: &support::TestEnv, patch: serde_json::Value) {
    sqlx::query("UPDATE fraud.platform_settings SET thresholds = thresholds || $1::jsonb WHERE id = 1")
        .bind(patch)
        .execute(&env.admin_pool)
        .await
        .expect("thresholds");
}

#[tokio::test]
async fn contract_login_velocity_when_threshold_exceeded_then_fraud_blocked_and_event() {
    let env = setup().await;
    seed_user(&env, "user@test.com", "secret123", "Admin", true).await;
    set_fraud_threshold(&env, json!({ "loginFailureMax": 1 })).await;

    let login_body = json!({ "email": "user@test.com", "password": "wrong" }).to_string();
    let (status, _) = request(
        &env,
        "POST",
        "/v1/auth/login",
        None,
        Some(login_body.clone()),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let (status, resp) = request(
        &env,
        "POST",
        "/v1/auth/login",
        None,
        Some(login_body),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{resp}");
    assert_eq!(resp["error"]["code"], "FRAUD_BLOCKED");

    let token = platform_access_token(&env).await;
    let (status, events) = request(
        &env,
        "GET",
        "/v1/platform/fraud/events?filter[status]=Open",
        Some(&token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{events}");
    let items = events["data"].as_array().expect("data");
    assert!(
        items.iter().any(|e| e["eventType"] == "LoginVelocity"),
        "expected LoginVelocity event: {events}"
    );
}

#[tokio::test]
async fn contract_blocklist_when_email_blocked_then_provision_rejected() {
    let env = setup().await;
    let token = platform_access_token(&env).await;

    let block_body = json!({
        "email": "blocked-provision@test.com",
        "reason": "Confirmed fraud"
    })
    .to_string();
    let (status, _) = request(
        &env,
        "POST",
        "/v1/platform/blocklist",
        Some(&token),
        Some(block_body),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);

    let tenant_body = json!({
        "legalName": "Blocked Co",
        "displayName": "Blocked",
        "adminEmail": "blocked-provision@test.com",
        "planId": PRO_PLAN
    })
    .to_string();
    let (status, resp) = request(
        &env,
        "POST",
        "/v1/platform/tenants",
        Some(&token),
        Some(tenant_body),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{resp}");
    assert_eq!(resp["error"]["code"], "FRAUD_BLOCKED");
}

#[tokio::test]
async fn contract_payment_velocity_when_threshold_exceeded_then_fraud_blocked() {
    let env = setup().await;
    set_fraud_threshold(&env, json!({ "paymentVelocityMax": 0 })).await;

    for _ in 0..2 {
        let key = format!("fraud:velocity:payment:{}", env.tenant_id.as_uuid());
        let _ = env
            .state
            .velocity_counter
            .increment(&key, std::time::Duration::from_secs(3600))
            .await
            .expect("inc");
    }

    let (_, _admin_token) = seed_admin(&env).await;
    infra_postgres::shared::update_tenant_lifecycle(
        &env.admin_pool,
        env.tenant_id,
        domain_platform::TenantStatus::Active,
        Some(uuid::Uuid::parse_str(PRO_PLAN).expect("plan")),
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await
    .expect("plan");

    infra_postgres::billing::upsert_payment_settings(
        &env.admin_pool,
        env.tenant_id,
        true,
        domain_billing::PaymentMethodToggles::all_enabled(),
        true,
    )
    .await
    .expect("payment settings");

    let commerce_id = support::seed_commerce(&env, "11222333000181").await;
    let address_id = support::seed_delivery_address(&env, commerce_id).await;
    let product_id = support::seed_product(&env, "FRAUD-SKU", "Fraud Product", 2_000).await;
    let (_contact_id, contact_token) =
        support::seed_commerce_contact(&env, commerce_id, "buyer@test.com").await;

    let order_body = json!({
        "deliveryAddressId": address_id,
        "items": [{ "productId": product_id, "quantity": 1 }]
    })
    .to_string();
    let (status, created) = request(
        &env,
        "POST",
        "/v1/portal/orders",
        Some(&contact_token),
        Some(order_body),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{created}");
    let order_id = created["id"].as_str().expect("order id");

    let (status, resp) = request(
        &env,
        "POST",
        &format!("/v1/portal/orders/{order_id}/submit"),
        Some(&contact_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{resp}");
    assert_eq!(resp["error"]["code"], "FRAUD_BLOCKED");
}

#[tokio::test]
async fn contract_tenant_admin_when_fraud_alerts_then_lists_tenant_events() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;
    let event_id = uuid::Uuid::now_v7();
    sqlx::query(
        "INSERT INTO fraud.fraud_events (id, tenant_id, event_type, severity, metadata)
         VALUES ($1, $2, 'PaymentVelocity', 'High', '{}'::jsonb)",
    )
    .bind(event_id)
    .bind(env.tenant_id.as_uuid())
    .execute(&env.admin_pool)
    .await
    .expect("seed fraud event");

    let (status, alerts) = request(
        &env,
        "GET",
        "/v1/fraud/alerts",
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{alerts}");
    let items = alerts.as_array().expect("alerts array");
    assert!(
        items.iter().any(|e| e["eventType"] == "PaymentVelocity"),
        "expected tenant fraud alerts: {alerts}"
    );
}

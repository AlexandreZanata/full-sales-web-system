//! Phase 4 — subscription billing contract tests (exit gate + BR-BI-002/003).

#[path = "support/mod.rs"]
mod support;

use chrono::{Duration, Utc};
use domain_billing::SubscriptionStatus;
use domain_platform::TenantStatus;
use http::StatusCode;
use serde_json::json;

use support::{platform_access_token, request, seed_admin, seed_platform_admin, setup};

const WEBHOOK_TOKEN: &str = "test-webhook-token-phase3";
const STARTER_PLAN: &str = "01900002-0001-7000-8000-000000000001";

async fn webhook_post(env: &support::TestEnv, body: String) -> (StatusCode, serde_json::Value) {
    let app = api_http::full_app(env.state.clone());
    let request = axum::http::Request::builder()
        .method("POST")
        .uri("/v1/billing/webhooks/asaas")
        .header("content-type", "application/json")
        .header("asaas-access-token", WEBHOOK_TOKEN)
        .body(axum::body::Body::from(body))
        .expect("request");
    let response = tower::ServiceExt::oneshot(app, request)
        .await
        .expect("response");
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap_or(json!({}));
    (status, json)
}

#[tokio::test]
async fn contract_payment_confirmed_when_trial_tenant_then_active() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let token = platform_access_token(&env).await;
    let body = json!({
        "legalName": "Trial Co",
        "displayName": "Trial",
        "adminEmail": "trial-billing@test.com",
        "planId": STARTER_PLAN,
        "trial": true,
        "cnpj": "11222333000181"
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
    assert_eq!(status, StatusCode::CREATED, "{resp}");
    let tenant_id = resp["tenantId"].as_str().expect("id");
    assert_eq!(resp["status"], "Trial");

    let webhook_body = json!({
        "id": "evt_pay_confirm_1",
        "event": "PAYMENT_CONFIRMED",
        "payment": {
            "id": "pay_cycle_1",
            "customer": "cus_mock",
            "status": "CONFIRMED",
            "value": 99.0,
            "externalReference": tenant_id
        }
    })
    .to_string();
    let (status, _) = webhook_post(&env, webhook_body).await;
    assert_eq!(status, StatusCode::OK);

    let tenant_uuid = uuid::Uuid::parse_str(tenant_id).expect("uuid");
    let tenant = infra_postgres::shared::find_tenant_lifecycle(
        &env.admin_pool,
        domain_shared::TenantId::from_uuid(tenant_uuid),
    )
    .await
    .expect("tenant")
    .expect("row");
    assert_eq!(tenant.status, TenantStatus::Active);

    let sub = infra_postgres::billing::find_subscription_by_tenant(
        &env.admin_pool,
        domain_shared::TenantId::from_uuid(tenant_uuid),
    )
    .await
    .expect("sub")
    .expect("subscription");
    assert_eq!(sub.status, SubscriptionStatus::Active);
}

#[tokio::test]
async fn contract_payment_overdue_then_dunning_suspends_after_grace() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let platform_token = platform_access_token(&env).await;

    let body = json!({
        "legalName": "Past Due Co",
        "displayName": "PastDue",
        "adminEmail": "pastdue@test.com",
        "planId": STARTER_PLAN,
        "trial": false,
        "cnpj": "11222333000181"
    })
    .to_string();
    let (status, resp) = request(
        &env,
        "POST",
        "/v1/platform/tenants",
        Some(&platform_token),
        Some(body),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{resp}");
    let provisioned_id = resp["tenantId"].as_str().expect("id");

    let overdue_body = json!({
        "id": "evt_overdue_1",
        "event": "PAYMENT_OVERDUE",
        "payment": {
            "id": "pay_overdue_1",
            "externalReference": provisioned_id
        }
    })
    .to_string();
    let (status, _) = webhook_post(&env, overdue_body).await;
    assert_eq!(status, StatusCode::OK);

    let tenant_uuid = uuid::Uuid::parse_str(provisioned_id).expect("uuid");
    infra_postgres::shared::backdate_past_due_at(
        &env.admin_pool,
        domain_shared::TenantId::from_uuid(tenant_uuid),
        Utc::now() - Duration::days(8),
    )
    .await
    .expect("backdate past due");

    let (status, dunning) = request(
        &env,
        "POST",
        "/v1/platform/jobs/dunning",
        Some(&platform_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{dunning}");
    let processed = dunning["processed"].as_array().expect("processed");
    assert!(
        processed
            .iter()
            .any(|id| id.as_str() == Some(provisioned_id))
    );

    let tenant = infra_postgres::shared::find_tenant_lifecycle(
        &env.admin_pool,
        domain_shared::TenantId::from_uuid(tenant_uuid),
    )
    .await
    .expect("tenant")
    .expect("row");
    assert_eq!(tenant.status, TenantStatus::Suspended);
}

#[tokio::test]
async fn contract_tenant_admin_gets_subscription() {
    let env = setup().await;
    let (admin_id, admin_token) = seed_admin(&env).await;
    let _ = admin_id;

    infra_postgres::shared::update_tenant_lifecycle(
        &env.admin_pool,
        env.tenant_id,
        TenantStatus::Active,
        Some(uuid::Uuid::parse_str(STARTER_PLAN).expect("plan")),
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await
    .expect("lifecycle");

    infra_postgres::billing::insert_subscription(
        &env.admin_pool,
        infra_postgres::billing::SubscriptionInsert {
            id: uuid::Uuid::now_v7(),
            tenant_id: env.tenant_id,
            plan_id: uuid::Uuid::parse_str(STARTER_PLAN).expect("plan"),
            asaas_subscription_id: Some("sub_mock_test".into()),
            status: SubscriptionStatus::Active,
            current_period_end: Some(Utc::now() + Duration::days(30)),
        },
    )
    .await
    .expect("subscription");

    let (status, resp) = request(
        &env,
        "GET",
        "/v1/billing/subscription",
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{resp}");
    assert_eq!(resp["plan"]["code"], "Starter");
    assert_eq!(resp["status"], "Active");
}

#[tokio::test]
async fn contract_plan_upgrade_via_platform_patch() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let token = platform_access_token(&env).await;
    let pro_plan = "01900002-0001-7000-8000-000000000002";

    let create_body = json!({
        "legalName": "Upgrade Co",
        "displayName": "Upgrade",
        "adminEmail": "upgrade@test.com",
        "planId": STARTER_PLAN,
        "trial": false,
        "cnpj": "11222333000181"
    })
    .to_string();
    let (status, resp) = request(
        &env,
        "POST",
        "/v1/platform/tenants",
        Some(&token),
        Some(create_body),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{resp}");
    let tenant_id = resp["tenantId"].as_str().expect("id");

    let patch_body = json!({ "planId": pro_plan }).to_string();
    let (status, patched) = request(
        &env,
        "PATCH",
        &format!("/v1/platform/tenants/{tenant_id}"),
        Some(&token),
        Some(patch_body),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{patched}");
    assert_eq!(patched["planId"], pro_plan);
}

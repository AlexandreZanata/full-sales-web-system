//! Phase 17J — Tenant billing HTTP (T-17-144..150).
#[path = "support/mod.rs"]
mod support;

use chrono::{Duration, Utc};
use domain_billing::SubscriptionStatus;
use domain_platform::TenantStatus;
use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use support::{request, seed_admin, seed_driver, setup};

const STARTER_PLAN: &str = "01900002-0001-7000-8000-000000000001";

async fn seed_active_subscription(env: &support::TestEnv) -> Uuid {
    let plan_id = Uuid::parse_str(STARTER_PLAN).expect("plan");
    infra_postgres::shared::update_tenant_lifecycle(
        &env.admin_pool,
        env.tenant_id,
        TenantStatus::Active,
        Some(plan_id),
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await
    .expect("plan");
    let sub_id = Uuid::now_v7();
    infra_postgres::billing::insert_subscription(
        &env.admin_pool,
        infra_postgres::billing::SubscriptionInsert {
            id: sub_id,
            tenant_id: env.tenant_id,
            plan_id,
            asaas_subscription_id: Some("sub_mock_17j".into()),
            status: SubscriptionStatus::Active,
            current_period_end: Some(Utc::now() + Duration::days(30)),
        },
    )
    .await
    .expect("sub");
    sub_id
}

// T-17-145 / T-17-146 / T-17-150
#[tokio::test]
async fn given_admin_when_subscription_invoices_cancel_then_ok() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let _ = seed_active_subscription(&env).await;

    let (sub_st, sub) = request(&env, "GET", "/v1/billing/subscription", Some(&admin), None).await;
    assert_eq!(sub_st, StatusCode::OK);
    assert_eq!(sub["status"], "Active");

    let (inv_st, inv) = request(
        &env,
        "GET",
        "/v1/billing/invoices?limit=10",
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(inv_st, StatusCode::OK);
    assert!(inv["data"].is_array());

    let (cx_st, cx) = request(
        &env,
        "POST",
        "/v1/billing/subscription/cancel",
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(cx_st, StatusCode::ACCEPTED, "{cx}");
    assert_eq!(cx["status"], "Cancelled");
    assert_eq!(cx["cancelAtPeriodEnd"], true);
}

// T-17-147 / T-17-148 / T-17-144 authz + errors
#[tokio::test]
async fn given_billing_when_driver_or_missing_invoice_then_errors() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let (_, driver) = seed_driver(&env, "bill-drv@test.com").await;
    let _ = seed_active_subscription(&env).await;

    let (drv_st, drv) = request(&env, "GET", "/v1/billing/subscription", Some(&driver), None).await;
    assert_eq!(drv_st, StatusCode::FORBIDDEN);
    assert_eq!(drv["error"]["code"], "FORBIDDEN");

    let missing = Uuid::now_v7();
    let (nf_st, _) = request(
        &env,
        "GET",
        &format!("/v1/billing/invoices/{missing}"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(nf_st, StatusCode::NOT_FOUND);

    let (pm_st, pm) = request(
        &env,
        "POST",
        "/v1/billing/payment-methods",
        Some(&admin),
        Some(json!({ "type": "credit_card", "creditCardToken": "" }).to_string()),
    )
    .await;
    assert_eq!(pm_st, StatusCode::BAD_REQUEST);
    assert_eq!(pm["error"]["code"], "INVALID_PAYMENT_METHOD");

    let (wh_st, wh) = request(
        &env,
        "POST",
        "/v1/billing/webhooks/asaas",
        None,
        Some(json!({ "id": "evt_x", "event": "PAYMENT_CREATED" }).to_string()),
    )
    .await;
    assert_eq!(wh_st, StatusCode::UNAUTHORIZED);
    assert_eq!(wh["error"]["code"], "WEBHOOK_UNAUTHORIZED");
}

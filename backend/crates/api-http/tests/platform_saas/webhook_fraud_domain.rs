//! Phase 13A — webhook idempotency, fraud flag, domain verify (mock DNS).

use std::sync::Arc;

use api_http::MockDnsTxtResolver;
use domain_domains::txt_record_name;
use http::StatusCode;
use serde_json::json;

use crate::helpers::{PRO_PLAN, post_asaas_webhook};
use crate::support::{platform_access_token, request, seed_admin, seed_user, setup};

const HOSTNAME: &str = "shop.phase13.example.com";

#[tokio::test]
async fn contract_webhook_duplicate_event_is_idempotent() {
    let env = setup().await;
    let body = json!({
        "id": "evt_phase13_dup",
        "event": "PAYMENT_CONFIRMED",
        "payment": {
            "id": "pay_dup",
            "externalReference": env.tenant_id.as_uuid().to_string()
        }
    })
    .to_string();

    let (status, _) = post_asaas_webhook(&env, body.clone()).await;
    assert_eq!(status, StatusCode::OK);
    let (status, resp) = post_asaas_webhook(&env, body).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(resp["duplicate"], true);

    let count = infra_postgres::billing::count_payment_events(&env.admin_pool)
        .await
        .expect("count");
    assert_eq!(count, 1);
}

#[tokio::test]
async fn contract_fraud_velocity_when_exceeded_then_blocked_and_platform_event() {
    let env = setup().await;
    sqlx::query("UPDATE fraud.platform_settings SET thresholds = thresholds || '{\"loginFailureMax\": 1}'::jsonb WHERE id = 1")
        .execute(&env.admin_pool)
        .await
        .expect("thresholds");
    seed_user(&env, "fraud-user@test.com", "secret123", "Admin", true).await;

    let login_body = json!({ "email": "fraud-user@test.com", "password": "wrong" }).to_string();
    let _ = request(
        &env,
        "POST",
        "/v1/auth/login",
        None,
        Some(login_body.clone()),
    )
    .await;
    let (status, resp) = request(&env, "POST", "/v1/auth/login", None, Some(login_body)).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
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
    assert!(
        events["data"]
            .as_array()
            .expect("data")
            .iter()
            .any(|e| e["eventType"] == "LoginVelocity")
    );
}

#[tokio::test]
async fn contract_domain_verify_with_mock_dns_then_public_settings_resolve() {
    let mut env = setup().await;
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
    let (_, token) = seed_admin(&env).await;

    let body = json!({ "hostname": HOSTNAME }).to_string();
    let (status, created) = request(
        &env,
        "POST",
        "/v1/settings/domains",
        Some(&token),
        Some(body),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{created}");
    let txt_value = created["txtValue"].as_str().expect("txt");
    let domain_id = created["id"].as_str().expect("id");

    let mock_dns = Arc::new(MockDnsTxtResolver::new());
    mock_dns.set_txt(&txt_record_name(HOSTNAME), vec![txt_value.to_string()]);
    env.state.dns_resolver = mock_dns;

    let platform_token = platform_access_token(&env).await;
    let (status, job) = request(
        &env,
        "POST",
        "/v1/platform/jobs/domain-verification",
        Some(&platform_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{job}");
    assert!(
        job["verified"]
            .as_array()
            .expect("verified")
            .iter()
            .any(|id| id.as_str() == Some(domain_id))
    );

    let (status, settings) = crate::support::request_with_headers(
        &env,
        "GET",
        "/v1/public/settings",
        None,
        None,
        &[("host", HOSTNAME)],
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{settings}");
    assert_eq!(settings["displayName"], "Test Tenant");
}

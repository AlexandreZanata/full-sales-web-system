//! Phase 7 — custom domains contract tests (BR-DM-001, ADR-017).

#[path = "support/mod.rs"]
mod support;

use std::sync::Arc;

use api_http::MockDnsTxtResolver;
use domain_domains::txt_record_name;
use http::StatusCode;
use serde_json::json;

use support::{platform_access_token, request, seed_admin, setup};

const PRO_PLAN: &str = "01900002-0001-7000-8000-000000000002";
const HOSTNAME: &str = "shop.custom.example.com";

async fn set_pro_plan(env: &support::TestEnv) {
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
}

#[tokio::test]
async fn contract_starter_plan_when_add_domain_then_forbidden() {
    let env = setup().await;
    let (_, token) = seed_admin(&env).await;
    let body = json!({ "hostname": HOSTNAME }).to_string();
    let (status, resp) = request(
        &env,
        "POST",
        "/v1/settings/domains",
        Some(&token),
        Some(body),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{resp}");
    assert_eq!(resp["error"]["code"], "PLAN_FEATURE_UNAVAILABLE");
}

#[tokio::test]
async fn contract_dns_verify_when_txt_matches_then_active_and_host_resolves() {
    let mut env = setup().await;
    set_pro_plan(&env).await;
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
    let verified = job["verified"].as_array().expect("verified");
    assert!(
        verified.iter().any(|id| id.as_str() == Some(domain_id)),
        "expected domain verified: {job}"
    );

    let (status, settings) = support::request_with_headers(
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

#[tokio::test]
// T-17-155
async fn contract_set_primary_when_second_domain_then_first_detached() {
    let mut env = setup().await;
    set_pro_plan(&env).await;
    let (_, token) = seed_admin(&env).await;

    let first = request(
        &env,
        "POST",
        "/v1/settings/domains",
        Some(&token),
        Some(json!({ "hostname": "a.custom.example.com" }).to_string()),
    )
    .await;
    assert_eq!(first.0, StatusCode::CREATED);
    let first_id = first.1["id"].as_str().expect("id").to_string();
    env.state.dns_resolver = Arc::new(MockDnsTxtResolver::new());
    let platform_token = platform_access_token(&env).await;
    let _ = request(
        &env,
        "POST",
        &format!("/v1/platform/domains/{first_id}/force-verify"),
        Some(&platform_token),
        None,
    )
    .await;
    let _ = request(
        &env,
        "POST",
        &format!("/v1/settings/domains/{first_id}/set-primary"),
        Some(&token),
        None,
    )
    .await;

    let second = request(
        &env,
        "POST",
        "/v1/settings/domains",
        Some(&token),
        Some(json!({ "hostname": "b.custom.example.com" }).to_string()),
    )
    .await;
    assert_eq!(second.0, StatusCode::CREATED);
    let second_id = second.1["id"].as_str().expect("id");

    env.state.dns_resolver = Arc::new(MockDnsTxtResolver::new());
    let platform_token = platform_access_token(&env).await;
    let _ = request(
        &env,
        "POST",
        &format!("/v1/platform/domains/{second_id}/force-verify"),
        Some(&platform_token),
        None,
    )
    .await;

    let (status, _) = request(
        &env,
        "POST",
        &format!("/v1/settings/domains/{second_id}/set-primary"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let (status, list) = request(&env, "GET", "/v1/settings/domains", Some(&token), None).await;
    assert_eq!(status, StatusCode::OK, "{list}");
    let items = list["data"].as_array().expect("data");
    let first = items
        .iter()
        .find(|d| d["id"].as_str() == Some(first_id.as_str()))
        .expect("first");
    assert_eq!(first["status"], "Detached");
}

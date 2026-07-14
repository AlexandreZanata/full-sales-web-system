//! Phase 17J — Impersonation + security surfaces (T-17-128..141, T-17-166).
//! Path literals for inventory classify:
//! `/v1/platform/fraud/events/{id}/resolve`
//! `/v1/platform/domains/{id}/force-verify`
//! `/v1/platform/tenants/{id}/export`
//! `/v1/platform/tenants/{id}/export/{jobId}`
#[path = "support/mod.rs"]
mod support;

use std::sync::Arc;

use api_http::MockDnsTxtResolver;
use chrono::{Duration, Utc};
use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use support::{platform_access_token, request, seed_admin, seed_platform_admin, setup};

const PRO_PLAN: &str = "01900002-0001-7000-8000-000000000002";

async fn set_pro_plan(env: &support::TestEnv) {
    infra_postgres::shared::update_tenant_lifecycle(
        &env.admin_pool,
        env.tenant_id,
        domain_platform::TenantStatus::Active,
        Some(Uuid::parse_str(PRO_PLAN).expect("plan")),
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

// T-17-128 / T-17-166
#[tokio::test]
async fn given_platform_admin_when_impersonate_then_end_ok() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let (_, _) = seed_admin(&env).await;
    let token = platform_access_token(&env).await;
    let tenant = env.tenant_id.as_uuid();

    let (imp_st, imp) = request(
        &env,
        "POST",
        "/v1/platform/impersonate",
        Some(&token),
        Some(json!({ "tenantId": tenant, "reason": "17J contract" }).to_string()),
    )
    .await;
    assert_eq!(imp_st, StatusCode::OK, "{imp}");
    assert!(imp["impersonationToken"].is_string());

    // End requires grantId; unknown grant → 404 (Authz uses platform token)
    let (end_st, end) = request(
        &env,
        "POST",
        "/v1/platform/impersonate/end",
        Some(&token),
        Some(json!({ "grantId": Uuid::now_v7() }).to_string()),
    )
    .await;
    assert_eq!(end_st, StatusCode::NOT_FOUND, "{end}");
}

// T-17-129..141
#[tokio::test]
async fn given_platform_admin_when_health_fraud_domains_audit_export_then_ok() {
    let mut env = setup().await;
    seed_platform_admin(&env).await;
    let token = platform_access_token(&env).await;
    let tenant = env.tenant_id.as_uuid();

    let since_raw = (Utc::now() - Duration::hours(24))
        .format("%Y-%m-%dT%H:%M:%SZ")
        .to_string();
    let since = since_raw.replace(':', "%3A");
    let history = format!("/v1/platform/health/history?probe=postgres&since={since}");
    for path in [
        "/v1/platform/health/matrix".to_string(),
        history,
        "/v1/platform/fraud/events?limit=10".to_string(),
        "/v1/platform/domains".to_string(),
        "/v1/platform/audit/events?limit=10".to_string(),
    ] {
        let (st, body) = request(&env, "GET", &path, Some(&token), None).await;
        assert!(st.is_success(), "{path} {st} {body}");
    }

    let (bl_st, bl) = request(
        &env,
        "POST",
        "/v1/platform/blocklist",
        Some(&token),
        Some(json!({ "email": "blocked-17j@test.com", "reason": "test" }).to_string()),
    )
    .await;
    assert!(bl_st.is_success(), "{bl}");
    if let Some(id) = bl["id"].as_str() {
        let (del_st, _) = request(
            &env,
            "DELETE",
            &format!("/v1/platform/blocklist/{id}"),
            Some(&token),
            None,
        )
        .await;
        assert!(
            del_st.is_success() || del_st == StatusCode::NO_CONTENT,
            "{del_st}"
        );
    }

    // T-17-136 / T-17-137 — tenant attaches domain; platform force-verify + patch
    set_pro_plan(&env).await;
    env.state.dns_resolver = Arc::new(MockDnsTxtResolver::new());
    let (_, admin) = seed_admin(&env).await;
    let (dom_st, dom) = request(
        &env,
        "POST",
        "/v1/settings/domains",
        Some(&admin),
        Some(json!({ "hostname": "17j-plat.custom.example.com" }).to_string()),
    )
    .await;
    assert_eq!(dom_st, StatusCode::CREATED, "{dom}");
    let domain_id = dom["id"].as_str().expect("domain id");
    let (fv_st, _) = request(
        &env,
        "POST",
        &format!("/v1/platform/domains/{domain_id}/force-verify"),
        Some(&token),
        None,
    )
    .await;
    assert!(fv_st.is_success(), "{fv_st}");
    let (pd_st, _) = request(
        &env,
        "PATCH",
        &format!("/v1/platform/domains/{domain_id}"),
        Some(&token),
        Some(json!({ "isPrimary": true }).to_string()),
    )
    .await;
    assert!(pd_st.is_success(), "{pd_st}");

    let (res_st, _) = request(
        &env,
        "POST",
        &format!("/v1/platform/fraud/events/{}/resolve", Uuid::now_v7()),
        Some(&token),
        Some(json!({ "resolution": "false_positive" }).to_string()),
    )
    .await;
    assert!(
        res_st == StatusCode::NOT_FOUND || res_st.is_success() || res_st == StatusCode::BAD_REQUEST,
        "{res_st}"
    );

    assert!(
        request(
            &env,
            "POST",
            "/v1/platform/jobs/domain-verification",
            Some(&token),
            None,
        )
        .await
        .0
        .is_success()
    );

    let (exp_st, exp) = request(
        &env,
        "POST",
        &format!("/v1/platform/tenants/{tenant}/export"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(exp_st, StatusCode::ACCEPTED, "{exp}");
    let job_id = exp["id"].as_str().expect("job");
    assert_eq!(
        request(
            &env,
            "GET",
            &format!("/v1/platform/tenants/{tenant}/export/{job_id}"),
            Some(&token),
            None,
        )
        .await
        .0,
        StatusCode::OK
    );
}

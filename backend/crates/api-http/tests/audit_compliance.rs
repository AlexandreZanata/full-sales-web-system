//! Phase 10 — audit and compliance contract tests.

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;
use support::{platform_access_token, request, seed_platform_admin, setup};

#[tokio::test]
async fn contract_suspend_tenant_when_platform_admin_then_audit_row_exists() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let token = platform_access_token(&env).await;
    let path = format!("/v1/platform/tenants/{}/suspend", env.tenant_id.as_uuid());
    let body = json!({ "reason": "Compliance review" }).to_string();
    let (status, _) = request(&env, "POST", &path, Some(&token), Some(body)).await;
    assert_eq!(status, StatusCode::OK);

    let audit_path = format!(
        "/v1/platform/audit/events?limit=10&filter[tenant_id]={}&filter[action]=tenant.suspend",
        env.tenant_id.as_uuid()
    );
    let (status, audit) = request(&env, "GET", &audit_path, Some(&token), None).await;
    assert_eq!(status, StatusCode::OK, "{audit}");
    assert_eq!(audit["data"][0]["action"], "tenant.suspend");
    assert_eq!(audit["data"][0]["actorType"], "PlatformAdmin");
}

#[tokio::test]
async fn contract_data_export_when_platform_admin_then_produces_zip_job() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let token = platform_access_token(&env).await;
    let start_path = format!("/v1/platform/tenants/{}/export", env.tenant_id.as_uuid());
    let (status, body) = request(&env, "POST", &start_path, Some(&token), None).await;
    assert_eq!(status, StatusCode::ACCEPTED, "{body}");
    let job_id = body["id"].as_str().expect("job id");

    let mut completed = false;
    for _ in 0..20 {
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        let get_path = format!(
            "/v1/platform/tenants/{}/export/{}",
            env.tenant_id.as_uuid(),
            job_id
        );
        let (status, job) = request(&env, "GET", &get_path, Some(&token), None).await;
        assert_eq!(status, StatusCode::OK, "{job}");
        if job["status"] == "completed" {
            assert!(job["downloadUrl"].as_str().is_some());
            completed = true;
            break;
        }
        if job["status"] == "failed" {
            panic!("export failed: {job}");
        }
    }
    assert!(completed, "export job did not complete in time");
}

#[tokio::test]
async fn contract_audit_range_when_over_90_days_then_rejected() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let token = platform_access_token(&env).await;
    let from = (chrono::Utc::now() - chrono::Duration::days(120))
        .format("%Y-%m-%dT%H:%M:%SZ")
        .to_string()
        .replace(':', "%3A");
    let to = chrono::Utc::now()
        .format("%Y-%m-%dT%H:%M:%SZ")
        .to_string()
        .replace(':', "%3A");
    let path = format!(
        "/v1/platform/audit/events?filter[created_at][gte]={from}&filter[created_at][lte]={to}"
    );
    let (status, body) = request(&env, "GET", &path, Some(&token), None).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    assert_eq!(body["error"]["code"], "AUDIT_RANGE_TOO_WIDE");
}

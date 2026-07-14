//! Phase 17J — Platform tenants + jobs (T-17-107..114, T-17-149).
#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{platform_access_token, request, seed_admin, seed_platform_admin, setup};

const STARTER_PLAN: &str = "01900002-0001-7000-8000-000000000001";

// T-17-107 / T-17-108 / T-17-109 / T-17-110 / T-17-111 / T-17-112
#[tokio::test]
async fn given_platform_admin_when_tenant_lifecycle_then_ok() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let token = platform_access_token(&env).await;

    let (list_st, list) = request(&env, "GET", "/v1/platform/tenants", Some(&token), None).await;
    assert_eq!(list_st, StatusCode::OK);
    assert!(list["data"].is_array() || list.is_array());

    let (create_st, created) = request(
        &env,
        "POST",
        "/v1/platform/tenants",
        Some(&token),
        Some(
            json!({
                "legalName": "17J Co",
                "displayName": "17J",
                "adminEmail": "admin-17j@test.com",
                "planId": STARTER_PLAN,
                "trial": true,
                "cnpj": "11222333000181"
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_st, StatusCode::CREATED, "{created}");
    let id = created["tenantId"].as_str().expect("id");

    let (get_st, got) = request(
        &env,
        "GET",
        &format!("/v1/platform/tenants/{id}"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(get_st, StatusCode::OK);
    assert_eq!(got["id"].as_str().unwrap_or(id), id);

    let (patch_st, _) = request(
        &env,
        "PATCH",
        &format!("/v1/platform/tenants/{id}"),
        Some(&token),
        Some(json!({ "displayName": "17J Renamed" }).to_string()),
    )
    .await;
    assert!(patch_st.is_success(), "{patch_st}");

    let (sus_st, _) = request(
        &env,
        "POST",
        &format!("/v1/platform/tenants/{id}/suspend"),
        Some(&token),
        Some(json!({ "reason": "Contract test" }).to_string()),
    )
    .await;
    assert!(sus_st.is_success(), "{sus_st}");

    let (re_st, _) = request(
        &env,
        "POST",
        &format!("/v1/platform/tenants/{id}/reactivate"),
        Some(&token),
        None,
    )
    .await;
    assert!(re_st.is_success(), "{re_st}");
}

// T-17-113 / T-17-114 / T-17-149 + authz
#[tokio::test]
async fn given_offboard_dunning_or_tenant_admin_when_platform_then_expected() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let token = platform_access_token(&env).await;
    let (_, tenant_admin) = seed_admin(&env).await;

    // Tenant JWT is not a platform token → 401 (see platform_auth_matrix / OD audience).
    let (deny_st, deny) = request(
        &env,
        "GET",
        "/v1/platform/tenants",
        Some(&tenant_admin),
        None,
    )
    .await;
    assert_eq!(deny_st, StatusCode::UNAUTHORIZED);
    assert_eq!(deny["error"]["code"], "UNAUTHORIZED");

    let (create_st, created) = request(
        &env,
        "POST",
        "/v1/platform/tenants",
        Some(&token),
        Some(
            json!({
                "legalName": "Offboard Co",
                "displayName": "Off",
                "adminEmail": "off-17j@test.com",
                "planId": STARTER_PLAN,
                "trial": true,
                "cnpj": "11333444000191"
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_st, StatusCode::CREATED, "{created}");
    let id = created["tenantId"].as_str().expect("id");

    // Trial cannot offboard directly — suspend first (domain SM).
    let (sus_st, _) = request(
        &env,
        "POST",
        &format!("/v1/platform/tenants/{id}/suspend"),
        Some(&token),
        Some(json!({ "reason": "Pre-offboard" }).to_string()),
    )
    .await;
    assert!(sus_st.is_success(), "{sus_st}");

    let (off_st, off) = request(
        &env,
        "POST",
        &format!("/v1/platform/tenants/{id}/offboard"),
        Some(&token),
        None,
    )
    .await;
    assert!(
        off_st.is_success() || off_st == StatusCode::CONFLICT,
        "{off_st} {off}"
    );

    let (job_st, job) = request(
        &env,
        "POST",
        "/v1/platform/jobs/offboarding",
        Some(&token),
        None,
    )
    .await;
    assert!(job_st.is_success(), "{job}");

    let (dun_st, dun) = request(
        &env,
        "POST",
        "/v1/platform/jobs/dunning",
        Some(&token),
        None,
    )
    .await;
    assert!(dun_st.is_success(), "{dun}");
}

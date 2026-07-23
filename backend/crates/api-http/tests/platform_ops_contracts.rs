//! Phase 17J — Platform users + tenant read models (T-17-115..128).
//! Path literals for inventory classify:
//! `/v1/platform/users/{id}/reset-password`
//! `/v1/platform/users/{id}/disable`
//! `/v1/platform/users/{id}/enable`
//! `/v1/platform/tenants/{id}/users`
//! T-17-177 POST /v1/platform/tenants/{id}/users
//! `/v1/platform/tenants/{id}/stats`
//! `/v1/platform/tenants/{id}/orders`
//! `/v1/platform/tenants/{id}/sales`
//! `/v1/platform/tenants/{id}/products`
//! `/v1/platform/tenants/{id}/features`
#[path = "support/mod.rs"]
mod support;

use chrono::{Duration, Utc};
use http::StatusCode;
use serde_json::json;

use support::{
    platform_access_token, request, seed_admin, seed_driver, seed_platform_admin, setup,
};

// T-17-115..125 / T-17-126..127
#[tokio::test]
async fn given_platform_admin_when_users_and_tenant_reads_then_ok() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let token = platform_access_token(&env).await;
    let (_, _) = seed_admin(&env).await;
    let (driver_id, _) = seed_driver(&env, "plat-drv@test.com").await;
    let tenant = env.tenant_id.as_uuid();

    assert_eq!(
        request(&env, "GET", "/v1/platform/users", Some(&token), None)
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(
        request(
            &env,
            "GET",
            &format!("/v1/platform/users/{driver_id}"),
            Some(&token),
            None,
        )
        .await
        .0,
        StatusCode::OK
    );

    // Tenant reads before user mutations (stats query is sensitive to mid-test churn).
    for path in [
        format!("/v1/platform/tenants/{tenant}/users"),
        format!("/v1/platform/tenants/{tenant}/stats"),
        format!("/v1/platform/tenants/{tenant}/orders"),
        format!("/v1/platform/tenants/{tenant}/sales"),
        format!("/v1/platform/tenants/{tenant}/products"),
    ] {
        let (st, body) = request(&env, "GET", &path, Some(&token), None).await;
        assert!(st.is_success(), "{path} {st} {body}");
    }

    // T-17-117
    let (patch_st, _) = request(
        &env,
        "PATCH",
        &format!("/v1/platform/users/{driver_id}"),
        Some(&token),
        Some(json!({ "role": "Seller" }).to_string()),
    )
    .await;
    assert!(
        patch_st.is_success() || patch_st == StatusCode::BAD_REQUEST,
        "{patch_st}"
    );

    for path_extra in [
        ("reset-password", Some("{}".to_string())),
        ("disable", None),
        ("enable", None),
    ] {
        let path = format!("/v1/platform/users/{driver_id}/{}", path_extra.0);
        let (st, _) = request(&env, "POST", &path, Some(&token), path_extra.1).await;
        assert!(
            st.is_success() || st == StatusCode::CONFLICT || st == StatusCode::BAD_REQUEST,
            "{path} {st}"
        );
    }

    let now = Utc::now();
    let (maint_st, _) = request(
        &env,
        "POST",
        "/v1/platform/maintenance",
        Some(&token),
        Some(
            json!({
                "tenantId": tenant,
                "message": "17J window",
                "startsAt": (now - Duration::minutes(1)).to_rfc3339(),
                "endsAt": (now + Duration::hours(1)).to_rfc3339()
            })
            .to_string(),
        ),
    )
    .await;
    assert!(maint_st.is_success(), "{maint_st}");

    let (feat_st, _) = request(
        &env,
        "PATCH",
        &format!("/v1/platform/tenants/{tenant}/features"),
        Some(&token),
        Some(json!({ "customDomain": false }).to_string()),
    )
    .await;
    assert!(feat_st.is_success(), "{feat_st}");

    // T-17-177
    let (create_st, create_body) = request(
        &env,
        "POST",
        &format!("/v1/platform/tenants/{tenant}/users"),
        Some(&token),
        Some(
            json!({
                "name": "Platform Created Seller",
                "email": "plat-created-seller@test.com",
                "role": "Seller"
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_st, StatusCode::CREATED, "{create_body}");
    assert!(
        create_body
            .get("temporaryPassword")
            .and_then(|v| v.as_str())
            .is_some(),
        "{create_body}"
    );
}

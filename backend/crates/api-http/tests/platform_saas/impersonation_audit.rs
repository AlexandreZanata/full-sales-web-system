//! Phase 13B — impersonation scope and audit trail.

use http::StatusCode;
use serde_json::json;

use crate::support::{
    platform_access_token, request, seed_admin, seed_driver, seed_platform_admin, setup,
};

#[tokio::test]
async fn contract_impersonation_when_started_then_scoped_to_tenant_and_audited() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let (_, _) = seed_admin(&env).await;
    seed_driver(&env, "imp-driver@test.com").await;

    let platform_token = platform_access_token(&env).await;
    let impersonate_body = json!({
        "tenantId": env.tenant_id.as_uuid(),
        "reason": "PO acceptance impersonation"
    })
    .to_string();
    let (status, imp) = request(
        &env,
        "POST",
        "/v1/platform/impersonate",
        Some(&platform_token),
        Some(impersonate_body),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{imp}");
    let imp_token = imp["impersonationToken"].as_str().expect("token");

    let (status, users) = request(&env, "GET", "/v1/users", Some(imp_token), None).await;
    assert_eq!(status, StatusCode::OK, "{users}");
    assert!(
        users["data"]
            .as_array()
            .map(|a| !a.is_empty())
            .unwrap_or(false)
    );

    let (status, blocked) = request(&env, "GET", "/v1/platform/users", Some(imp_token), None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(blocked["error"]["code"], "UNAUTHORIZED");

    let audit_path = format!(
        "/v1/platform/audit/events?limit=5&filter[tenant_id]={}&filter[action]=impersonation.start",
        env.tenant_id.as_uuid()
    );
    let (status, audit) = request(&env, "GET", &audit_path, Some(&platform_token), None).await;
    assert_eq!(status, StatusCode::OK, "{audit}");
    assert_eq!(audit["data"][0]["action"], "impersonation.start");
    assert_eq!(audit["data"][0]["actorType"], "PlatformAdmin");
}

#[tokio::test]
async fn contract_impersonation_cannot_access_other_tenant_commerce() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let (_, _) = seed_admin(&env).await;

    let other_tenant = domain_shared::TenantId::generate();
    infra_postgres::shared::insert_tenant(&env.admin_pool, other_tenant, "Other")
        .await
        .expect("tenant");
    let other_commerce = uuid::Uuid::now_v7();
    infra_postgres::commerces::insert_commerce(
        &env.app_pool,
        other_tenant,
        other_commerce,
        "99888777000161",
        "Other Ltda",
        "Other Store",
        serde_json::json!({}),
    )
    .await
    .expect("commerce");

    let platform_token = platform_access_token(&env).await;
    let body = json!({
        "tenantId": env.tenant_id.as_uuid(),
        "reason": "scoped impersonation test"
    })
    .to_string();
    let (_, imp) = request(
        &env,
        "POST",
        "/v1/platform/impersonate",
        Some(&platform_token),
        Some(body),
    )
    .await;
    let imp_token = imp["impersonationToken"].as_str().expect("token");

    let (status, resp) = request(
        &env,
        "GET",
        &format!("/v1/commerces/{other_commerce}"),
        Some(imp_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(resp["error"]["code"], "COMMERCE_NOT_FOUND");
}

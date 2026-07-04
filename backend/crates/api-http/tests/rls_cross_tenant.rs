//! Phase 26 — Cross-tenant RLS contract test (TS-E2E-004).

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;

use support::{request, seed_admin, seed_commerce, setup};

// Contract: TS-E2E-004 — admin from tenant A cannot read tenant B commerce
#[tokio::test]
async fn given_other_tenant_commerce_when_admin_gets_then_not_found() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let other_tenant = domain_shared::TenantId::generate();
    infra_postgres::shared::insert_tenant(&env.admin_pool, other_tenant, "Other Tenant")
        .await
        .expect("other tenant");

    let other_commerce = uuid::Uuid::now_v7();
    infra_postgres::commerces::insert_commerce(
        &env.app_pool,
        other_tenant,
        other_commerce,
        "99888777000161",
        "Other Ltda",
        "Other Store",
        serde_json::json!({"city": "RJ"}),
    )
    .await
    .expect("other commerce");

    let own_commerce = seed_commerce(&env, "11222333000181").await;
    let (own_status, _) = request(
        &env,
        "GET",
        &format!("/v1/commerces/{own_commerce}"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(own_status, StatusCode::OK);

    let (status, body) = request(
        &env,
        "GET",
        &format!("/v1/commerces/{other_commerce}"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "COMMERCE_NOT_FOUND");
}

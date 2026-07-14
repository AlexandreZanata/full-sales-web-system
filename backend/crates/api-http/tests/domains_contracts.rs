//! Phase 17H — Domains CRUD gaps (T-17-151..155).
#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{request, seed_admin, seed_driver, setup};

const PRO_PLAN: &str = "01900002-0001-7000-8000-000000000002";

async fn set_pro(env: &support::TestEnv) {
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

// T-17-151 / T-17-152 / T-17-153 / T-17-154
#[tokio::test]
async fn given_admin_when_domain_list_create_verify_delete_then_ok() {
    let env = setup().await;
    set_pro(&env).await;
    let (_, admin) = seed_admin(&env).await;

    let (list0_st, list0) = request(&env, "GET", "/v1/settings/domains", Some(&admin), None).await;
    assert_eq!(list0_st, StatusCode::OK);
    assert!(list0["data"].is_array());

    let (create_st, created) = request(
        &env,
        "POST",
        "/v1/settings/domains",
        Some(&admin),
        Some(json!({ "hostname": "phase17h-delete.custom.example.com" }).to_string()),
    )
    .await;
    assert_eq!(create_st, StatusCode::CREATED, "create={created}");
    let id = created
        .get("id")
        .and_then(|v| v.as_str())
        .or_else(|| created.pointer("/domain/id").and_then(|v| v.as_str()))
        .expect("id");

    let (verify_st, verify) = request(
        &env,
        "GET",
        &format!("/v1/settings/domains/{id}/verify"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(verify_st, StatusCode::OK, "{verify}");
    assert!(verify["txtRecord"].is_string() || verify["txt_record"].is_string());

    let (del_st, del_body) = request(
        &env,
        "DELETE",
        &format!("/v1/settings/domains/{id}"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(del_st, StatusCode::NO_CONTENT, "{del_body}");
}

// T-17-155 authz
#[tokio::test]
async fn given_driver_when_domains_then_403() {
    let env = setup().await;
    let (_, driver) = seed_driver(&env, "dom-drv@test.com").await;
    let (st, body) = request(&env, "GET", "/v1/settings/domains", Some(&driver), None).await;
    assert_eq!(st, StatusCode::FORBIDDEN);
    assert_eq!(body["error"]["code"], "FORBIDDEN");
}

//! Phase 17H — Payments settings settlement (T-17-156..161).
#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use support::{request, seed_admin, seed_driver, setup};

const PRO_PLAN: &str = "01900002-0001-7000-8000-000000000002";
const TENANT_API_KEY: &str = "sk_valid_tenant_key_99";

async fn set_pro(env: &mut support::TestEnv) {
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

async fn mount_asaas(mock: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/myAccount"))
        .and(header("access_token", TENANT_API_KEY))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "name": "Sandbox",
            "email": "a@b.com"
        })))
        .mount(mock)
        .await;
    Mock::given(method("GET"))
        .and(path("/finance/balance"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "balance": 10.5 })))
        .mount(mock)
        .await;
    Mock::given(method("GET"))
        .and(path("/financialTransactions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [],
            "has_more": false
        })))
        .mount(mock)
        .await;
}

// T-17-156 / T-17-157 / T-17-158 / T-17-160 / T-17-161
#[tokio::test]
async fn given_admin_when_payments_balance_and_tx_then_ok() {
    let mut env = setup().await;
    set_pro(&mut env).await;
    let (_, admin) = seed_admin(&env).await;
    let mock = MockServer::start().await;
    env.state.tenant_asaas_base_url = Some(mock.uri());
    mount_asaas(&mock).await;

    let (st, _) = request(
        &env,
        "POST",
        "/v1/settings/payments/asaas/connect",
        Some(&admin),
        Some(json!({ "apiKey": TENANT_API_KEY }).to_string()),
    )
    .await;
    assert_eq!(st, StatusCode::OK);

    let (get_st, get) = request(&env, "GET", "/v1/settings/payments", Some(&admin), None).await;
    assert_eq!(get_st, StatusCode::OK);
    assert_eq!(get["asaas"]["connected"], true);

    let (put_st, put) = request(
        &env,
        "PUT",
        "/v1/settings/payments",
        Some(&admin),
        Some(
            json!({
                "enabled": true,
                "methods": { "pix": true, "credit": true, "boleto": false },
                "autoCapture": true
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(put_st, StatusCode::OK);
    assert_eq!(put["enabled"], true);

    let (bal_st, bal) = request(
        &env,
        "GET",
        "/v1/settings/payments/balance",
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(bal_st, StatusCode::OK, "{bal}");
    assert!(bal["balanceMinor"].is_number());
    assert_eq!(bal["currency"], "BRL");

    let (tx_st, tx) = request(
        &env,
        "GET",
        "/v1/settings/payments/transactions?limit=10",
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(tx_st, StatusCode::OK, "{tx}");
    assert!(tx["data"].is_array());
}

// T-17-159 + authz
#[tokio::test]
async fn given_disconnect_or_driver_when_payments_then_expected() {
    let mut env = setup().await;
    set_pro(&mut env).await;
    let (_, admin) = seed_admin(&env).await;
    let (_, driver) = seed_driver(&env, "pay-drv@test.com").await;
    let mock = MockServer::start().await;
    env.state.tenant_asaas_base_url = Some(mock.uri());
    mount_asaas(&mock).await;
    assert_eq!(
        request(
            &env,
            "POST",
            "/v1/settings/payments/asaas/connect",
            Some(&admin),
            Some(json!({ "apiKey": TENANT_API_KEY }).to_string()),
        )
        .await
        .0,
        StatusCode::OK
    );

    let (del_st, _) = request(
        &env,
        "DELETE",
        "/v1/settings/payments/asaas/connect",
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(del_st, StatusCode::NO_CONTENT);

    let (drv_st, drv) = request(&env, "GET", "/v1/settings/payments", Some(&driver), None).await;
    assert_eq!(drv_st, StatusCode::FORBIDDEN);
    assert_eq!(drv["error"]["code"], "FORBIDDEN");
}

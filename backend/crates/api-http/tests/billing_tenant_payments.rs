//! Phase 5 — tenant payment controls contract tests.

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use support::{
    request, seed_admin, seed_commerce, seed_commerce_contact, seed_delivery_address, seed_product,
    setup,
};

const PRO_PLAN: &str = "01900002-0001-7000-8000-000000000002";
const STARTER_PLAN: &str = "01900002-0001-7000-8000-000000000001";
const TENANT_API_KEY: &str = "sk_valid_tenant_key_99";

async fn set_tenant_plan(env: &mut support::TestEnv, plan_id: &str) {
    infra_postgres::shared::update_tenant_lifecycle(
        &env.admin_pool,
        env.tenant_id,
        domain_platform::TenantStatus::Active,
        Some(uuid::Uuid::parse_str(plan_id).expect("plan uuid")),
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

fn point_tenant_asaas_at(env: &mut support::TestEnv, base_url: String) {
    env.state.tenant_asaas_base_url = Some(base_url);
}

#[tokio::test]
async fn contract_starter_plan_when_enable_online_payments_then_forbidden() {
    let mut env = setup().await;
    let (_, token) = seed_admin(&env).await;
    set_tenant_plan(&mut env, STARTER_PLAN).await;
    let body = json!({
        "enabled": true,
        "methods": { "pix": true, "credit": true, "boleto": false },
        "autoCapture": true
    })
    .to_string();
    let (status, _) = request(
        &env,
        "PUT",
        "/v1/settings/payments",
        Some(&token),
        Some(body),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn contract_invalid_asaas_key_when_connect_then_bad_request() {
    let mut env = setup().await;
    let (_, token) = seed_admin(&env).await;
    set_tenant_plan(&mut env, PRO_PLAN).await;

    let mock = MockServer::start().await;
    point_tenant_asaas_at(&mut env, mock.uri());
    Mock::given(method("GET"))
        .and(path("/myAccount"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&mock)
        .await;

    let body = json!({ "apiKey": "sk_invalid_key_1234" }).to_string();
    let (status, resp) = request(
        &env,
        "POST",
        "/v1/settings/payments/asaas/connect",
        Some(&token),
        Some(body),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{resp}");
    assert_eq!(resp["error"]["code"], "INVALID_ASAAS_CREDENTIALS");
}

#[tokio::test]
async fn contract_valid_asaas_key_when_connect_then_success() {
    let mut env = setup().await;
    let (_, token) = seed_admin(&env).await;
    set_tenant_plan(&mut env, PRO_PLAN).await;

    let mock = MockServer::start().await;
    point_tenant_asaas_at(&mut env, mock.uri());
    Mock::given(method("GET"))
        .and(path("/myAccount"))
        .and(header("access_token", TENANT_API_KEY))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "name": "Tenant Sandbox",
            "email": "tenant@example.com"
        })))
        .mount(&mock)
        .await;

    let body = json!({ "apiKey": TENANT_API_KEY }).to_string();
    let (status, resp) = request(
        &env,
        "POST",
        "/v1/settings/payments/asaas/connect",
        Some(&token),
        Some(body),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{resp}");
    assert_eq!(resp["connected"], true);
    assert_eq!(resp["accountName"], "Tenant Sandbox");

    let (status, resp) = request(
        &env,
        "GET",
        "/v1/settings/payments",
        Some(&token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(resp["asaas"]["connected"], true);
    assert_eq!(resp["asaas"]["apiKeyLast4"], "y_99");
}

#[tokio::test]
async fn contract_disconnect_asaas_when_connected_then_no_content() {
    let mut env = setup().await;
    let (_, token) = seed_admin(&env).await;
    set_tenant_plan(&mut env, PRO_PLAN).await;
    let encryptor = api_http::AppState::test_credential_encryptor();
    let blob = encryptor.encrypt(TENANT_API_KEY).expect("encrypt");
    infra_postgres::billing::upsert_credentials(
        &env.admin_pool,
        env.tenant_id,
        &blob.ciphertext,
        &blob.nonce,
        blob.key_version,
        "y_99",
    )
    .await
    .expect("credentials");
    infra_postgres::billing::upsert_payment_settings(
        &env.admin_pool,
        env.tenant_id,
        true,
        domain_billing::PaymentMethodToggles::all_enabled(),
        true,
    )
    .await
    .expect("settings");

    let (status, _) = request(
        &env,
        "DELETE",
        "/v1/settings/payments/asaas/connect",
        Some(&token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (status, resp) = request(
        &env,
        "GET",
        "/v1/settings/payments",
        Some(&token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(resp["asaas"]["connected"], false);
    assert_eq!(resp["enabled"], false);
}

#[tokio::test]
async fn contract_portal_submit_when_online_payments_enabled_then_awaiting_payment() {
    let mut env = setup().await;
    set_tenant_plan(&mut env, PRO_PLAN).await;
    let encryptor = api_http::AppState::test_credential_encryptor();
    let blob = encryptor.encrypt(TENANT_API_KEY).expect("encrypt");
    infra_postgres::billing::upsert_credentials(
        &env.admin_pool,
        env.tenant_id,
        &blob.ciphertext,
        &blob.nonce,
        blob.key_version,
        "y_99",
    )
    .await
    .expect("credentials");
    infra_postgres::billing::upsert_payment_settings(
        &env.admin_pool,
        env.tenant_id,
        true,
        domain_billing::PaymentMethodToggles::all_enabled(),
        true,
    )
    .await
    .expect("settings");

    let mock = MockServer::start().await;
    point_tenant_asaas_at(&mut env, mock.uri());
    Mock::given(method("POST"))
        .and(path("/payments"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "id": "pay_order_1" })))
        .mount(&mock)
        .await;

    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let address_id = seed_delivery_address(&env, commerce_id).await;
    let product_id = seed_product(&env, "SKU-PAY-1", "Paid Product", 2500).await;
    let (_, contact_token) =
        seed_commerce_contact(&env, commerce_id, "pay-online@store.com").await;

    let (create_status, create_body) = request(
        &env,
        "POST",
        "/v1/portal/orders",
        Some(&contact_token),
        Some(
            json!({
                "deliveryAddressId": address_id,
                "items": [{ "productId": product_id, "quantity": 1 }]
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let order_id = create_body["id"].as_str().expect("order id");

    let (status, resp) = request(
        &env,
        "POST",
        &format!("/v1/portal/orders/{order_id}/submit"),
        Some(&contact_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{resp}");
    assert_eq!(resp["status"], "AwaitingPayment");
}

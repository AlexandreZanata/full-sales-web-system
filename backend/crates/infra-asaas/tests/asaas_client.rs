//! Wiremock contract tests for Asaas HTTP adapter.

use std::time::Duration;

use application::billing::{
    CreateCustomerRequest, CreateSubscriptionRequest, PaymentGateway,
};
use infra_asaas::{AsaasClient, AsaasConfig};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn test_config(base_url: String, api_key: &str) -> AsaasConfig {
    AsaasConfig {
        base_url,
        api_key: api_key.to_owned(),
        timeout: Duration::from_secs(2),
        max_retries: 1,
        circuit_threshold: 5,
    }
}

#[tokio::test]
async fn contract_create_customer_when_valid_then_returns_id() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/customers"))
        .and(header("access_token", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "cus_000005401844"
        })))
        .mount(&server)
        .await;

    let gateway = AsaasClient::new(test_config(server.uri(), "test-key")).expect("config");
    let response = PaymentGateway::create_customer(
        &gateway,
        CreateCustomerRequest {
            name: "Acme Ltda".into(),
            cpf_cnpj: "11222333000181".into(),
            email: "billing@acme.test".into(),
            external_reference: "01900002-0001-7000-8000-000000000099".into(),
        },
    )
    .await
    .expect("customer");
    assert_eq!(response.id, "cus_000005401844");
}

#[tokio::test]
async fn contract_create_customer_when_unauthorized_then_invalid_credentials() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/customers"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "errors": [{ "code": "invalid_access_token" }]
        })))
        .mount(&server)
        .await;

    let gateway = AsaasClient::new(test_config(server.uri(), "bad-key")).expect("config");
    let err = PaymentGateway::create_customer(
        &gateway,
        CreateCustomerRequest {
            name: "Acme".into(),
            cpf_cnpj: "11222333000181".into(),
            email: "a@b.com".into(),
            external_reference: "tenant-1".into(),
        },
    )
    .await
    .expect_err("unauthorized");
    assert_eq!(err, domain_billing::BillingError::InvalidCredentials);
}

#[tokio::test]
async fn contract_cancel_subscription_when_valid_then_ok() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/subscriptions/sub_123"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let gateway = AsaasClient::new(test_config(server.uri(), "test-key")).expect("config");
    PaymentGateway::cancel_subscription(
        &gateway,
        application::billing::CancelSubscriptionRequest {
            subscription_id: "sub_123".into(),
        },
    )
    .await
    .expect("cancel");
}

#[tokio::test]
async fn contract_create_subscription_when_valid_then_returns_id() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/subscriptions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "sub_456"
        })))
        .mount(&server)
        .await;

    let gateway = AsaasClient::new(test_config(server.uri(), "test-key")).expect("config");
    let response = PaymentGateway::create_subscription(
        &gateway,
        CreateSubscriptionRequest {
            customer_id: "cus_1".into(),
            billing_type: "PIX".into(),
            value: 199.9,
            cycle: "MONTHLY".into(),
            description: "Pro".into(),
            external_reference: "tenant-1".into(),
        },
    )
    .await
    .expect("subscription");
    assert_eq!(response.id, "sub_456");
}

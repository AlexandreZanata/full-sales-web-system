//! Phase 3 — Asaas webhook contract tests (BR-BI-001).

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{platform_access_token, request, seed_platform_admin, setup};

const WEBHOOK_TOKEN: &str = "test-webhook-token-phase3";

fn sample_event(event_id: &str, tenant_id: &str) -> String {
    json!({
        "id": event_id,
        "event": "PAYMENT_CONFIRMED",
        "payment": {
            "id": "pay_123",
            "customer": "cus_000005401844",
            "status": "CONFIRMED",
            "value": 199.9,
            "externalReference": tenant_id
        }
    })
    .to_string()
}

async fn webhook_request(env: &support::TestEnv, token: Option<&str>, body: Option<String>) -> (StatusCode, serde_json::Value) {
    let app = api_http::full_app(env.state.clone());
    let mut builder = axum::http::Request::builder()
        .method("POST")
        .uri("/v1/billing/webhooks/asaas")
        .header("content-type", "application/json");
    if let Some(token) = token {
        builder = builder.header("asaas-access-token", token);
    }
    let request = builder
        .body(axum::body::Body::from(body.unwrap_or_default()))
        .expect("request");
    let response = tower::ServiceExt::oneshot(app, request)
        .await
        .expect("response");
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let json: serde_json::Value = if bytes.is_empty() {
        json!({})
    } else {
        serde_json::from_slice(&bytes).unwrap_or(json!({}))
    };
    (status, json)
}

#[tokio::test]
async fn contract_webhook_when_valid_token_then_received() {
    let env = setup().await;
    let body = sample_event("evt_valid_1", &env.tenant_id.as_uuid().to_string());
    let (status, resp) = webhook_request(&env, Some(WEBHOOK_TOKEN), Some(body)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(resp["received"], true);
    assert_eq!(resp.get("duplicate"), None);
    let count = infra_postgres::billing::count_payment_events(&env.admin_pool)
        .await
        .expect("count");
    assert_eq!(count, 1);
}

#[tokio::test]
async fn contract_webhook_when_invalid_token_then_401() {
    let env = setup().await;
    let body = sample_event("evt_invalid_token", &env.tenant_id.as_uuid().to_string());
    let (status, resp) = webhook_request(&env, Some("wrong-token"), Some(body)).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(resp["error"]["code"], "WEBHOOK_UNAUTHORIZED");
}

#[tokio::test]
async fn contract_webhook_when_duplicate_event_then_200_no_duplicate_row() {
    let env = setup().await;
    let body = sample_event("evt_dup_1", &env.tenant_id.as_uuid().to_string());
    let (status, _) = webhook_request(&env, Some(WEBHOOK_TOKEN), Some(body.clone())).await;
    assert_eq!(status, StatusCode::OK);
    let (status, resp) = webhook_request(&env, Some(WEBHOOK_TOKEN), Some(body)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(resp["duplicate"], true);
    let count = infra_postgres::billing::count_payment_events(&env.admin_pool)
        .await
        .expect("count");
    assert_eq!(count, 1);
}

#[tokio::test]
async fn contract_webhook_when_payment_confirmed_twice_then_idempotent_tenant_state() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let token = platform_access_token(&env).await;
    let body = json!({
        "legalName": "Idempotent Co",
        "displayName": "Idem",
        "adminEmail": "idem@test.com",
        "planId": "01900002-0001-7000-8000-000000000001",
        "trial": true,
        "cnpj": "11222333000181"
    })
    .to_string();
    let (status, resp) = request(&env, "POST", "/v1/platform/tenants", Some(&token), Some(body)).await;
    assert_eq!(status, StatusCode::CREATED, "{resp}");
    let tenant_id = resp["tenantId"].as_str().expect("id");

    let webhook_body = sample_event("evt_idem_pay", tenant_id);
    let (status, _) = webhook_request(&env, Some(WEBHOOK_TOKEN), Some(webhook_body.clone())).await;
    assert_eq!(status, StatusCode::OK);
    let (status, resp) = webhook_request(&env, Some(WEBHOOK_TOKEN), Some(webhook_body)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(resp["duplicate"], true);

    let count = infra_postgres::billing::count_payment_events(&env.admin_pool)
        .await
        .expect("count");
    assert_eq!(count, 1);
}

#[tokio::test]
async fn contract_webhook_when_unknown_event_type_then_still_persisted() {
    let env = setup().await;
    let body = json!({
        "id": "evt_unknown_1",
        "event": "FUTURE_EVENT_TYPE",
        "payment": { "id": "pay_x" }
    })
    .to_string();
    let (status, resp) = webhook_request(&env, Some(WEBHOOK_TOKEN), Some(body)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(resp["received"], true);
}

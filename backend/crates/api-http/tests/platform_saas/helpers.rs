//! Shared helpers for Phase 13 platform SaaS journey tests.

use http::StatusCode;
use serde_json::{Value, json};

use crate::support::TestEnv;

pub const WEBHOOK_TOKEN: &str = "test-webhook-token-phase3";
pub const STARTER_PLAN: &str = "01900002-0001-7000-8000-000000000001";
pub const PRO_PLAN: &str = "01900002-0001-7000-8000-000000000002";

pub fn payment_confirmed_event(event_id: &str, tenant_id: &str) -> String {
    json!({
        "id": event_id,
        "event": "PAYMENT_CONFIRMED",
        "payment": {
            "id": format!("pay_{event_id}"),
            "customer": "cus_mock",
            "status": "CONFIRMED",
            "value": 99.0,
            "externalReference": tenant_id
        }
    })
    .to_string()
}

pub fn payment_overdue_event(event_id: &str, tenant_id: &str) -> String {
    json!({
        "id": event_id,
        "event": "PAYMENT_OVERDUE",
        "payment": {
            "id": format!("pay_{event_id}"),
            "externalReference": tenant_id
        }
    })
    .to_string()
}

pub async fn post_asaas_webhook(env: &TestEnv, body: String) -> (StatusCode, Value) {
    let app = api_http::full_app(env.state.clone());
    let request = axum::http::Request::builder()
        .method("POST")
        .uri("/v1/billing/webhooks/asaas")
        .header("content-type", "application/json")
        .header("asaas-access-token", WEBHOOK_TOKEN)
        .body(axum::body::Body::from(body))
        .expect("request");
    let response = tower::ServiceExt::oneshot(app, request)
        .await
        .expect("response");
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let json: Value = serde_json::from_slice(&bytes).unwrap_or(json!({}));
    (status, json)
}

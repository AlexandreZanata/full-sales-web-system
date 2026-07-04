//! Phase 26 — Audit events list contract tests.

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use infra_postgres::audit::{self, NewAuditEvent};
use uuid::Uuid;

use support::{request, seed_admin, setup};

// Contract: admin lists paginated audit events for tenant
#[tokio::test]
async fn given_audit_event_when_admin_lists_then_returns_row() {
    let env = setup().await;
    let (admin_id, admin_token) = seed_admin(&env).await;
    let event_id = Uuid::now_v7();
    let resource_id = Uuid::now_v7();

    audit::insert_audit_event(
        &env.app_pool,
        env.tenant_id,
        NewAuditEvent {
            id: event_id,
            actor_id: admin_id,
            action: "payment.declared".into(),
            resource_type: "sale".into(),
            resource_id,
            metadata: None,
            correlation_id: None,
        },
    )
    .await
    .expect("insert audit");

    let (status, body) = request(
        &env,
        "GET",
        "/v1/audit/events?page=1&pageSize=10",
        Some(&admin_token),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["total"], 1);
    assert_eq!(body["items"][0]["id"], event_id.to_string());
    assert_eq!(body["items"][0]["action"], "payment.declared");
}

// Contract: driver cannot read audit events
#[tokio::test]
async fn given_driver_when_list_audit_events_then_forbidden() {
    let env = setup().await;
    let (_, driver_token) = support::seed_driver(&env, "driver-audit@test.com").await;

    let (status, body) = request(&env, "GET", "/v1/audit/events", Some(&driver_token), None).await;

    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body["error"]["code"], "FORBIDDEN");
}

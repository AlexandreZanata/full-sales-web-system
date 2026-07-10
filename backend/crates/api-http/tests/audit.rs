//! Phase 68F — Audit events list cursor contract tests.

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use infra_postgres::audit::{self, NewAuditEvent};
use uuid::Uuid;

use support::{request, seed_admin, setup};

#[tokio::test]
async fn given_audit_event_when_admin_lists_then_cursor_envelope() {
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
            actor_type: domain_audit::ActorType::User,
            action: "payment.declared".into(),
            resource_type: "sale".into(),
            resource_id,
            metadata: None,
            correlation_id: None,
            ip: None,
        },
    )
    .await
    .expect("insert audit");

    let (status, body) = request(
        &env,
        "GET",
        "/v1/audit/events?limit=10",
        Some(&admin_token),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert!(body["data"].is_array());
    assert_eq!(body["data"][0]["id"], event_id.to_string());
    assert_eq!(body["data"][0]["action"], "payment.declared");
    assert_eq!(body["pagination"]["limit"], 10);
    assert_eq!(body["pagination"]["has_more"], false);
}

#[tokio::test]
async fn given_audit_event_when_filter_by_actor_then_matches() {
    let env = setup().await;
    let (admin_id, admin_token) = seed_admin(&env).await;
    let event_id = Uuid::now_v7();

    audit::insert_audit_event(
        &env.app_pool,
        env.tenant_id,
        NewAuditEvent {
            id: event_id,
            actor_id: admin_id,
            actor_type: domain_audit::ActorType::User,
            action: "sale.confirmed".into(),
            resource_type: "sale".into(),
            resource_id: Uuid::now_v7(),
            metadata: None,
            correlation_id: None,
            ip: None,
        },
    )
    .await
    .expect("insert audit");

    let (status, body) = request(
        &env,
        "GET",
        &format!("/v1/audit/events?limit=10&filter[actor_id]={admin_id}"),
        Some(&admin_token),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"].as_array().map(|a| a.len()), Some(1));
    assert_eq!(body["data"][0]["actorId"], admin_id.to_string());
}

#[tokio::test]
async fn given_invalid_filter_when_list_audit_events_then_400() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/audit/events?filter[unknown]=x",
        Some(&admin_token),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "invalid_filter_field");
}

#[tokio::test]
async fn given_driver_when_list_audit_events_then_forbidden() {
    let env = setup().await;
    let (_, driver_token) = support::seed_driver(&env, "driver-audit@test.com").await;

    let (status, body) = request(&env, "GET", "/v1/audit/events", Some(&driver_token), None).await;

    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body["error"]["code"], "FORBIDDEN");
}

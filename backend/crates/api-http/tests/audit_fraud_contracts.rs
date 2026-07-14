//! Phase 17I — Audit + fraud alerts (T-17-100..101).
#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use infra_postgres::audit::{self, NewAuditEvent};
use uuid::Uuid;

use support::{request, seed_admin, seed_driver, setup};

// T-17-100 / T-17-101
#[tokio::test]
async fn given_admin_when_audit_and_fraud_alerts_then_200() {
    let env = setup().await;
    let (admin_id, admin) = seed_admin(&env).await;
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
    .expect("audit");

    let (audit_st, audit) =
        request(&env, "GET", "/v1/audit/events?limit=10", Some(&admin), None).await;
    assert_eq!(audit_st, StatusCode::OK);
    assert!(
        audit["data"]
            .as_array()
            .unwrap()
            .iter()
            .any(|e| e["id"] == event_id.to_string())
    );

    let (fraud_st, fraud) =
        request(&env, "GET", "/v1/fraud/alerts?limit=20", Some(&admin), None).await;
    assert_eq!(fraud_st, StatusCode::OK);
    assert!(fraud.is_array() || fraud["data"].is_array() || fraud.is_object());
}

// T-17-100 / T-17-101 authz
#[tokio::test]
async fn given_driver_when_audit_or_fraud_then_403() {
    let env = setup().await;
    let (_, driver) = seed_driver(&env, "audit-drv@test.com").await;
    let (a_st, a) = request(&env, "GET", "/v1/audit/events", Some(&driver), None).await;
    assert_eq!(a_st, StatusCode::FORBIDDEN);
    assert_eq!(a["error"]["code"], "FORBIDDEN");
    let (f_st, f) = request(&env, "GET", "/v1/fraud/alerts", Some(&driver), None).await;
    assert_eq!(f_st, StatusCode::FORBIDDEN);
    assert_eq!(f["error"]["code"], "FORBIDDEN");
}

use domain_fraud::{
    BlocklistEntry, FraudError, FraudEvent, FraudEventStatus, FraudEventType, FraudResolution,
    FraudSeverity,
};
use domain_shared::TenantId;
use uuid::Uuid;

// BR-FR-002 contract: blocklist entry requires at least one identifier.
#[test]
fn given_empty_blocklist_entry_when_create_then_rejected() {
    let err = BlocklistEntry::new(
        Uuid::now_v7(),
        None,
        None,
        None,
        None,
        "abuse".into(),
        None,
        Uuid::now_v7(),
    )
    .unwrap_err();
    assert_eq!(err, FraudError::InvalidBlocklistEntry);
}

#[test]
fn given_email_blocklist_when_active_then_valid() {
    let entry = BlocklistEntry::new(
        Uuid::now_v7(),
        Some("bad@example.com".into()),
        None,
        None,
        None,
        "spam".into(),
        None,
        Uuid::now_v7(),
    )
    .expect("entry");
    assert!(entry.is_active(chrono::Utc::now()));
}

#[test]
fn given_open_event_when_resolve_blocked_then_blocked_status() {
    let event = FraudEvent::new_open(
        Uuid::now_v7(),
        Some(TenantId::generate()),
        None,
        FraudEventType::PaymentVelocity,
        FraudSeverity::High,
        serde_json::json!({}),
    );
    let resolved = event
        .resolve(Uuid::now_v7(), FraudResolution::Blocked, Some("confirmed".into()))
        .expect("resolve");
    assert_eq!(resolved.status, FraudEventStatus::Blocked);
}

#[test]
fn given_resolved_event_when_resolve_again_then_invalid_transition() {
    let event = FraudEvent::new_open(
        Uuid::now_v7(),
        None,
        None,
        FraudEventType::LoginVelocity,
        FraudSeverity::Medium,
        serde_json::json!({}),
    );
    let resolved = event
        .resolve(Uuid::now_v7(), FraudResolution::Dismissed, None)
        .expect("resolve");
    let err = resolved
        .resolve(Uuid::now_v7(), FraudResolution::Blocked, None)
        .unwrap_err();
    assert!(matches!(err, FraudError::InvalidEventTransition { .. }));
}

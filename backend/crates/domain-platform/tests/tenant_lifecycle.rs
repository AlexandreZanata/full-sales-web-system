use chrono::{Duration, Utc};
use domain_platform::{PlatformError, Tenant, TenantStatus};
use domain_shared::TenantId;
use uuid::Uuid;

#[test]
fn given_provisioning_when_activate_trial_then_trial() {
    let mut tenant =
        Tenant::new_provisioning(TenantId::generate(), "Acme LTDA".into(), "Acme".into())
            .expect("tenant");
    let plan = Uuid::now_v7();
    tenant
        .activate_trial(plan, Utc::now() + Duration::days(14))
        .expect("trial");
    assert_eq!(tenant.status, TenantStatus::Trial);
    assert_eq!(tenant.plan_id, Some(plan));
}

#[test]
fn given_active_when_suspend_then_suspended() {
    let mut tenant = sample_active();
    tenant.suspend("fraud review", Utc::now()).expect("suspend");
    assert_eq!(tenant.status, TenantStatus::Suspended);
}

#[test]
fn given_suspended_when_reactivate_then_active() {
    let mut tenant = sample_active();
    tenant.suspend("past due", Utc::now()).expect("suspend");
    tenant.reactivate().expect("reactivate");
    assert_eq!(tenant.status, TenantStatus::Active);
}

#[test]
fn given_deleted_when_any_transition_then_error() {
    let mut tenant = sample_active();
    tenant.begin_offboarding(Utc::now()).expect("offboard");
    tenant.mark_deleted().expect("deleted");
    let err = tenant.reactivate().unwrap_err();
    assert!(matches!(err, PlatformError::InvalidTenantTransition { .. }));
}

#[test]
fn given_offboarding_when_reactivate_then_invalid() {
    let mut tenant = sample_active();
    tenant.begin_offboarding(Utc::now()).expect("offboard");
    let err = tenant.reactivate().unwrap_err();
    assert!(matches!(err, PlatformError::InvalidTenantTransition { .. }));
}

#[test]
fn given_active_when_begin_offboarding_then_offboarding() {
    let mut tenant = sample_active();
    tenant.begin_offboarding(Utc::now()).expect("offboard");
    assert_eq!(tenant.status, TenantStatus::Offboarding);
}

#[test]
fn given_suspended_when_create_sale_gate_then_blocked() {
    let mut tenant = sample_active();
    tenant.suspend("manual", Utc::now()).expect("suspend");
    let err = tenant.ensure_mutations_allowed().unwrap_err();
    assert!(matches!(err, PlatformError::TenantMutationsBlocked(_)));
}

#[test]
fn given_past_due_when_mutations_then_allowed() {
    let mut tenant = sample_active();
    tenant
        .transition_to(TenantStatus::PastDue)
        .expect("past due");
    tenant.ensure_mutations_allowed().expect("ok");
}

#[test]
fn given_short_suspend_reason_when_suspend_then_error() {
    let mut tenant = sample_active();
    let err = tenant.suspend("no", Utc::now()).unwrap_err();
    assert_eq!(err, PlatformError::SuspendReasonTooShort);
}

fn sample_active() -> Tenant {
    let mut tenant =
        Tenant::new_provisioning(TenantId::generate(), "Acme LTDA".into(), "Acme".into())
            .expect("tenant");
    tenant.activate_paid(Uuid::now_v7()).expect("activate");
    tenant
}

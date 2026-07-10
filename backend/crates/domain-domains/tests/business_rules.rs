use chrono::Utc;
use domain_domains::{DomainError, DomainStatus, TenantDomain};
use domain_shared::TenantId;
use uuid::Uuid;

// BR-DM-001 / ADR-017: cannot activate before DNS verification.
#[test]
fn given_pending_domain_when_activate_then_rejected() {
    let mut domain = TenantDomain::add(
        Uuid::now_v7(),
        TenantId::generate(),
        "shop.example.com",
        "token-123456789012345".into(),
        Utc::now(),
    )
    .expect("domain");
    let err = domain.activate(Utc::now()).unwrap_err();
    assert_eq!(err, DomainError::NotVerified);
}

#[test]
fn given_verified_domain_when_activate_then_active() {
    let mut domain = TenantDomain::add(
        Uuid::now_v7(),
        TenantId::generate(),
        "shop.example.com",
        "token-123456789012345".into(),
        Utc::now(),
    )
    .expect("domain");
    domain.start_verifying(Utc::now()).expect("verifying");
    domain.mark_verified(Utc::now()).expect("verified");
    domain.activate(Utc::now()).expect("active");
    assert_eq!(domain.status, DomainStatus::Active);
}

#[test]
fn given_active_primary_when_set_primary_on_verified_then_promotes() {
    let now = Utc::now();
    let mut verified = TenantDomain::add(
        Uuid::now_v7(),
        TenantId::generate(),
        "new.example.com",
        "token-123456789012345".into(),
        now,
    )
    .expect("domain");
    verified.start_verifying(now).expect("verifying");
    verified.mark_verified(now).expect("verified");
    verified.set_primary(now).expect("primary");
    assert!(verified.is_primary);
    assert_eq!(verified.status, DomainStatus::Active);
}

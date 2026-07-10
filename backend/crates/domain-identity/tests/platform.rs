use domain_shared::TenantId;

use domain_identity::{
    Email, IdentityError, ImpersonationGrant, PlatformUserId, UserId,
};

#[test]
fn given_expired_grant_when_ensure_active_then_error() {
    let grant = ImpersonationGrant::new(
        PlatformUserId::generate(),
        TenantId::generate(),
        None,
        "support ticket #42",
        1_000,
    )
    .expect("grant");
    let err = grant.ensure_active(2_000).unwrap_err();
    assert_eq!(err, IdentityError::ImpersonationExpired);
}

#[test]
fn given_revoked_grant_when_ensure_active_then_error() {
    let mut grant = ImpersonationGrant::new(
        PlatformUserId::generate(),
        TenantId::generate(),
        Some(UserId::generate()),
        "audit review",
        u64::MAX,
    )
    .expect("grant");
    grant.revoked = true;
    let err = grant.ensure_active(0).unwrap_err();
    assert_eq!(err, IdentityError::ImpersonationRevoked);
}

#[test]
fn given_short_reason_when_new_grant_then_error() {
    let err = ImpersonationGrant::new(
        PlatformUserId::generate(),
        TenantId::generate(),
        None,
        "no",
        9_999,
    )
    .unwrap_err();
    assert_eq!(err, IdentityError::InvalidImpersonationReason);
}

#[test]
fn given_wrong_tenant_context_when_grant_active_then_still_validates_time_only() {
    let tenant_a = TenantId::generate();
    let tenant_b = TenantId::generate();
    let grant = ImpersonationGrant::new(
        PlatformUserId::generate(),
        tenant_a,
        None,
        "cross-tenant support",
        9_999_999,
    )
    .expect("grant");
    assert_ne!(grant.target_tenant_id, tenant_b);
    grant.ensure_active(1).expect("time ok");
}

#[test]
fn given_inactive_platform_user_when_authenticate_then_error() {
    let user = domain_identity::PlatformUser {
        id: PlatformUserId::generate(),
        email: Email::parse("ops@platform.test").expect("email"),
        active: false,
        mfa_enrolled: true,
    };
    let err = user.ensure_can_authenticate().unwrap_err();
    assert_eq!(err, IdentityError::InactiveUser);
}

use domain_audit::AuditAction;

#[test]
fn given_audit_action_when_as_str_then_stable_codes() {
    assert_eq!(AuditAction::TenantSuspend.as_str(), "tenant.suspend");
    assert_eq!(AuditAction::ImpersonateStart.as_str(), "impersonate.start");
    assert_eq!(AuditAction::UserDisable.as_str(), "user.disable");
}

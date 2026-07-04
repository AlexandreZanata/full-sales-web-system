#[cfg(test)]
mod tests {
    use domain_identity::{Email, FullName, IdentityError, RegisterUserInput, Role, User, UserId};
    use domain_shared::TenantId;

    #[test]
    fn br_ia_002_given_inactive_user_when_ensure_can_authenticate_then_inactive_user() {
        let user = User::register(RegisterUserInput {
            id: UserId::generate(),
            name: FullName::parse("Jane Doe").expect("name"),
            email: Email::parse("jane@example.com").expect("email"),
            role: Role::Driver,
            tenant_id: TenantId::generate(),
        })
        .deactivate();

        assert_eq!(
            user.ensure_can_authenticate(),
            Err(IdentityError::InactiveUser)
        );
    }

    #[test]
    fn br_ia_001_given_driver_role_when_can_register_commerce_then_false() {
        assert!(!Role::Driver.can_register_commerce());
        assert!(Role::Admin.can_register_commerce());
    }
}

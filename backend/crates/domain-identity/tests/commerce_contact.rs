//! BR-IA-003 and CommerceContact registration — contract from ENTITY-SPEC-user-delta.

use domain_identity::{
    ensure_same_commerce, Email, FullName, IdentityError, RegisterUserInput, Role, User, UserId,
};
use domain_shared::TenantId;
use uuid::Uuid;

#[test]
fn given_commerce_contact_without_commerce_id_when_register_then_commerce_required() {
    let result = User::register(RegisterUserInput {
        id: UserId::generate(),
        name: FullName::parse("Portal User").expect("name"),
        email: Email::parse("portal@store.com").expect("email"),
        role: Role::CommerceContact,
        tenant_id: TenantId::generate(),
        commerce_id: None,
        profile_file_id: None,
    });
    assert_eq!(
        result.expect_err("must fail"),
        IdentityError::CommerceRequired
    );
}

#[test]
fn given_driver_with_commerce_id_when_register_then_invalid_commerce_scope() {
    let result = User::register(RegisterUserInput {
        id: UserId::generate(),
        name: FullName::parse("Jane Driver").expect("name"),
        email: Email::parse("driver@test.com").expect("email"),
        role: Role::Driver,
        tenant_id: TenantId::generate(),
        commerce_id: Some(Uuid::now_v7()),
        profile_file_id: None,
    });
    assert_eq!(
        result.expect_err("must fail"),
        IdentityError::InvalidCommerceScope
    );
}

#[test]
fn br_ia_003_given_commerce_contact_for_x_when_access_commerce_y_then_forbidden() {
    let commerce_x = Uuid::now_v7();
    let commerce_y = Uuid::now_v7();
    let err = ensure_same_commerce(Role::CommerceContact, Some(commerce_x), commerce_y)
        .expect_err("cross-commerce access must fail");
    assert_eq!(err, IdentityError::Forbidden);
}

#[test]
fn br_ia_003_given_commerce_contact_for_x_when_access_commerce_x_then_ok() {
    let commerce_x = Uuid::now_v7();
    ensure_same_commerce(Role::CommerceContact, Some(commerce_x), commerce_x).expect("same commerce");
}

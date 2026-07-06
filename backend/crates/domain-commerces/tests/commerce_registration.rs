//! Contract: commerce registration status transitions (BR-CO-010, BR-CO-011).

use domain_commerces::{
    Cnpj, Commerce, CommerceId, CreateCommerceInput, RegistrationMode, RegistrationStatus,
    SubmitCommerceRegistrationInput,
};
use domain_shared::TenantId;
use uuid::Uuid;

fn tenant() -> TenantId {
    TenantId::generate()
}

fn valid_cnpj() -> Cnpj {
    Cnpj::parse("11222333000181").expect("valid test cnpj")
}

#[test]
fn br_co_010_given_seller_submit_when_created_then_pending_inactive() {
    let seller_id = Uuid::now_v7();
    let commerce = Commerce::submit_registration(SubmitCommerceRegistrationInput {
        id: CommerceId::generate(),
        cnpj: valid_cnpj(),
        legal_name: "Acme Ltda".into(),
        trade_name: Some("Acme".into()),
        tenant_id: tenant(),
        submitted_by_user_id: seller_id,
        registration_mode: RegistrationMode::Manual,
    });
    assert_eq!(
        commerce.registration_status(),
        RegistrationStatus::PendingReview
    );
    assert!(!commerce.is_active());
    assert_eq!(commerce.submitted_by_user_id(), Some(seller_id));
}

#[test]
fn br_co_011_given_pending_when_approve_then_active() {
    let commerce = Commerce::submit_registration(SubmitCommerceRegistrationInput {
        id: CommerceId::generate(),
        cnpj: valid_cnpj(),
        legal_name: "Acme Ltda".into(),
        trade_name: None,
        tenant_id: tenant(),
        submitted_by_user_id: Uuid::now_v7(),
        registration_mode: RegistrationMode::CnpjLookup,
    });
    let reviewer = Uuid::now_v7();
    let approved = commerce.approve(reviewer).expect("approve");
    assert!(approved.is_active());
    assert_eq!(approved.registration_status(), RegistrationStatus::Active);
    assert_eq!(approved.reviewed_by_user_id(), Some(reviewer));
}

#[test]
fn given_active_commerce_when_approve_then_invalid_transition() {
    let commerce = Commerce::create(CreateCommerceInput {
        id: CommerceId::generate(),
        cnpj: valid_cnpj(),
        legal_name: "Direct".into(),
        trade_name: None,
        tenant_id: tenant(),
    });
    assert!(commerce.approve(Uuid::now_v7()).is_err());
}

#[test]
fn given_pending_when_reject_without_reason_then_fails() {
    let commerce = Commerce::submit_registration(SubmitCommerceRegistrationInput {
        id: CommerceId::generate(),
        cnpj: valid_cnpj(),
        legal_name: "Acme".into(),
        trade_name: None,
        tenant_id: tenant(),
        submitted_by_user_id: Uuid::now_v7(),
        registration_mode: RegistrationMode::Manual,
    });
    assert!(commerce.reject(Uuid::now_v7(), "   ").is_err());
}

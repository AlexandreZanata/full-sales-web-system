//! RN-PAG1–RN-PAG3 — declared payment domain rules.

mod support;

use chrono::Utc;
use domain_identity::UserId;
use domain_sales::{
    DeclarePaymentInput, DeclaredPaymentMethod, InMemoryPaymentDeclarationAuditPort,
    SaleError, SaleFromDeliveryInput, SaleId,
};

use support::{delivered_order, field_sale_with_item};

// Contract: RN-PAG1 — declaration optional; sale valid without it
#[test]
fn given_confirmed_field_sale_when_no_declaration_then_still_valid() {
    let sale = field_sale_with_item().confirm().expect("confirm");
    assert!(!sale.declared_payment().received());
    assert_eq!(
        sale.declared_payment().method(),
        DeclaredPaymentMethod::NotDeclared
    );
}

// Contract: RN-PAG2 — only responsible driver may declare
#[test]
fn given_sale_when_other_user_declares_then_unauthorized() {
    let sale = field_sale_with_item().confirm().expect("confirm");
    let other = UserId::generate();
    let mut audit = InMemoryPaymentDeclarationAuditPort::default();
    let err = sale
        .declare_payment(
            DeclarePaymentInput {
                method: DeclaredPaymentMethod::Pix,
                received: true,
                declared_at: Utc::now(),
                declaring_user: other,
                notes: None,
            },
            &mut audit,
        )
        .expect_err("must fail");
    assert_eq!(err, SaleError::UnauthorizedPaymentDeclaration);
    assert!(audit.entries.is_empty());
}

// Contract: RN-PAG2 — driver may declare
#[test]
fn given_sale_when_driver_declares_then_payment_recorded() {
    let sale = field_sale_with_item().confirm().expect("confirm");
    let driver = sale.driver_id();
    let mut audit = InMemoryPaymentDeclarationAuditPort::default();
    let updated = sale
        .declare_payment(
            DeclarePaymentInput {
                method: DeclaredPaymentMethod::Cash,
                received: true,
                declared_at: Utc::now(),
                declaring_user: driver,
                notes: Some("paid in full".into()),
            },
            &mut audit,
        )
        .expect("declare");
    assert_eq!(updated.declared_payment().method(), DeclaredPaymentMethod::Cash);
    assert!(updated.declared_payment().received());
    assert_eq!(audit.entries.len(), 1);
}

// Contract: RN-PAG3 — each change appends audit entry
#[test]
fn given_declared_sale_when_changed_then_second_audit_entry() {
    let sale = field_sale_with_item().confirm().expect("confirm");
    let driver = sale.driver_id();
    let mut audit = InMemoryPaymentDeclarationAuditPort::default();
    let first = sale
        .declare_payment(
            DeclarePaymentInput {
                method: DeclaredPaymentMethod::Pix,
                received: false,
                declared_at: Utc::now(),
                declaring_user: driver,
                notes: None,
            },
            &mut audit,
        )
        .expect("first");
    let _second = first
        .declare_payment(
            DeclarePaymentInput {
                method: DeclaredPaymentMethod::Pix,
                received: true,
                declared_at: Utc::now(),
                declaring_user: driver,
                notes: None,
            },
            &mut audit,
        )
        .expect("second");
    assert_eq!(audit.entries.len(), 2);
}

// Contract: order-born sale links order_id and uses delivered quantities
#[test]
fn given_delivered_order_when_from_delivery_then_confirmed_with_totals() {
    let (order, driver) = delivered_order();
    let sale = domain_sales::Sale::from_delivery(SaleFromDeliveryInput {
        id: SaleId::generate(),
        driver_id: driver,
        order,
    })
    .expect("from delivery");
    assert!(sale.order_id().is_some());
    assert_eq!(sale.total().expect("total").amount_minor(), 3_000);
}

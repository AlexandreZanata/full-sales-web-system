//! RN4 — proof photo required for delivery confirmation.

mod support;

use domain_deliveries::{ConfirmDeliveryInput, DeliveryError};

use support::{in_transit_delivery, proof_file};

#[test]
fn given_in_transit_without_proof_when_confirm_then_proof_required() {
    let delivery = in_transit_delivery();
    let driver = delivery.driver_id();
    let err = delivery
        .confirm(
            ConfirmDeliveryInput {
                proof_file_id: None,
                latitude: Some(-23.5),
                longitude: Some(-46.6),
                received_by_name: Some("Maria".into()),
            },
            driver,
        )
        .expect_err("must fail");
    assert_eq!(err, DeliveryError::ProofRequired);
}

#[test]
fn given_in_transit_with_proof_when_confirm_then_delivered() {
    let delivery = in_transit_delivery();
    let driver = delivery.driver_id();
    let confirmed = delivery
        .confirm(
            ConfirmDeliveryInput {
                proof_file_id: Some(proof_file()),
                latitude: Some(-23.5),
                longitude: Some(-46.6),
                received_by_name: Some("Maria".into()),
            },
            driver,
        )
        .expect("confirm");
    assert_eq!(confirmed.status(), domain_deliveries::DeliveryStatus::Delivered);
    assert!(confirmed.proof_file_id().is_some());
}

#[test]
fn given_driver_b_when_confirm_driver_a_delivery_then_not_assigned() {
    let delivery = in_transit_delivery();
    let other_driver = domain_identity::UserId::generate();
    let err = delivery
        .confirm(
            ConfirmDeliveryInput {
                proof_file_id: Some(proof_file()),
                latitude: None,
                longitude: None,
                received_by_name: None,
            },
            other_driver,
        )
        .expect_err("must fail");
    assert_eq!(err, DeliveryError::DriverNotAssigned);
}

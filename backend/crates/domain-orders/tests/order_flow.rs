//! Contract: Draft → PendingApproval → Approved.

mod support;

use domain_orders::OrderStatus;

use support::{draft_submitted_order, port_for_order};

#[test]
fn given_draft_order_when_submit_then_pending_approval() {
    let order = support::draft_order_with_item();
    let submitted = order.submit().expect("submit");
    assert_eq!(submitted.status(), OrderStatus::PendingApproval);
}

#[test]
fn given_empty_draft_when_submit_then_fails() {
    let order = support::empty_draft_order();
    let err = order.submit().expect_err("must fail");
    assert_eq!(err, domain_orders::OrderError::EmptyOrder);
}

#[test]
fn given_pending_approval_when_approve_then_approved_with_reservations() {
    let order = draft_submitted_order();
    let mut port = port_for_order(&order, 50);
    let (approved, reservations) = order.approve(&mut port).expect("approve");
    assert_eq!(approved.status(), OrderStatus::Approved);
    assert_eq!(reservations.len(), 1);
}

#[test]
fn given_insufficient_stock_when_approve_then_fails() {
    let order = draft_submitted_order();
    let mut port = port_for_order(&order, 1);
    let err = order.approve(&mut port).expect_err("oversell");
    assert_eq!(err, domain_orders::OrderError::InsufficientAvailableStock);
}

//! RN10 — reject requires rejection_reason.

mod support;

use domain_orders::OrderStatus;

use support::{draft_submitted_order, reject_without_reason_fails};

#[test]
fn given_pending_approval_when_reject_without_reason_then_fails() {
    let order = draft_submitted_order();
    let err = reject_without_reason_fails(order, "").expect_err("must fail");
    assert_eq!(err, domain_orders::OrderError::RejectionReasonRequired);
}

#[test]
fn given_pending_approval_when_reject_with_reason_then_rejected() {
    let order = draft_submitted_order();
    let rejected = order.reject("Out of delivery zone").expect("reject");
    assert_eq!(rejected.status(), OrderStatus::Rejected);
    assert_eq!(rejected.rejection_reason(), Some("Out of delivery zone"));
}

//! RN6 — cancel before InTransit releases reservation (domain transition rules).

mod support;

use domain_orders::OrderStatus;

use support::{draft_submitted_order, port_for_order};

#[test]
fn given_in_transit_order_when_cancel_then_invalid_transition() {
    let order = draft_submitted_order();
    let mut port = port_for_order(&order, 100);
    let (order, _) = order.approve(&mut port).expect("approve");
    let order = order
        .start_picking()
        .expect("picking")
        .mark_in_transit()
        .expect("in transit");

    let err = order.cancel().expect_err("must fail");
    assert!(matches!(
        err,
        domain_orders::OrderError::InvalidTransition {
            from: OrderStatus::InTransit,
            to: OrderStatus::Cancelled
        }
    ));
}

#[test]
fn given_approved_order_when_cancel_then_cancelled() {
    let order = draft_submitted_order();
    let mut port = port_for_order(&order, 100);
    let (order, _) = order.approve(&mut port).expect("approve");

    let cancelled = order.cancel().expect("cancel");
    assert_eq!(cancelled.status(), OrderStatus::Cancelled);
}

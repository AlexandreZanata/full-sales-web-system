//! RN5 — partial delivery sets order item quantities and PartiallyDelivered status.

mod support;

use domain_inventory::Quantity;
use domain_orders::{DeliveredItemInput, OrderStatus};

use support::in_transit_order_with_two_items;

#[test]
fn given_partial_quantities_when_confirm_delivery_then_partially_delivered() {
    let order = in_transit_order_with_two_items();
    let item_a = order.items()[0].id();
    let item_b = order.items()[1].id();
    let confirmed = order
        .confirm_delivery(&[
            DeliveredItemInput {
                order_item_id: item_a,
                quantity_delivered: Quantity::of(3).expect("qty"),
            },
            DeliveredItemInput {
                order_item_id: item_b,
                quantity_delivered: Quantity::of(5).expect("qty"),
            },
        ])
        .expect("confirm");

    assert_eq!(confirmed.status(), OrderStatus::PartiallyDelivered);
    assert_eq!(
        confirmed.items()[0]
            .quantity_delivered()
            .expect("qty")
            .value(),
        3
    );
    assert_eq!(
        confirmed.items()[1]
            .quantity_delivered()
            .expect("qty")
            .value(),
        5
    );
}

#[test]
fn given_full_quantities_when_confirm_delivery_then_delivered() {
    let order = in_transit_order_with_two_items();
    let item_a = order.items()[0].id();
    let item_b = order.items()[1].id();
    let qty_a = order.items()[0].quantity_requested();
    let qty_b = order.items()[1].quantity_requested();
    let confirmed = order
        .confirm_delivery(&[
            DeliveredItemInput {
                order_item_id: item_a,
                quantity_delivered: qty_a,
            },
            DeliveredItemInput {
                order_item_id: item_b,
                quantity_delivered: qty_b,
            },
        ])
        .expect("confirm");

    assert_eq!(confirmed.status(), OrderStatus::Delivered);
}

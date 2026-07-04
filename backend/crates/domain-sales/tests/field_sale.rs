//! Regression — field sale without order still works (Phase 13).

mod support;

use domain_sales::SaleStatus;

use support::field_sale_with_item;

#[test]
fn given_field_sale_when_create_confirm_then_order_id_null_and_confirmed() {
    let sale = field_sale_with_item()
        .confirm()
        .expect("confirm");
    assert!(sale.order_id().is_none());
    assert_eq!(sale.status(), SaleStatus::Confirmed);
    assert_eq!(sale.items().len(), 1);
}

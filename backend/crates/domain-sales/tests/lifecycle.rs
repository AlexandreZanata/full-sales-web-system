//! Contract: Sale state machine — Pending → Confirmed | Cancelled.

mod support;

use domain_identity::UserId;
use domain_sales::{PaymentMethod, SaleCreateInput, SaleError, SaleId, SaleStatus};

use support::{sample_commerce, sample_product};

#[test]
fn given_pending_sale_with_items_when_confirm_then_confirmed() {
    let commerce = sample_commerce();
    let tenant_id = commerce.tenant_id();
    let product = sample_product(tenant_id);
    let sale = domain_sales::Sale::create(SaleCreateInput {
        id: SaleId::generate(),
        driver_id: UserId::generate(),
        commerce,
        payment_method: PaymentMethod::Cash,
        tenant_id,
    })
    .expect("create")
    .add_item(domain_sales::AddSaleItemInput {
        product,
        quantity: domain_inventory::Quantity::of(2).expect("qty"),
    })
    .expect("add item")
    .confirm()
    .expect("confirm");
    assert_eq!(sale.status(), SaleStatus::Confirmed);
}

#[test]
fn given_confirmed_sale_when_confirm_again_then_invalid_transition() {
    let commerce = sample_commerce();
    let tenant_id = commerce.tenant_id();
    let product = sample_product(tenant_id);
    let sale = domain_sales::Sale::create(SaleCreateInput {
        id: SaleId::generate(),
        driver_id: UserId::generate(),
        commerce,
        payment_method: PaymentMethod::Pix,
        tenant_id,
    })
    .expect("create")
    .add_item(domain_sales::AddSaleItemInput {
        product,
        quantity: domain_inventory::Quantity::of(1).expect("qty"),
    })
    .expect("add")
    .confirm()
    .expect("confirm");
    let err = sale.confirm().expect_err("must fail");
    assert!(matches!(
        err,
        SaleError::InvalidTransition {
            from: SaleStatus::Confirmed,
            to: SaleStatus::Confirmed
        }
    ));
}

#[test]
fn given_confirmed_sale_when_cancel_then_invalid_transition() {
    let commerce = sample_commerce();
    let tenant_id = commerce.tenant_id();
    let product = sample_product(tenant_id);
    let sale = domain_sales::Sale::create(SaleCreateInput {
        id: SaleId::generate(),
        driver_id: UserId::generate(),
        commerce,
        payment_method: PaymentMethod::Debit,
        tenant_id,
    })
    .expect("create")
    .add_item(domain_sales::AddSaleItemInput {
        product,
        quantity: domain_inventory::Quantity::of(1).expect("qty"),
    })
    .expect("add")
    .confirm()
    .expect("confirm");
    let err = sale.cancel().expect_err("must fail");
    assert!(matches!(
        err,
        SaleError::InvalidTransition {
            from: SaleStatus::Confirmed,
            to: SaleStatus::Cancelled
        }
    ));
}

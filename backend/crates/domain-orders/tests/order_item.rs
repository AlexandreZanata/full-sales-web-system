//! RN3 — unit price frozen at order item creation.

mod support;

use domain_inventory::{ProductCreateInput, ProductId, Quantity, Sku, UnitOfMeasure};
use domain_orders::{AddOrderItemInput, OrderCreateInput, OrderId, OrderItemId, OrderSource};
use domain_shared::{Currency, Money};

use support::{delivery_address, sample_commerce};

#[test]
fn given_product_price_change_after_add_when_fulfilled_then_frozen_price_used() {
    let commerce = sample_commerce();
    let tenant_id = commerce.tenant_id();
    let product = domain_inventory::Product::create(ProductCreateInput {
        id: ProductId::generate(),
        name: "Widget".into(),
        sku: Sku::parse("WGT-FRZ").expect("sku"),
        unit_price: Money::new(1_000, Currency::brl()).expect("price"),
        tenant_id,
        active: true,
        category: None,
        unit_of_measure: UnitOfMeasure::Unit,
    });

    let order = domain_orders::Order::create(OrderCreateInput {
        id: OrderId::generate(),
        tenant_id,
        commerce: commerce.clone(),
        created_by: domain_identity::UserId::generate(),
        source: OrderSource::CommercePortal,
        delivery_address: delivery_address(&commerce),
        notes: None,
    })
    .expect("create")
    .add_item(AddOrderItemInput {
        item_id: OrderItemId::generate(),
        product: product.clone(),
        quantity: Quantity::of(3).expect("qty"),
    })
    .expect("add");

    let item = order.items().first().expect("item");
    assert_eq!(item.unit_price().amount_minor(), 1_000);
    assert_eq!(item.line_total().amount_minor(), 3_000);

    let repriced = domain_inventory::Product::create(ProductCreateInput {
        id: product.id(),
        name: "Widget".into(),
        sku: Sku::parse("WGT-FRZ").expect("sku"),
        unit_price: Money::new(2_500, Currency::brl()).expect("new price"),
        tenant_id,
        active: true,
        category: None,
        unit_of_measure: UnitOfMeasure::Unit,
    });

    assert_eq!(repriced.unit_price().amount_minor(), 2_500);
    assert_eq!(item.unit_price().amount_minor(), 1_000);
    assert_eq!(item.line_total().amount_minor(), 3_000);
}

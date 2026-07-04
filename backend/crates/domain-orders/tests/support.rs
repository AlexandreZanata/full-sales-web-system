//! Shared fixtures for domain-orders contract tests.

use domain_commerces::{
    AddressType, Cnpj, Commerce, CommerceAddress, CommerceAddressId, CommerceId,
    CreateCommerceAddressInput, CreateCommerceInput,
};
use domain_identity::UserId;
use domain_inventory::{Product, ProductCreateInput, ProductId, Quantity, Sku, UnitOfMeasure};
use domain_orders::{
    AddOrderItemInput, InMemoryReservationPort, Order, OrderCreateInput, OrderId, OrderItemId,
    OrderSource, StockSnapshot,
};
use domain_shared::{Currency, Money, TenantId};

pub fn sample_commerce() -> Commerce {
    Commerce::create(CreateCommerceInput {
        id: CommerceId::generate(),
        cnpj: Cnpj::parse("11222333000181").expect("cnpj"),
        legal_name: "Acme".into(),
        trade_name: None,
        tenant_id: TenantId::generate(),
    })
}

pub fn delivery_address(commerce: &Commerce) -> CommerceAddress {
    CommerceAddress::create(
        commerce,
        CreateCommerceAddressInput {
            id: CommerceAddressId::generate(),
            tenant_id: commerce.tenant_id(),
            commerce_id: commerce.id(),
            address_type: AddressType::Delivery,
            street: "Rua A".into(),
            number: "100".into(),
            district: None,
            city: "SP".into(),
            state: "SP".into(),
            postal_code: "01310100".into(),
            latitude: None,
            longitude: None,
            is_primary: true,
        },
        &[],
    )
    .expect("address")
}

fn sample_product(tenant_id: TenantId) -> Product {
    Product::create(ProductCreateInput {
        id: ProductId::generate(),
        name: "Widget".into(),
        sku: Sku::parse("WGT-001").expect("sku"),
        unit_price: Money::new(1_000, Currency::brl()).expect("price"),
        tenant_id,
        active: true,
        category: None,
        unit_of_measure: UnitOfMeasure::Unit,
    })
}

pub fn draft_order_with_item() -> Order {
    let commerce = sample_commerce();
    let tenant_id = commerce.tenant_id();
    Order::create(OrderCreateInput {
        id: OrderId::generate(),
        tenant_id,
        commerce: commerce.clone(),
        created_by: UserId::generate(),
        source: OrderSource::SellerVisit,
        delivery_address: delivery_address(&commerce),
        notes: None,
    })
    .expect("create")
    .add_item(AddOrderItemInput {
        item_id: OrderItemId::generate(),
        product: sample_product(tenant_id),
        quantity: Quantity::of(2).expect("qty"),
    })
    .expect("add item")
}

pub fn empty_draft_order() -> Order {
    let commerce = sample_commerce();
    Order::create(OrderCreateInput {
        id: OrderId::generate(),
        tenant_id: commerce.tenant_id(),
        commerce: commerce.clone(),
        created_by: UserId::generate(),
        source: OrderSource::CommercePortal,
        delivery_address: delivery_address(&commerce),
        notes: None,
    })
    .expect("create")
}

pub fn draft_submitted_order() -> Order {
    draft_order_with_item().submit().expect("submit")
}

pub fn port_for_order(order: &Order, balance: i32) -> InMemoryReservationPort {
    let product_id = order.items()[0].product_id();
    InMemoryReservationPort::new(StockSnapshot::default().with_balance(product_id, balance))
}

pub fn reject_without_reason_fails(
    order: Order,
    reason: &str,
) -> Result<Order, domain_orders::OrderError> {
    order.reject(reason)
}

//! Shared fixtures for domain-sales contract tests.

use domain_commerces::{
    AddressType, Cnpj, Commerce, CommerceAddressId, CommerceId, CreateCommerceAddressInput,
    CreateCommerceInput,
};
use domain_identity::UserId;
use domain_inventory::{Product, ProductCreateInput, ProductId, Quantity, Sku, UnitOfMeasure};
use domain_orders::{
    AddOrderItemInput, InMemoryReservationPort, Order, OrderCreateInput, OrderId, OrderItemId,
    OrderSource, StockSnapshot,
};
use domain_sales::{Sale, SaleCreateInput, SaleId, PaymentMethod};
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

pub fn sample_product(tenant_id: TenantId) -> Product {
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

pub fn field_sale_with_item() -> Sale {
    let commerce = sample_commerce();
    let tenant_id = commerce.tenant_id();
    let product = sample_product(tenant_id);
    Sale::create(SaleCreateInput {
        id: SaleId::generate(),
        driver_id: UserId::generate(),
        commerce,
        payment_method: PaymentMethod::Cash,
        tenant_id,
    })
    .expect("create")
    .add_item(domain_sales::AddSaleItemInput {
        product,
        quantity: Quantity::of(2).expect("qty"),
    })
    .expect("add item")
}

pub fn delivered_order() -> (Order, UserId) {
    let commerce = sample_commerce();
    let tenant_id = commerce.tenant_id();
    let driver = UserId::generate();
    let address = domain_commerces::CommerceAddress::create(
        &commerce,
        CreateCommerceAddressInput {
            id: CommerceAddressId::generate(),
            tenant_id,
            commerce_id: commerce.id(),
            address_type: AddressType::Delivery,
            street: "Rua".into(),
            number: "1".into(),
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
    .expect("address");
    let product = sample_product(tenant_id);
    let item_id = OrderItemId::generate();
    let product_id = product.id();
    let order = Order::create(OrderCreateInput {
        id: OrderId::generate(),
        tenant_id,
        commerce,
        created_by: UserId::generate(),
        source: OrderSource::SellerVisit,
        delivery_address: address,
        notes: None,
    })
    .expect("create")
    .add_item(AddOrderItemInput {
        item_id,
        product,
        quantity: Quantity::of(3).expect("qty"),
    })
    .expect("add")
    .submit()
    .expect("submit");

    let mut port = InMemoryReservationPort::new(
        StockSnapshot::default().with_balance(product_id, 50),
    );
    let order = order
        .approve(&mut port)
        .expect("approve")
        .0
        .start_picking()
        .expect("picking")
        .mark_in_transit()
        .expect("in transit")
        .confirm_delivery(&[domain_orders::DeliveredItemInput {
            order_item_id: item_id,
            quantity_delivered: Quantity::of(3).expect("qty"),
        }])
        .expect("deliver");

    (order, driver)
}

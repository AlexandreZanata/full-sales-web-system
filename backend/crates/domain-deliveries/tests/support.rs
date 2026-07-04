use domain_commerces::{
    AddressType, Cnpj, Commerce, CommerceAddressId, CommerceId, CreateCommerceAddressInput,
    CreateCommerceInput,
};
use domain_deliveries::{Delivery, DeliveryCreateInput, DeliveryId};
use domain_identity::UserId;
use domain_inventory::{Product, ProductCreateInput, ProductId, Quantity, Sku, UnitOfMeasure};
use domain_media::FileId;
use domain_orders::{
    AddOrderItemInput, Order, OrderCreateInput, OrderId, OrderItemId, OrderSource,
};
use domain_shared::{Currency, Money, TenantId};

pub fn proof_file() -> FileId {
    FileId::generate()
}

pub fn waiting_delivery(driver_id: UserId) -> Delivery {
    Delivery::create(DeliveryCreateInput {
        id: DeliveryId::generate(),
        tenant_id: TenantId::generate(),
        order_id: OrderId::generate(),
        driver_id,
    })
}

pub fn in_transit_delivery() -> Delivery {
    let driver = UserId::generate();
    waiting_delivery(driver)
        .start_transit(driver)
        .expect("in transit")
}

pub fn in_transit_order_with_two_items() -> Order {
    let commerce = Commerce::create(CreateCommerceInput {
        id: CommerceId::generate(),
        cnpj: Cnpj::parse("11222333000181").expect("cnpj"),
        legal_name: "Acme".into(),
        trade_name: None,
        tenant_id: TenantId::generate(),
    });
    let tenant_id = commerce.tenant_id();
    let address = domain_commerces::CommerceAddress::create(
        &commerce,
        CreateCommerceAddressInput {
            id: CommerceAddressId::generate(),
            tenant_id,
            commerce_id: commerce.id(),
            address_type: AddressType::Delivery,
            street: "Rua A".into(),
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

    let product_a_id = ProductId::generate();
    let product_b_id = ProductId::generate();
    let order = Order::create(OrderCreateInput {
        id: OrderId::generate(),
        tenant_id,
        commerce: commerce.clone(),
        created_by: UserId::generate(),
        source: OrderSource::SellerVisit,
        delivery_address: address,
        notes: None,
    })
    .expect("create")
    .add_item(AddOrderItemInput {
        item_id: OrderItemId::generate(),
        product: Product::create(ProductCreateInput {
            id: product_a_id,
            name: "Item".into(),
            sku: Sku::parse("SKU-A").expect("sku"),
            unit_price: Money::new(1_000, Currency::brl()).expect("price"),
            tenant_id,
            active: true,
            category: None,
            unit_of_measure: UnitOfMeasure::Unit,
        }),
        quantity: Quantity::of(5).expect("qty"),
    })
    .expect("add")
    .add_item(AddOrderItemInput {
        item_id: OrderItemId::generate(),
        product: Product::create(ProductCreateInput {
            id: product_b_id,
            name: "Item".into(),
            sku: Sku::parse("SKU-B").expect("sku"),
            unit_price: Money::new(1_000, Currency::brl()).expect("price"),
            tenant_id,
            active: true,
            category: None,
            unit_of_measure: UnitOfMeasure::Unit,
        }),
        quantity: Quantity::of(10).expect("qty"),
    })
    .expect("add")
    .submit()
    .expect("submit");

    let mut port = domain_orders::InMemoryReservationPort::new(
        domain_orders::StockSnapshot::default()
            .with_balance(product_a_id, 100)
            .with_balance(product_b_id, 100),
    );
    order
        .approve(&mut port)
        .expect("approve")
        .0
        .start_picking()
        .expect("picking")
        .mark_in_transit()
        .expect("in transit")
}

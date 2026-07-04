//! Deliveries integration tests — Phase 12.

use domain_shared::TenantId;
use infra_postgres::commerces;
use infra_postgres::deliveries::{self, ConfirmDeliveryItemUpdate, ConfirmDeliveryTxInput, DeliveryInsert};
use infra_postgres::identity;
use infra_postgres::inventory;
use infra_postgres::inventory::reservations;
use infra_postgres::orders::{self, OrderInsert, OrderItemInsert};
use infra_postgres::sales;
use infra_postgres::{migrate, PgPool, SessionContext};
use testcontainers::runners::AsyncRunner;
use testcontainers::ImageExt;
use testcontainers_modules::postgres::Postgres;
use uuid::Uuid;

struct TestPools {
    admin: PgPool,
    app: PgPool,
    _container: testcontainers::ContainerAsync<Postgres>,
}

async fn setup_pools() -> TestPools {
    let container = Postgres::default()
        .with_tag("18-alpine")
        .start()
        .await
        .expect("start postgres container");

    let host = container.get_host().await.expect("container host");
    let port = container
        .get_host_port_ipv4(5432)
        .await
        .expect("container port");

    let admin_url = format!("postgres://postgres:postgres@{host}:{port}/postgres");
    let app_url = format!("postgres://app_user:app_password@{host}:{port}/postgres");

    let admin = infra_postgres::connect(&admin_url)
        .await
        .expect("connect admin pool");
    migrate(&admin).await.expect("run migrations");

    let app = infra_postgres::connect(&app_url)
        .await
        .expect("connect app pool");

    TestPools {
        admin,
        app,
        _container: container,
    }
}

struct DeliveryFixture {
    tenant: TenantId,
    admin_session: SessionContext,
    driver_session: SessionContext,
    other_driver_session: SessionContext,
    driver_id: Uuid,
    delivery_id: Uuid,
    order_id: Uuid,
    item_id: Uuid,
    product_id: Uuid,
    commerce_id: Uuid,
    proof_file_id: Uuid,
}

async fn seed_in_transit_delivery(pools: &TestPools) -> DeliveryFixture {
    let tenant = TenantId::generate();
    infra_postgres::shared::insert_tenant(&pools.admin, tenant, "Tenant")
        .await
        .expect("tenant");

    let driver_id = Uuid::now_v7();
    let other_driver_id = Uuid::now_v7();
    let commerce_id = Uuid::now_v7();
    let order_id = Uuid::now_v7();
    let item_id = Uuid::now_v7();
    let delivery_id = Uuid::now_v7();
    let address_id = Uuid::now_v7();
    let proof_file_id = Uuid::now_v7();

    identity::insert_user(
        &pools.app,
        tenant,
        identity::InsertUserParams {
            id: driver_id,
            email: "driver@test.com",
            name: "Driver",
            role: "Driver",
            password_hash: "hash",
            commerce_id: None,
            profile_file_id: None,
        },
    )
    .await
    .expect("driver");

    identity::insert_user(
        &pools.app,
        tenant,
        identity::InsertUserParams {
            id: other_driver_id,
            email: "other-driver@test.com",
            name: "Other Driver",
            role: "Driver",
            password_hash: "hash",
            commerce_id: None,
            profile_file_id: None,
        },
    )
    .await
    .expect("other driver");

    commerces::insert_commerce(
        &pools.app,
        tenant,
        commerce_id,
        "11222333000181",
        "Legal",
        "Trade",
        serde_json::json!({"city": "SP"}),
    )
    .await
    .expect("commerce");

    commerces::addresses::insert_address(
        &pools.app,
        tenant,
        commerces::addresses::AddressInsert {
            id: address_id,
            commerce_id,
            address_type: "Delivery".into(),
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
    )
    .await
    .expect("address");

    let product_id = Uuid::now_v7();
    inventory::insert_product_with_catalog(
        &pools.app,
        tenant,
        inventory::ProductInsert {
            id: product_id,
            sku: "DEL-SKU".into(),
            name: "Delivery Product".into(),
            price_amount: 1500,
            price_currency: "BRL".into(),
            category: None,
            unit_of_measure: "Unit".into(),
        },
    )
    .await
    .expect("product");
    inventory::upsert_stock_balance(&pools.app, tenant, driver_id, product_id, 20)
        .await
        .expect("stock");

    let admin_session = SessionContext {
        tenant_id: tenant,
        role: "Admin".into(),
        user_id: Uuid::now_v7(),
        commerce_id: None,
    };

    orders::insert_order(
        &pools.app,
        &admin_session,
        &OrderInsert {
            id: order_id,
            commerce_id,
            created_by_user_id: Uuid::now_v7(),
            source: "SellerVisit".into(),
            status: "InTransit".into(),
            delivery_address_id: address_id,
            notes: None,
            total_amount: 6000,
            total_currency: "BRL".into(),
        },
    )
    .await
    .expect("order");

    orders::insert_order_items(
        &pools.app,
        &admin_session,
        &[OrderItemInsert {
            id: item_id,
            order_id,
            product_id,
            quantity_requested: 4,
            unit_price_amount: 1500,
            unit_price_currency: "BRL".into(),
            line_total_amount: 6000,
        }],
    )
    .await
    .expect("items");

    reservations::reserve_stock_for_order(
        &pools.app,
        tenant,
        &[reservations::ReservationLine {
            id: Uuid::now_v7(),
            order_id,
            order_item_id: item_id,
            product_id,
            quantity: 4,
            driver_id: Some(driver_id),
        }],
    )
    .await
    .expect("reserve");

    let driver_session = SessionContext {
        tenant_id: tenant,
        role: "Driver".into(),
        user_id: driver_id,
        commerce_id: None,
    };

    deliveries::insert_delivery(
        &pools.app,
        &driver_session,
        &DeliveryInsert {
            id: delivery_id,
            order_id,
            driver_id,
            status: "InTransit".into(),
        },
    )
    .await
    .expect("delivery");

    infra_postgres::media::insert_file(
        &pools.app,
        tenant,
        infra_postgres::media::FileInsert {
            id: proof_file_id,
            entity_type: "Delivery".into(),
            entity_id: delivery_id,
            bucket: "proofs".into(),
            object_key: "proof.jpg".into(),
            mime_type: "image/jpeg".into(),
            size_bytes: 1024,
            sha256: "abc123".into(),
            uploaded_by_user_id: driver_id,
        },
    )
    .await
    .expect("proof file");

    DeliveryFixture {
        tenant,
        admin_session,
        driver_session,
        other_driver_session: SessionContext {
            tenant_id: tenant,
            role: "Driver".into(),
            user_id: other_driver_id,
            commerce_id: None,
        },
        driver_id,
        delivery_id,
        order_id,
        item_id,
        product_id,
        commerce_id,
        proof_file_id,
    }
}

#[tokio::test]
async fn given_in_transit_delivery_when_confirm_then_sale_stock_and_reservations() {
    let pools = setup_pools().await;
    let fixture = seed_in_transit_delivery(&pools).await;
    let sale_id = Uuid::now_v7();

    deliveries::confirm_delivery_transaction(
        &pools.app,
        &fixture.driver_session,
        &fixture.admin_session,
        &ConfirmDeliveryTxInput {
            delivery_id: fixture.delivery_id,
            order_id: fixture.order_id,
            driver_id: fixture.driver_id,
            order_status: "Delivered".into(),
            proof_file_id: fixture.proof_file_id,
            latitude: Some(-23.5505),
            longitude: Some(-46.6333),
            received_by_name: Some("Maria".into()),
            order_items: vec![ConfirmDeliveryItemUpdate {
                order_item_id: fixture.item_id,
                quantity_delivered: 4,
            }],
            sale_id,
            commerce_id: fixture.commerce_id,
            sale_items: vec![sales::NewSaleItem {
                id: Uuid::now_v7(),
                sale_id,
                product_id: fixture.product_id,
                quantity: 4,
                unit_price_amount: 1500,
                unit_price_currency: "BRL".into(),
                line_total_amount: 6000,
            }],
            stock_lines: vec![sales::ConfirmSaleItem {
                product_id: fixture.product_id,
                quantity: 4,
            }],
        },
    )
    .await
    .expect("confirm");

    let delivery = deliveries::find_delivery_by_id(
        &pools.app,
        &fixture.driver_session,
        fixture.delivery_id,
    )
    .await
    .expect("delivery")
    .expect("row");
    assert_eq!(delivery.status, "Delivered");

    let order = orders::find_order_by_id(
        &pools.app,
        &SessionContext {
            tenant_id: fixture.tenant,
            role: "Admin".into(),
            user_id: Uuid::now_v7(),
            commerce_id: None,
        },
        fixture.order_id,
    )
    .await
    .expect("order")
    .expect("row");
    assert_eq!(order.status, "Delivered");

    let reservation_rows =
        reservations::list_reservations_by_order(&pools.app, fixture.tenant, fixture.order_id)
            .await
            .expect("reservations");
    assert_eq!(reservation_rows[0].status, "Consumed");

    let sale_ids = sales::list_sale_ids(&pools.app, fixture.tenant)
        .await
        .expect("sales");
    assert!(sale_ids.contains(&sale_id));

    let sale = sales::find_sale_by_id(&pools.app, fixture.tenant, sale_id)
        .await
        .expect("find sale")
        .expect("row");
    assert_eq!(sale.order_id, Some(fixture.order_id));
    assert_eq!(sale.total_amount, 6000);
    assert_eq!(sale.declared_payment_method, "NotDeclared");
}

#[tokio::test]
async fn given_other_driver_when_confirm_delivery_then_not_found() {
    let pools = setup_pools().await;
    let fixture = seed_in_transit_delivery(&pools).await;

    let err = deliveries::confirm_delivery_transaction(
        &pools.app,
        &fixture.other_driver_session,
        &fixture.admin_session,
        &ConfirmDeliveryTxInput {
            delivery_id: fixture.delivery_id,
            order_id: fixture.order_id,
            driver_id: fixture.other_driver_session.user_id,
            order_status: "Delivered".into(),
            proof_file_id: fixture.proof_file_id,
            latitude: None,
            longitude: None,
            received_by_name: None,
            order_items: vec![ConfirmDeliveryItemUpdate {
                order_item_id: fixture.item_id,
                quantity_delivered: 4,
            }],
            sale_id: Uuid::now_v7(),
            commerce_id: fixture.commerce_id,
            sale_items: vec![],
            stock_lines: vec![],
        },
    )
    .await
    .expect_err("must fail");

    assert!(matches!(
        err,
        infra_postgres::PostgresError::Database(sqlx::Error::RowNotFound)
    ));
}

#[tokio::test]
async fn given_partial_qty_when_confirm_then_order_partially_delivered() {
    let pools = setup_pools().await;
    let fixture = seed_in_transit_delivery(&pools).await;

    let sale_id = Uuid::now_v7();

    deliveries::confirm_delivery_transaction(
        &pools.app,
        &fixture.driver_session,
        &fixture.admin_session,
        &ConfirmDeliveryTxInput {
            delivery_id: fixture.delivery_id,
            order_id: fixture.order_id,
            driver_id: fixture.driver_id,
            order_status: "PartiallyDelivered".into(),
            proof_file_id: fixture.proof_file_id,
            latitude: None,
            longitude: None,
            received_by_name: None,
            order_items: vec![ConfirmDeliveryItemUpdate {
                order_item_id: fixture.item_id,
                quantity_delivered: 2,
            }],
            sale_id,
            commerce_id: fixture.commerce_id,
            sale_items: vec![sales::NewSaleItem {
                id: Uuid::now_v7(),
                sale_id,
                product_id: fixture.product_id,
                quantity: 2,
                unit_price_amount: 1500,
                unit_price_currency: "BRL".into(),
                line_total_amount: 3000,
            }],
            stock_lines: vec![sales::ConfirmSaleItem {
                product_id: fixture.product_id,
                quantity: 2,
            }],
        },
    )
    .await
    .expect("confirm");

    let order = orders::find_order_by_id(
        &pools.app,
        &SessionContext {
            tenant_id: fixture.tenant,
            role: "Admin".into(),
            user_id: Uuid::now_v7(),
            commerce_id: None,
        },
        fixture.order_id,
    )
    .await
    .expect("order")
    .expect("row");
    assert_eq!(order.status, "PartiallyDelivered");
}

//! E2E — order → approve → deliver → sale with matching totals (Phase 13).

use domain_shared::TenantId;
use infra_postgres::commerces;
use infra_postgres::deliveries::{
    self, ConfirmDeliveryItemUpdate, ConfirmDeliveryTxInput, DeliveryInsert,
};
use infra_postgres::identity;
use infra_postgres::inventory;
use infra_postgres::inventory::reservations;
use infra_postgres::orders::{self, OrderInsert, OrderItemInsert};
use infra_postgres::sales;
use infra_postgres::{PgPool, SessionContext, migrate};
use testcontainers::ImageExt;
use testcontainers::runners::AsyncRunner;
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

#[tokio::test]
async fn given_order_flow_when_deliver_then_sale_matches_delivered_totals() {
    let pools = setup_pools().await;
    let tenant = TenantId::generate();
    infra_postgres::shared::insert_tenant(&pools.admin, tenant, "Tenant")
        .await
        .expect("tenant");

    let admin_id = Uuid::now_v7();
    let seller_id = Uuid::now_v7();
    let driver_id = Uuid::now_v7();
    let commerce_id = Uuid::now_v7();
    let order_id = Uuid::now_v7();
    let item_id = Uuid::now_v7();
    let delivery_id = Uuid::now_v7();
    let address_id = Uuid::now_v7();
    let proof_file_id = Uuid::now_v7();
    let product_id = Uuid::now_v7();
    let sale_id = Uuid::now_v7();

    identity::insert_user(
        &pools.app,
        tenant,
        identity::InsertUserParams {
            id: admin_id,
            email: "admin@test.com",
            name: "Admin",
            role: "Admin",
            password_hash: "hash",
            commerce_id: None,
            profile_file_id: None,
        },
    )
    .await
    .expect("admin");

    identity::insert_user(
        &pools.app,
        tenant,
        identity::InsertUserParams {
            id: seller_id,
            email: "seller@test.com",
            name: "Seller",
            role: "Seller",
            password_hash: "hash",
            commerce_id: None,
            profile_file_id: None,
        },
    )
    .await
    .expect("seller");

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

    inventory::insert_product_with_catalog(
        &pools.app,
        tenant,
        inventory::ProductInsert {
            id: product_id,
            sku: "BRIDGE-SKU".into(),
            name: "Bridge Product".into(),
            price_amount: 2500,
            price_currency: "BRL".into(),
            category_id: None,
            unit_of_measure: "Unit".into(),
            description: None,
        },
    )
    .await
    .expect("product");
    inventory::upsert_stock_balance(&pools.app, tenant, driver_id, product_id, 30)
        .await
        .expect("stock");

    let seller_session = SessionContext {
        tenant_id: tenant,
        role: "Seller".into(),
        user_id: seller_id,
        commerce_id: None,
    };
    let admin_session = SessionContext {
        tenant_id: tenant,
        role: "Admin".into(),
        user_id: admin_id,
        commerce_id: None,
    };
    let driver_session = SessionContext {
        tenant_id: tenant,
        role: "Driver".into(),
        user_id: driver_id,
        commerce_id: None,
    };

    orders::insert_order(
        &pools.app,
        &seller_session,
        &OrderInsert {
            id: order_id,
            commerce_id,
            created_by_user_id: seller_id,
            source: "SellerVisit".into(),
            status: "PendingApproval".into(),
            delivery_address_id: address_id,
            notes: None,
            total_amount: 10_000,
            total_currency: "BRL".into(),
        },
    )
    .await
    .expect("order");

    orders::insert_order_items(
        &pools.app,
        &seller_session,
        &[OrderItemInsert {
            id: item_id,
            order_id,
            product_id,
            quantity_requested: 4,
            unit_price_amount: 2500,
            unit_price_currency: "BRL".into(),
            line_total_amount: 10_000,
        }],
    )
    .await
    .expect("items");

    orders::approve_order_transaction(
        &pools.app,
        &admin_session,
        order_id,
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
    .expect("approve");

    orders::update_order_status(&pools.app, &admin_session, order_id, "Picking")
        .await
        .expect("picking");
    orders::update_order_status(&pools.app, &admin_session, order_id, "InTransit")
        .await
        .expect("in transit");

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
    .expect("proof");

    deliveries::confirm_delivery_transaction(
        &pools.app,
        &driver_session,
        &admin_session,
        &ConfirmDeliveryTxInput {
            delivery_id,
            order_id,
            driver_id,
            order_status: "Delivered".into(),
            proof_file_id,
            latitude: None,
            longitude: None,
            received_by_name: Some("Receiver".into()),
            order_items: vec![ConfirmDeliveryItemUpdate {
                order_item_id: item_id,
                quantity_delivered: 4,
            }],
            sale_id,
            commerce_id,
            sale_items: vec![sales::NewSaleItem {
                id: Uuid::now_v7(),
                sale_id,
                product_id,
                quantity: 4,
                unit_price_amount: 2500,
                unit_price_currency: "BRL".into(),
                line_total_amount: 10_000,
            }],
            stock_lines: vec![sales::ConfirmSaleItem {
                product_id,
                quantity: 4,
            }],
        },
    )
    .await
    .expect("confirm delivery");

    let sale = sales::find_sale_by_id(&pools.app, tenant, sale_id)
        .await
        .expect("find sale")
        .expect("row");
    assert_eq!(sale.order_id, Some(order_id));
    assert_eq!(sale.total_amount, 10_000);
    assert_eq!(sale.payment_method, "NotDeclared");
}

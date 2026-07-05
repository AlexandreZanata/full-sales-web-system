//! Orders integration tests — Phase 11.

use domain_shared::TenantId;
use infra_postgres::commerces;
use infra_postgres::identity;
use infra_postgres::inventory;
use infra_postgres::inventory::reservations;
use infra_postgres::orders::{self, OrderInsert, OrderItemInsert};
use infra_postgres::{PgPool, PostgresError, SessionContext, migrate};
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

struct OrderFixture {
    tenant: TenantId,
    admin_session: SessionContext,
    seller_session: SessionContext,
    contact_session: SessionContext,
    commerce_id: Uuid,
    address_id: Uuid,
    product_id: Uuid,
    order_id: Uuid,
    item_id: Uuid,
}

async fn seed_order_fixture(pools: &TestPools) -> OrderFixture {
    let tenant = TenantId::generate();
    infra_postgres::shared::insert_tenant(&pools.admin, tenant, "Tenant")
        .await
        .expect("tenant");

    let admin_id = Uuid::now_v7();
    let seller_id = Uuid::now_v7();
    let contact_id = Uuid::now_v7();
    let commerce_id = Uuid::now_v7();

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
            id: contact_id,
            email: "contact@test.com",
            name: "Contact",
            role: "CommerceContact",
            password_hash: "hash",
            commerce_id: Some(commerce_id),
            profile_file_id: None,
        },
    )
    .await
    .expect("contact");

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

    let address_id = Uuid::now_v7();
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
    let driver_id = Uuid::now_v7();
    inventory::insert_product_with_catalog(
        &pools.app,
        tenant,
        inventory::ProductInsert {
            id: product_id,
            sku: "ORD-SKU".into(),
            name: "Order Product".into(),
            price_amount: 1000,
            price_currency: "BRL".into(),
            category_id: None,
            unit_of_measure: "Unit".into(),
        },
    )
    .await
    .expect("product");
    inventory::upsert_stock_balance(&pools.app, tenant, driver_id, product_id, 20)
        .await
        .expect("stock");

    let order_id = Uuid::now_v7();
    let item_id = Uuid::now_v7();
    let seller_session = SessionContext {
        tenant_id: tenant,
        role: "Seller".into(),
        user_id: seller_id,
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
            total_amount: 4000,
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
            unit_price_amount: 1000,
            unit_price_currency: "BRL".into(),
            line_total_amount: 4000,
        }],
    )
    .await
    .expect("items");

    OrderFixture {
        tenant,
        admin_session: SessionContext {
            tenant_id: tenant,
            role: "Admin".into(),
            user_id: admin_id,
            commerce_id: None,
        },
        seller_session,
        contact_session: SessionContext {
            tenant_id: tenant,
            role: "CommerceContact".into(),
            user_id: contact_id,
            commerce_id: Some(commerce_id),
        },
        commerce_id,
        address_id,
        product_id,
        order_id,
        item_id,
    }
}

#[tokio::test]
async fn given_admin_when_approve_order_then_status_and_reservations_atomic() {
    let pools = setup_pools().await;
    let fixture = seed_order_fixture(&pools).await;

    orders::approve_order_transaction(
        &pools.app,
        &fixture.admin_session,
        fixture.order_id,
        &[reservations::ReservationLine {
            id: Uuid::now_v7(),
            order_id: fixture.order_id,
            order_item_id: fixture.item_id,
            product_id: fixture.product_id,
            quantity: 4,
            driver_id: None,
        }],
    )
    .await
    .expect("approve");

    let row = orders::find_order_by_id(&pools.app, &fixture.admin_session, fixture.order_id)
        .await
        .expect("find")
        .expect("order");
    assert_eq!(row.status, "Approved");

    let reservation_rows =
        reservations::list_reservations_by_order(&pools.app, fixture.tenant, fixture.order_id)
            .await
            .expect("reservations");
    assert_eq!(reservation_rows.len(), 1);
    assert_eq!(reservation_rows[0].status, "Active");
}

#[tokio::test]
async fn given_insufficient_stock_when_approve_order_then_rolls_back_status() {
    let pools = setup_pools().await;
    let fixture = seed_order_fixture(&pools).await;

    let err = orders::approve_order_transaction(
        &pools.app,
        &fixture.admin_session,
        fixture.order_id,
        &[reservations::ReservationLine {
            id: Uuid::now_v7(),
            order_id: fixture.order_id,
            order_item_id: fixture.item_id,
            product_id: fixture.product_id,
            quantity: 100,
            driver_id: None,
        }],
    )
    .await
    .expect_err("oversell");

    assert!(matches!(err, PostgresError::InsufficientAvailableStock));

    let row = orders::find_order_by_id(&pools.app, &fixture.admin_session, fixture.order_id)
        .await
        .expect("find")
        .expect("order");
    assert_eq!(row.status, "PendingApproval");
}

#[tokio::test]
async fn given_approved_order_when_cancel_then_releases_reservations() {
    let pools = setup_pools().await;
    let fixture = seed_order_fixture(&pools).await;

    orders::approve_order_transaction(
        &pools.app,
        &fixture.admin_session,
        fixture.order_id,
        &[reservations::ReservationLine {
            id: Uuid::now_v7(),
            order_id: fixture.order_id,
            order_item_id: fixture.item_id,
            product_id: fixture.product_id,
            quantity: 4,
            driver_id: None,
        }],
    )
    .await
    .expect("approve");

    orders::cancel_order_transaction(&pools.app, &fixture.admin_session, fixture.order_id, true)
        .await
        .expect("cancel");

    let row = orders::find_order_by_id(&pools.app, &fixture.admin_session, fixture.order_id)
        .await
        .expect("find")
        .expect("order");
    assert_eq!(row.status, "Cancelled");

    let reservation_rows =
        reservations::list_reservations_by_order(&pools.app, fixture.tenant, fixture.order_id)
            .await
            .expect("reservations");
    assert_eq!(reservation_rows[0].status, "Released");
}

#[tokio::test]
async fn given_role_scoped_rls_when_seller_queries_then_sees_own_orders_only() {
    let pools = setup_pools().await;
    let fixture = seed_order_fixture(&pools).await;

    let visible = orders::find_order_by_id(&pools.app, &fixture.seller_session, fixture.order_id)
        .await
        .expect("seller find");
    assert!(visible.is_some());

    let other_seller = SessionContext {
        tenant_id: fixture.tenant,
        role: "Seller".into(),
        user_id: Uuid::now_v7(),
        commerce_id: None,
    };
    let hidden = orders::find_order_by_id(&pools.app, &other_seller, fixture.order_id)
        .await
        .expect("other seller");
    assert!(hidden.is_none());

    let contact_visible =
        orders::find_order_by_id(&pools.app, &fixture.contact_session, fixture.order_id)
            .await
            .expect("contact find");
    assert!(contact_visible.is_some());
}

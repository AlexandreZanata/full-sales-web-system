//! Phase 1c repository + hardening integration tests.

use chrono::{Duration, Utc};
use domain_inventory::validate_adjustment_reason;
use domain_shared::TenantId;
use infra_postgres::audit::{self, NewAuditEvent};
use infra_postgres::commerces;
use infra_postgres::identity;
use infra_postgres::inventory;
use infra_postgres::reports;
use infra_postgres::sales::{self, SaleFilters};
use infra_postgres::shared;
use infra_postgres::{PgPool, migrate};
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

async fn seed_tenant_with_sale(app: &PgPool, admin: &PgPool) -> (TenantId, Uuid, Uuid, Uuid) {
    let tenant = TenantId::generate();
    shared::insert_tenant(admin, tenant, "Test Tenant")
        .await
        .expect("insert tenant");

    let user = Uuid::now_v7();
    identity::insert_user(
        app,
        tenant,
        identity::InsertUserParams {
            id: user,
            email: "driver@test.com",
            name: "Driver",
            role: "Driver",
            password_hash: "hash",
            commerce_id: None,
            profile_file_id: None,
        },
    )
    .await
    .expect("insert user");

    let commerce = Uuid::now_v7();
    commerces::insert_commerce(
        app,
        tenant,
        commerce,
        "11222333000181",
        "Legal Name",
        "Trade Name",
        serde_json::json!({"city": "SP"}),
    )
    .await
    .expect("insert commerce");

    let product = Uuid::now_v7();
    inventory::insert_product(app, tenant, product, "SKU-1", "Product", 1000, "BRL")
        .await
        .expect("insert product");

    inventory::upsert_stock_balance(app, tenant, user, product, 10)
        .await
        .expect("seed stock");

    let sale = Uuid::now_v7();
    sales::insert_sale(app, tenant, sale, user, commerce, "Cash")
        .await
        .expect("insert sale");

    reports::insert_signing_key(app, tenant, "key-1", &[0u8; 32])
        .await
        .expect("insert signing key");

    (tenant, user, commerce, sale)
}

#[tokio::test]
async fn rls_audit_events_isolated() {
    let pools = setup_pools().await;
    let tenant_a = TenantId::generate();
    let tenant_b = TenantId::generate();
    shared::insert_tenant(&pools.admin, tenant_a, "A")
        .await
        .expect("tenant A");
    shared::insert_tenant(&pools.admin, tenant_b, "B")
        .await
        .expect("tenant B");

    let actor_a = Uuid::now_v7();
    audit::insert_audit_event(
        &pools.app,
        tenant_a,
        NewAuditEvent {
            id: Uuid::now_v7(),
            actor_id: actor_a,
            action: "sale.create".to_owned(),
            resource_type: "sale".to_owned(),
            resource_id: Uuid::now_v7(),
            metadata: None,
            correlation_id: None,
        },
    )
    .await
    .expect("insert audit A");

    let ids_a = audit::list_audit_event_ids(&pools.app, tenant_a)
        .await
        .expect("list audit A");
    assert_eq!(ids_a.len(), 1);

    let ids_b = audit::list_audit_event_ids(&pools.app, tenant_b)
        .await
        .expect("list audit B");
    assert!(ids_b.is_empty());
}

#[tokio::test]
async fn given_stock_movement_when_app_user_updates_then_rejected() {
    let pools = setup_pools().await;
    let (tenant, user, _commerce, _sale) = seed_tenant_with_sale(&pools.app, &pools.admin).await;

    let product = inventory::list_product_ids(&pools.app, tenant)
        .await
        .expect("list products")[0];

    let movement_id = Uuid::now_v7();
    inventory::insert_stock_movement(
        &pools.app,
        tenant,
        inventory::StockMovementInsert {
            id: movement_id,
            product_id: product,
            responsible_id: user,
            movement_type: "Inbound".to_owned(),
            quantity: 1,
            reference_id: None,
            reason: None,
        },
    )
    .await
    .expect("insert movement");

    let mut tx = pools.app.begin().await.expect("begin");
    infra_postgres::rls::apply_tenant_context(&mut tx, tenant)
        .await
        .expect("rls");
    let result = sqlx::query("UPDATE inventory.stock_movements SET quantity = 99 WHERE id = $1")
        .bind(movement_id)
        .execute(&mut *tx)
        .await;
    assert!(
        result.is_err(),
        "append-only: UPDATE must fail for app_user"
    );
}

#[tokio::test]
async fn given_pending_sale_when_cancel_then_cancelled_at_set() {
    let pools = setup_pools().await;
    let (tenant, user, commerce, sale) = seed_tenant_with_sale(&pools.app, &pools.admin).await;

    sales::cancel_sale_status(&pools.app, tenant, sale)
        .await
        .expect("cancel");

    let row = sqlx::query_as::<_, (String, Option<chrono::DateTime<Utc>>)>(
        "SELECT status, cancelled_at FROM sales.sales WHERE id = $1",
    )
    .bind(sale)
    .fetch_one(&pools.admin)
    .await
    .expect("fetch sale");

    assert_eq!(row.0, "Cancelled");
    assert!(row.1.is_some());

    let _ = (user, commerce);
}

#[tokio::test]
async fn given_sales_when_list_by_commerce_filter_then_matching_only() {
    let pools = setup_pools().await;
    let (tenant, user, commerce, sale) = seed_tenant_with_sale(&pools.app, &pools.admin).await;

    let other_commerce = Uuid::now_v7();
    commerces::insert_commerce(
        &pools.app,
        tenant,
        other_commerce,
        "99888777000100",
        "Other Legal",
        "Other Trade",
        serde_json::json!({}),
    )
    .await
    .expect("insert other commerce");

    let other_sale = Uuid::now_v7();
    sales::insert_sale(&pools.app, tenant, other_sale, user, other_commerce, "Pix")
        .await
        .expect("insert other sale");

    let filters = SaleFilters {
        commerce_id: Some(commerce),
        driver_id: None,
        status: None,
        from: None,
        to: None,
    };
    let rows = sales::list_sales(&pools.app, tenant, &filters, 10, 0)
        .await
        .expect("list sales");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].id, sale);
}

#[tokio::test]
async fn given_confirmed_sale_when_list_movements_by_reference_then_found() {
    let pools = setup_pools().await;
    let (tenant, user, commerce, sale) = seed_tenant_with_sale(&pools.app, &pools.admin).await;
    let product = inventory::list_product_ids(&pools.app, tenant)
        .await
        .expect("products")[0];

    let item_id = Uuid::now_v7();
    sales::insert_sale_item(
        &pools.app,
        tenant,
        sales::NewSaleItem {
            id: item_id,
            sale_id: sale,
            product_id: product,
            quantity: 2,
            unit_price_amount: 1000,
            unit_price_currency: "BRL".to_owned(),
            line_total_amount: 2000,
        },
    )
    .await
    .expect("add item");

    sales::confirm_sale_with_stock(
        &pools.app,
        tenant,
        user,
        sale,
        &[sales::ConfirmSaleItem {
            product_id: product,
            quantity: 2,
        }],
    )
    .await
    .expect("confirm");

    let movements = inventory::list_stock_movements_by_reference(&pools.app, tenant, sale)
        .await
        .expect("list movements");
    assert_eq!(movements.len(), 1);
    assert_eq!(movements[0].movement_type, "SaleOutbound");
    assert_eq!(movements[0].reference_id, Some(sale));

    let _ = commerce;
}

#[tokio::test]
async fn given_adjustment_when_reason_valid_then_movement_and_balance_updated() {
    assert!(validate_adjustment_reason(Some("cycle count")).is_ok());

    let pools = setup_pools().await;
    let (tenant, user, _commerce, _sale) = seed_tenant_with_sale(&pools.app, &pools.admin).await;
    let product = inventory::list_product_ids(&pools.app, tenant)
        .await
        .expect("products")[0];

    inventory::insert_adjustment_movement(
        &pools.app,
        tenant,
        user,
        product,
        3,
        "cycle count correction",
        -2,
    )
    .await
    .expect("adjustment");

    let qty = inventory::get_stock_quantity(&pools.app, tenant, user, product)
        .await
        .expect("qty");
    assert_eq!(qty, Some(8));

    let movements =
        inventory::list_stock_movements_by_reference(&pools.app, tenant, Uuid::now_v7())
            .await
            .expect("list");
    assert!(movements.is_empty());
}

#[tokio::test]
async fn given_active_signing_key_when_lookup_then_found() {
    let pools = setup_pools().await;
    let (tenant, _user, _commerce, _sale) = seed_tenant_with_sale(&pools.app, &pools.admin).await;

    let key = reports::find_active_signing_key(&pools.app, tenant)
        .await
        .expect("find key");
    assert!(key.is_some());
    assert_eq!(key.unwrap().public_key_id, "key-1");
}

#[tokio::test]
async fn given_commerces_when_list_paginated_then_returns_rows() {
    let pools = setup_pools().await;
    let (tenant, _user, _commerce, _sale) = seed_tenant_with_sale(&pools.app, &pools.admin).await;

    let count = commerces::count_commerces(&pools.app, tenant, None)
        .await
        .expect("count");
    assert_eq!(count, 1);

    let rows = commerces::list_commerces(&pools.app, tenant, None, 10, 0)
        .await
        .expect("list");
    assert_eq!(rows.len(), 1);
}

#[tokio::test]
async fn given_users_when_list_paginated_then_returns_rows() {
    let pools = setup_pools().await;
    let (tenant, _user, _commerce, _sale) = seed_tenant_with_sale(&pools.app, &pools.admin).await;

    let count = identity::count_users(&pools.app, tenant)
        .await
        .expect("count");
    assert_eq!(count, 1);

    let rows = identity::list_users(&pools.app, tenant, 10, 0)
        .await
        .expect("list");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].email, "driver@test.com");
}

#[tokio::test]
async fn given_sales_when_date_filter_then_respects_range() {
    let pools = setup_pools().await;
    let (tenant, user, commerce, sale) = seed_tenant_with_sale(&pools.app, &pools.admin).await;

    let now = Utc::now();
    let filters = SaleFilters {
        commerce_id: None,
        driver_id: Some(user),
        status: None,
        from: Some(now - Duration::hours(1)),
        to: Some(now + Duration::hours(1)),
    };
    let rows = sales::list_sales(&pools.app, tenant, &filters, 10, 0)
        .await
        .expect("list");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].id, sale);

    let empty = SaleFilters {
        commerce_id: None,
        driver_id: Some(user),
        status: None,
        from: Some(now + Duration::hours(2)),
        to: None,
    };
    let rows = sales::list_sales(&pools.app, tenant, &empty, 10, 0)
        .await
        .expect("list empty");
    assert!(rows.is_empty());

    let _ = commerce;
}

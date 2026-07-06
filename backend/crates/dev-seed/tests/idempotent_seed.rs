//! Contract: dev seed is idempotent and populates every admin list screen.

use dev_seed::{SeedPools, seed_dev_dataset};
use infra_postgres::{connect, migrate};
use testcontainers::ImageExt;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;

async fn setup_pools() -> (
    infra_postgres::PgPool,
    infra_postgres::PgPool,
    testcontainers::ContainerAsync<Postgres>,
) {
    let container = Postgres::default()
        .with_tag("18-alpine")
        .start()
        .await
        .expect("start postgres");

    let host = container.get_host().await.expect("host");
    let port = container.get_host_port_ipv4(5432).await.expect("port");
    let admin_url = format!("postgres://postgres:postgres@{host}:{port}/postgres");
    let app_url = format!("postgres://app_user:app_password@{host}:{port}/postgres");

    let admin = connect(&admin_url).await.expect("admin pool");
    migrate(&admin).await.expect("migrate");
    let app = connect(&app_url).await.expect("app pool");
    (admin, app, container)
}

#[tokio::test]
async fn given_fresh_db_when_seed_twice_then_idempotent_and_populated() {
    let (admin, app, _container) = setup_pools().await;
    let pools = SeedPools { admin, app };

    seed_dev_dataset(&pools).await.expect("first seed");
    seed_dev_dataset(&pools).await.expect("second seed");

    let tenant = dev_seed::ids::tenant_id();
    let users = infra_postgres::identity::list_user_ids(&pools.app, tenant)
        .await
        .expect("users");
    assert!(users.len() >= 6);

    let orders = infra_postgres::orders::list_orders(
        &pools.app,
        &infra_postgres::SessionContext {
            tenant_id: tenant,
            role: "Admin".into(),
            user_id: dev_seed::ids::admin_user_id(),
            commerce_id: None,
        },
        &infra_postgres::orders::OrderListFilters {
            status: None,
            commerce_id: None,
            from: None,
            to: None,
        },
        50,
        0,
    )
    .await
    .expect("orders");
    assert!(orders.len() >= 6);

    let sales = infra_postgres::sales::list_sale_ids(&pools.app, tenant)
        .await
        .expect("sales");
    assert!(sales.len() >= 3);

    let deliveries = infra_postgres::deliveries::list_deliveries(
        &pools.app,
        &infra_postgres::SessionContext {
            tenant_id: tenant,
            role: "Admin".into(),
            user_id: dev_seed::ids::admin_user_id(),
            commerce_id: None,
        },
        &infra_postgres::deliveries::DeliveryFilters {
            driver_id: None,
            status: None,
            from: None,
            to: None,
        },
        20,
        0,
    )
    .await
    .expect("deliveries");
    assert!(deliveries.len() >= 3);

    let reports = infra_postgres::reports::list_report_ids(&pools.app, tenant)
        .await
        .expect("reports");
    assert_eq!(reports.len(), 2);

    let audit_count = infra_postgres::audit::count_audit_events(&pools.app, tenant)
        .await
        .expect("audit");
    assert!(audit_count >= 10);

    let categories = infra_postgres::inventory::product_categories::list_categories(
        &pools.app, tenant, None, 20, 0,
    )
    .await
    .expect("categories");
    assert!(categories.len() >= 5);
    assert!(
        categories
            .iter()
            .any(|row| row.slug == "bebidas" && row.name == "Bebidas")
    );
}

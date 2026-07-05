//! Product categories infra integration tests — Phase 43.

use domain_shared::TenantId;
use infra_postgres::inventory;
use infra_postgres::inventory::product_categories::{self, CategoryInsert};
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

#[tokio::test]
async fn given_category_when_product_assigned_then_filter_by_slug() {
    let pools = setup_pools().await;
    let tenant = TenantId::generate();
    infra_postgres::shared::insert_tenant(&pools.admin, tenant, "Tenant")
        .await
        .expect("tenant");

    let category_id = Uuid::now_v7();
    product_categories::insert_category(
        &pools.app,
        tenant,
        CategoryInsert {
            id: category_id,
            name: "Beverages".into(),
            slug: "beverages".into(),
            description: None,
            sort_order: 0,
            active: true,
            image_file_id: None,
        },
    )
    .await
    .expect("category");

    let product_id = Uuid::now_v7();
    inventory::insert_product_with_catalog(
        &pools.app,
        tenant,
        inventory::ProductInsert {
            id: product_id,
            sku: "BEV-001".into(),
            name: "Cola".into(),
            price_amount: 500,
            price_currency: "BRL".into(),
            category_id: Some(category_id),
            unit_of_measure: "Unit".into(),
            description: None,
        },
    )
    .await
    .expect("product");

    let rows = inventory::list_portal_products(&pools.app, tenant, Some("beverages"), 10, 0)
        .await
        .expect("list");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].category_slug.as_deref(), Some("beverages"));
}

#[tokio::test]
async fn given_categories_when_reordered_then_sort_order_updates() {
    let pools = setup_pools().await;
    let tenant = TenantId::generate();
    infra_postgres::shared::insert_tenant(&pools.admin, tenant, "Tenant")
        .await
        .expect("tenant");

    let first = Uuid::now_v7();
    let second = Uuid::now_v7();
    for (id, slug, sort) in [(first, "alpha", 0), (second, "beta", 1)] {
        product_categories::insert_category(
            &pools.app,
            tenant,
            CategoryInsert {
                id,
                name: slug.into(),
                slug: slug.into(),
                description: None,
                sort_order: sort,
                active: true,
                image_file_id: None,
            },
        )
        .await
        .expect("insert");
    }

    product_categories::reorder_categories(&pools.app, tenant, &[second, first])
        .await
        .expect("reorder");

    let rows = product_categories::list_active_categories(&pools.app, tenant)
        .await
        .expect("list");
    assert_eq!(rows[0].id, second);
    assert_eq!(rows[1].id, first);
}

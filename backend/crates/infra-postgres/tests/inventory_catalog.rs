//! Inventory catalog integration tests — Phase 10.

use std::sync::Arc;

use domain_shared::TenantId;
use infra_postgres::inventory;
use infra_postgres::inventory::product_images;
use infra_postgres::inventory::reservations;
use infra_postgres::{PgPool, PostgresError, migrate};
use testcontainers::ImageExt;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;
use tokio::sync::Barrier;
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

async fn seed_product_with_stock(app: &PgPool, tenant: TenantId, qty: i32) -> Uuid {
    let product_id = Uuid::now_v7();
    let driver_id = Uuid::now_v7();
    inventory::insert_product_with_catalog(
        app,
        tenant,
        inventory::ProductInsert {
            id: product_id,
            sku: "SKU-CAT".into(),
            name: "Catalog Product".into(),
            price_amount: 1500,
            price_currency: "BRL".into(),
            category_id: None,
            unit_of_measure: "Box".into(),
            description: None,
        },
    )
    .await
    .expect("product");
    inventory::upsert_stock_balance(app, tenant, driver_id, product_id, qty)
        .await
        .expect("stock");
    product_id
}

#[tokio::test]
async fn given_catalog_columns_when_insert_product_then_persisted() {
    let pools = setup_pools().await;
    let tenant = TenantId::generate();
    infra_postgres::shared::insert_tenant(&pools.admin, tenant, "Tenant")
        .await
        .expect("tenant");

    let product_id = seed_product_with_stock(&pools.app, tenant, 5).await;
    let rows = inventory::find_products_by_ids(&pools.app, tenant, &[product_id])
        .await
        .expect("find");
    let row = rows.first().expect("product");
    assert_eq!(row.category_name.as_deref(), None);
    assert_eq!(row.unit_of_measure, "Box");
}

#[tokio::test]
async fn given_product_images_when_list_then_sorted_by_sort_order() {
    let pools = setup_pools().await;
    let tenant = TenantId::generate();
    infra_postgres::shared::insert_tenant(&pools.admin, tenant, "Tenant")
        .await
        .expect("tenant");

    let product_id = seed_product_with_stock(&pools.app, tenant, 1).await;
    let admin_id = Uuid::now_v7();
    infra_postgres::identity::insert_user(
        &pools.app,
        tenant,
        infra_postgres::identity::InsertUserParams {
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

    let file_a = Uuid::now_v7();
    let file_b = Uuid::now_v7();
    for (file_id, key) in [(file_a, "a.webp"), (file_b, "b.webp")] {
        infra_postgres::media::insert_file(
            &pools.app,
            tenant,
            infra_postgres::media::FileInsert {
                id: file_id,
                entity_type: "Product".to_owned(),
                entity_id: product_id,
                bucket: "media".to_owned(),
                object_key: key.to_owned(),
                mime_type: "image/webp".to_owned(),
                size_bytes: 32,
                sha256: "c".repeat(64),
                uploaded_by_user_id: admin_id,
            },
        )
        .await
        .expect("file");
    }

    let image_a = Uuid::now_v7();
    let image_b = Uuid::now_v7();
    product_images::insert_product_image(
        &pools.app,
        tenant,
        product_images::ProductImageInsert {
            id: image_a,
            product_id,
            file_id: file_a,
            sort_order: 0,
            is_primary: true,
        },
    )
    .await
    .expect("first image");
    product_images::insert_product_image(
        &pools.app,
        tenant,
        product_images::ProductImageInsert {
            id: image_b,
            product_id,
            file_id: file_b,
            sort_order: 1,
            is_primary: false,
        },
    )
    .await
    .expect("second image");

    let rows = product_images::list_product_images(&pools.app, tenant, product_id)
        .await
        .expect("list images");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].id, image_a);
    assert_eq!(rows[1].id, image_b);
    assert!(rows[0].is_primary);
    assert!(!rows[1].is_primary);
}

#[tokio::test]
async fn given_two_primary_images_when_second_insert_then_unique_violation() {
    let pools = setup_pools().await;
    let tenant = TenantId::generate();
    infra_postgres::shared::insert_tenant(&pools.admin, tenant, "Tenant")
        .await
        .expect("tenant");

    let product_id = seed_product_with_stock(&pools.app, tenant, 1).await;
    let admin_id = Uuid::now_v7();
    infra_postgres::identity::insert_user(
        &pools.app,
        tenant,
        infra_postgres::identity::InsertUserParams {
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

    let file_a = Uuid::now_v7();
    let file_b = Uuid::now_v7();
    for (file_id, key) in [(file_a, "a.webp"), (file_b, "b.webp")] {
        infra_postgres::media::insert_file(
            &pools.app,
            tenant,
            infra_postgres::media::FileInsert {
                id: file_id,
                entity_type: "Product".to_owned(),
                entity_id: product_id,
                bucket: "media".to_owned(),
                object_key: key.to_owned(),
                mime_type: "image/webp".to_owned(),
                size_bytes: 32,
                sha256: "c".repeat(64),
                uploaded_by_user_id: admin_id,
            },
        )
        .await
        .expect("file");
    }

    product_images::insert_product_image(
        &pools.app,
        tenant,
        product_images::ProductImageInsert {
            id: Uuid::now_v7(),
            product_id,
            file_id: file_a,
            sort_order: 0,
            is_primary: true,
        },
    )
    .await
    .expect("first primary");

    let result = product_images::insert_product_image(
        &pools.app,
        tenant,
        product_images::ProductImageInsert {
            id: Uuid::now_v7(),
            product_id,
            file_id: file_b,
            sort_order: 1,
            is_primary: true,
        },
    )
    .await;

    assert!(result.is_err(), "second primary must violate unique index");
}

#[tokio::test]
async fn given_product_image_when_delete_then_removed() {
    let pools = setup_pools().await;
    let tenant = TenantId::generate();
    infra_postgres::shared::insert_tenant(&pools.admin, tenant, "Tenant")
        .await
        .expect("tenant");

    let product_id = seed_product_with_stock(&pools.app, tenant, 1).await;
    let image_id = Uuid::now_v7();
    let file_id = Uuid::now_v7();
    product_images::insert_product_image(
        &pools.app,
        tenant,
        product_images::ProductImageInsert {
            id: image_id,
            product_id,
            file_id,
            sort_order: 0,
            is_primary: true,
        },
    )
    .await
    .expect("insert image");

    let deleted = product_images::delete_product_image(&pools.app, tenant, image_id)
        .await
        .expect("delete");
    assert!(deleted);

    let rows = product_images::list_product_images(&pools.app, tenant, product_id)
        .await
        .expect("list");
    assert!(rows.is_empty());
}

#[tokio::test]
async fn given_order_approved_when_reserve_release_consume_then_rn2_lifecycle() {
    let pools = setup_pools().await;
    let tenant = TenantId::generate();
    infra_postgres::shared::insert_tenant(&pools.admin, tenant, "Tenant")
        .await
        .expect("tenant");

    let product_id = seed_product_with_stock(&pools.app, tenant, 20).await;
    let order_id = Uuid::now_v7();
    let item_id = Uuid::now_v7();

    reservations::reserve_stock_for_order(
        &pools.app,
        tenant,
        &[reservations::ReservationLine {
            id: Uuid::now_v7(),
            order_id,
            order_item_id: item_id,
            product_id,
            quantity: 8,
            driver_id: None,
        }],
    )
    .await
    .expect("reserve");

    let available = reservations::tenant_available_for_product(&pools.app, tenant, product_id)
        .await
        .expect("available");
    assert_eq!(available, 12);

    let released = reservations::release_reservations(&pools.app, tenant, order_id)
        .await
        .expect("release");
    assert_eq!(released, 1);

    let available_after_release =
        reservations::tenant_available_for_product(&pools.app, tenant, product_id)
            .await
            .expect("available");
    assert_eq!(available_after_release, 20);

    reservations::reserve_stock_for_order(
        &pools.app,
        tenant,
        &[reservations::ReservationLine {
            id: Uuid::now_v7(),
            order_id,
            order_item_id: item_id,
            product_id,
            quantity: 5,
            driver_id: None,
        }],
    )
    .await
    .expect("re-reserve");

    let consumed = reservations::consume_reservations(&pools.app, tenant, order_id)
        .await
        .expect("consume");
    assert_eq!(consumed, 1);

    let rows = reservations::list_reservations_by_order(&pools.app, tenant, order_id)
        .await
        .expect("list");
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().any(|r| r.status == "Released"));
    assert!(rows.iter().any(|r| r.status == "Consumed"));
}

#[tokio::test]
async fn given_concurrent_approvals_when_total_exceeds_stock_then_one_fails() {
    let pools = setup_pools().await;
    let tenant = TenantId::generate();
    infra_postgres::shared::insert_tenant(&pools.admin, tenant, "Tenant")
        .await
        .expect("tenant");

    let product_id = seed_product_with_stock(&pools.app, tenant, 10).await;
    let pool = Arc::new(pools.app.clone());
    let barrier = Arc::new(Barrier::new(2));

    let order_a = Uuid::now_v7();
    let order_b = Uuid::now_v7();

    let task_a = {
        let pool = Arc::clone(&pool);
        let barrier = Arc::clone(&barrier);
        tokio::spawn(async move {
            barrier.wait().await;
            reservations::reserve_stock_for_order(
                &pool,
                tenant,
                &[reservations::ReservationLine {
                    id: Uuid::now_v7(),
                    order_id: order_a,
                    order_item_id: Uuid::now_v7(),
                    product_id,
                    quantity: 7,
                    driver_id: None,
                }],
            )
            .await
        })
    };

    let task_b = {
        let pool = Arc::clone(&pool);
        let barrier = Arc::clone(&barrier);
        tokio::spawn(async move {
            barrier.wait().await;
            reservations::reserve_stock_for_order(
                &pool,
                tenant,
                &[reservations::ReservationLine {
                    id: Uuid::now_v7(),
                    order_id: order_b,
                    order_item_id: Uuid::now_v7(),
                    product_id,
                    quantity: 7,
                    driver_id: None,
                }],
            )
            .await
        })
    };

    let (result_a, result_b) = tokio::join!(task_a, task_b);
    let outcomes = [result_a.expect("task a"), result_b.expect("task b")];
    let successes = outcomes.iter().filter(|r| r.is_ok()).count();
    let failures = outcomes
        .iter()
        .filter(|r| matches!(r, Err(PostgresError::InsufficientAvailableStock)))
        .count();

    assert_eq!(successes, 1, "exactly one approval must succeed");
    assert_eq!(failures, 1, "exactly one approval must fail oversell");

    let available = reservations::tenant_available_for_product(&pool, tenant, product_id)
        .await
        .expect("available");
    assert_eq!(available, 3);
}

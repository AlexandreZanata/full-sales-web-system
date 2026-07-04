//! RLS isolation integration tests — contract: tenant A cannot read tenant B rows.
//!
//! Requires Docker for testcontainers (Postgres 18+ for `uuidv7()`).

use chrono::Utc;
use domain_shared::TenantId;
use infra_postgres::commerces;
use infra_postgres::identity;
use infra_postgres::inventory;
use infra_postgres::media;
use infra_postgres::reports;
use infra_postgres::sales;
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

struct TenantFixture {
    tenant_a: TenantId,
    tenant_b: TenantId,
    user_a: Uuid,
    user_b: Uuid,
    commerce_a: Uuid,
    sale_a: Uuid,
}

async fn seed_two_tenants(admin: &PgPool, app: &PgPool) -> TenantFixture {
    let tenant_a = TenantId::generate();
    let tenant_b = TenantId::generate();
    shared::insert_tenant(admin, tenant_a, "Tenant A")
        .await
        .expect("insert tenant A");
    shared::insert_tenant(admin, tenant_b, "Tenant B")
        .await
        .expect("insert tenant B");

    let user_a = Uuid::now_v7();
    let user_b = Uuid::now_v7();
    identity::insert_user(
        app,
        tenant_a,
        user_a,
        "a@test.com",
        "User A",
        "Admin",
        "hash-a",
    )
    .await
    .expect("insert user A");
    identity::insert_user(
        app,
        tenant_b,
        user_b,
        "b@test.com",
        "User B",
        "Admin",
        "hash-b",
    )
    .await
    .expect("insert user B");

    let commerce_a = Uuid::now_v7();
    commerces::insert_commerce(
        app,
        tenant_a,
        commerce_a,
        "11222333000181",
        "Legal A",
        "Trade A",
        serde_json::json!({"city": "SP"}),
    )
    .await
    .expect("insert commerce A");

    let product_a = Uuid::now_v7();
    inventory::insert_product(app, tenant_a, product_a, "SKU-A", "Product A", 1000, "BRL")
        .await
        .expect("insert product A");

    let sale_a = Uuid::now_v7();
    sales::insert_sale(app, tenant_a, sale_a, user_a, commerce_a, "Cash")
        .await
        .expect("insert sale A");

    let public_key_id_a = "ed25519-test-a";
    reports::insert_signing_key(app, tenant_a, public_key_id_a, &[0u8; 32])
        .await
        .expect("insert signing key A");

    let report_a = Uuid::now_v7();
    let now = Utc::now();
    reports::insert_report(
        app,
        tenant_a,
        reports::NewReport {
            id: report_a,
            report_type: "DailyDriver",
            period_start: now,
            period_end: now,
            canonical_payload: r#"{"total":0}"#,
            signature: &[0u8; 64],
            public_key_id: public_key_id_a,
        },
    )
    .await
    .expect("insert report A");

    let _ = (product_a, report_a);

    TenantFixture {
        tenant_a,
        tenant_b,
        user_a,
        user_b,
        commerce_a,
        sale_a,
    }
}

#[tokio::test]
async fn rls_identity_users_isolated() {
    let pools = setup_pools().await;
    let fx = seed_two_tenants(&pools.admin, &pools.app).await;

    let visible_a = identity::list_user_ids(&pools.app, fx.tenant_a)
        .await
        .expect("list users tenant A");
    assert_eq!(visible_a.len(), 1);
    assert_eq!(visible_a[0], fx.user_a);

    let visible_b = identity::list_user_ids(&pools.app, fx.tenant_b)
        .await
        .expect("list users tenant B");
    assert_eq!(visible_b.len(), 1);
    assert_eq!(visible_b[0], fx.user_b);

    let cross = identity::find_user_by_id(&pools.app, fx.tenant_a, fx.user_b)
        .await
        .expect("find user B under tenant A context");
    assert!(cross.is_none(), "tenant A must not see tenant B user");
}

#[tokio::test]
async fn rls_commerces_isolated() {
    let pools = setup_pools().await;
    let fx = seed_two_tenants(&pools.admin, &pools.app).await;

    let ids_a = commerces::list_commerce_ids(&pools.app, fx.tenant_a)
        .await
        .expect("list commerces A");
    assert_eq!(ids_a, vec![fx.commerce_a]);

    let ids_b = commerces::list_commerce_ids(&pools.app, fx.tenant_b)
        .await
        .expect("list commerces B");
    assert!(ids_b.is_empty());
}

#[tokio::test]
async fn rls_inventory_products_isolated() {
    let pools = setup_pools().await;
    let fx = seed_two_tenants(&pools.admin, &pools.app).await;

    let ids_a = inventory::list_product_ids(&pools.app, fx.tenant_a)
        .await
        .expect("list products A");
    assert_eq!(ids_a.len(), 1);

    let ids_b = inventory::list_product_ids(&pools.app, fx.tenant_b)
        .await
        .expect("list products B");
    assert!(ids_b.is_empty());
}

#[tokio::test]
async fn rls_sales_isolated() {
    let pools = setup_pools().await;
    let fx = seed_two_tenants(&pools.admin, &pools.app).await;

    let ids_a = sales::list_sale_ids(&pools.app, fx.tenant_a)
        .await
        .expect("list sales A");
    assert_eq!(ids_a, vec![fx.sale_a]);

    let ids_b = sales::list_sale_ids(&pools.app, fx.tenant_b)
        .await
        .expect("list sales B");
    assert!(ids_b.is_empty());
}

#[tokio::test]
async fn rls_reports_isolated() {
    let pools = setup_pools().await;
    let fx = seed_two_tenants(&pools.admin, &pools.app).await;

    let ids_a = reports::list_report_ids(&pools.app, fx.tenant_a)
        .await
        .expect("list reports A");
    assert_eq!(ids_a.len(), 1);

    let ids_b = reports::list_report_ids(&pools.app, fx.tenant_b)
        .await
        .expect("list reports B");
    assert!(ids_b.is_empty());
}

#[tokio::test]
async fn rls_media_files_isolated() {
    let pools = setup_pools().await;
    let fx = seed_two_tenants(&pools.admin, &pools.app).await;

    let file_a = Uuid::now_v7();
    media::insert_file(
        &pools.app,
        fx.tenant_a,
        media::FileInsert {
            id: file_a,
            entity_type: "Product".to_owned(),
            entity_id: Uuid::now_v7(),
            bucket: "media".to_owned(),
            object_key: "tenant-a/product.webp".to_owned(),
            mime_type: "image/webp".to_owned(),
            size_bytes: 32,
            sha256: "a".repeat(64),
            uploaded_by_user_id: fx.user_a,
        },
    )
    .await
    .expect("insert file A");

    let ids_a = media::list_file_ids(&pools.app, fx.tenant_a)
        .await
        .expect("list files A");
    assert_eq!(ids_a, vec![file_a]);

    let ids_b = media::list_file_ids(&pools.app, fx.tenant_b)
        .await
        .expect("list files B");
    assert!(ids_b.is_empty());

    let cross = media::find_file_by_id(&pools.app, fx.tenant_a, file_a)
        .await
        .expect("find file under tenant A");
    assert!(cross.is_some());

    let ghost = Uuid::now_v7();
    let missing = media::find_file_by_id(&pools.app, fx.tenant_b, ghost)
        .await
        .expect("find under tenant B");
    assert!(missing.is_none());
}

#[tokio::test]
async fn rls_insert_wrong_tenant_id_rejected() {
    let pools = setup_pools().await;
    let fx = seed_two_tenants(&pools.admin, &pools.app).await;

    let mut tx = pools.app.begin().await.expect("begin tx");
    infra_postgres::rls::apply_tenant_context(&mut tx, fx.tenant_a)
        .await
        .expect("set tenant A");

    let wrong_id = Uuid::now_v7();
    let result = sqlx::query(
        "INSERT INTO identity.users (id, tenant_id, email, name, role, password_hash)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(wrong_id)
    .bind(fx.tenant_b.as_uuid())
    .bind("wrong@test.com")
    .bind("Wrong User")
    .bind("Admin")
    .bind("hash")
    .execute(&mut *tx)
    .await;

    assert!(
        result.is_err(),
        "insert with mismatched tenant_id must fail RLS"
    );
}

//! Commerce address integration tests — Phase 09.

use domain_shared::TenantId;
use infra_postgres::commerces;
use infra_postgres::commerces::addresses;
use infra_postgres::media;
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

async fn seed_commerce(app: &PgPool, tenant: TenantId) -> Uuid {
    let commerce_id = Uuid::now_v7();
    commerces::insert_commerce(
        app,
        tenant,
        commerce_id,
        "11222333000181",
        "Acme Ltda",
        "Acme",
        serde_json::json!({
            "street": "Legacy St",
            "city": "São Paulo",
            "state": "SP",
            "postalCode": "01310100"
        }),
    )
    .await
    .expect("commerce");
    commerce_id
}

#[tokio::test]
async fn given_delivery_address_when_insert_then_persisted() {
    let pools = setup_pools().await;
    let tenant = TenantId::generate();
    infra_postgres::shared::insert_tenant(&pools.admin, tenant, "Tenant")
        .await
        .expect("tenant");

    let commerce_id = seed_commerce(&pools.app, tenant).await;
    let address_id = Uuid::now_v7();

    addresses::insert_address(
        &pools.app,
        tenant,
        addresses::AddressInsert {
            id: address_id,
            commerce_id,
            address_type: "Delivery".into(),
            street: "Av Paulista".into(),
            number: "1000".into(),
            district: Some("Bela Vista".into()),
            city: "São Paulo".into(),
            state: "SP".into(),
            postal_code: "01310100".into(),
            latitude: None,
            longitude: None,
            is_primary: true,
        },
    )
    .await
    .expect("insert address");

    let row = addresses::find_address_by_id(&pools.app, tenant, address_id)
        .await
        .expect("find")
        .expect("exists");
    assert_eq!(row.address_type, "Delivery");
    assert!(row.is_primary);
}

#[tokio::test]
async fn given_two_primary_delivery_when_second_insert_then_unique_violation() {
    let pools = setup_pools().await;
    let tenant = TenantId::generate();
    infra_postgres::shared::insert_tenant(&pools.admin, tenant, "Tenant")
        .await
        .expect("tenant");

    let commerce_id = seed_commerce(&pools.app, tenant).await;

    addresses::insert_address(
        &pools.app,
        tenant,
        addresses::AddressInsert {
            id: Uuid::now_v7(),
            commerce_id,
            address_type: "Delivery".into(),
            street: "Rua A".into(),
            number: "1".into(),
            district: None,
            city: "São Paulo".into(),
            state: "SP".into(),
            postal_code: "01310100".into(),
            latitude: None,
            longitude: None,
            is_primary: true,
        },
    )
    .await
    .expect("first primary");

    let result = addresses::insert_address(
        &pools.app,
        tenant,
        addresses::AddressInsert {
            id: Uuid::now_v7(),
            commerce_id,
            address_type: "Delivery".into(),
            street: "Rua B".into(),
            number: "2".into(),
            district: None,
            city: "São Paulo".into(),
            state: "SP".into(),
            postal_code: "01310101".into(),
            latitude: None,
            longitude: None,
            is_primary: true,
        },
    )
    .await;

    assert!(result.is_err(), "second primary must violate unique index");
}

#[tokio::test]
async fn given_logo_file_when_update_commerce_then_logo_file_id_set() {
    let pools = setup_pools().await;
    let tenant = TenantId::generate();
    infra_postgres::shared::insert_tenant(&pools.admin, tenant, "Tenant")
        .await
        .expect("tenant");

    let commerce_id = seed_commerce(&pools.app, tenant).await;
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

    let file_id = Uuid::now_v7();
    media::insert_file(
        &pools.app,
        tenant,
        media::FileInsert {
            id: file_id,
            entity_type: "Commerce".to_owned(),
            entity_id: commerce_id,
            bucket: "media".to_owned(),
            object_key: "commerces/logo.webp".to_owned(),
            mime_type: "image/webp".to_owned(),
            size_bytes: 32,
            sha256: "a".repeat(64),
            uploaded_by_user_id: admin_id,
        },
    )
    .await
    .expect("media file");

    addresses::update_commerce_logo(&pools.app, tenant, commerce_id, Some(file_id))
        .await
        .expect("update logo");

    let mut tx = pools.app.begin().await.expect("begin");
    infra_postgres::rls::apply_tenant_context(&mut tx, tenant)
        .await
        .expect("rls");
    let logo: Option<Uuid> =
        sqlx::query_scalar("SELECT logo_file_id FROM commerces.commerces WHERE id = $1")
            .bind(commerce_id)
            .fetch_one(&mut *tx)
            .await
            .expect("query");
    tx.commit().await.expect("commit");
    assert_eq!(logo, Some(file_id));
}

#[tokio::test]
async fn given_tenant_b_when_read_tenant_a_address_then_empty() {
    let pools = setup_pools().await;
    let tenant_a = TenantId::generate();
    let tenant_b = TenantId::generate();
    infra_postgres::shared::insert_tenant(&pools.admin, tenant_a, "Tenant A")
        .await
        .expect("tenant A");
    infra_postgres::shared::insert_tenant(&pools.admin, tenant_b, "Tenant B")
        .await
        .expect("tenant B");

    let commerce_id = seed_commerce(&pools.app, tenant_a).await;
    let address_id = Uuid::now_v7();
    addresses::insert_address(
        &pools.app,
        tenant_a,
        addresses::AddressInsert {
            id: address_id,
            commerce_id,
            address_type: "Delivery".into(),
            street: "Rua A".into(),
            number: "1".into(),
            district: None,
            city: "São Paulo".into(),
            state: "SP".into(),
            postal_code: "01310100".into(),
            latitude: None,
            longitude: None,
            is_primary: false,
        },
    )
    .await
    .expect("insert");

    let cross = addresses::find_address_by_id(&pools.app, tenant_b, address_id)
        .await
        .expect("find");
    assert!(cross.is_none(), "RLS must hide other tenant addresses");
}

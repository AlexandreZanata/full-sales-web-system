//! Identity profile integration tests — Phase 08.

use domain_shared::TenantId;
use infra_postgres::commerces;
use infra_postgres::identity;
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

#[tokio::test]
async fn given_commerce_contact_without_commerce_id_when_insert_then_check_fails() {
    let pools = setup_pools().await;
    let tenant = TenantId::generate();
    infra_postgres::shared::insert_tenant(&pools.admin, tenant, "Tenant")
        .await
        .expect("tenant");

    let mut tx = pools.app.begin().await.expect("begin");
    infra_postgres::rls::apply_tenant_context(&mut tx, tenant)
        .await
        .expect("rls");

    let result = sqlx::query(
        "INSERT INTO identity.users
         (id, tenant_id, email, name, role, password_hash, commerce_id)
         VALUES ($1, $2, $3, $4, $5, $6, NULL)",
    )
    .bind(Uuid::now_v7())
    .bind(tenant.as_uuid())
    .bind("portal@store.com")
    .bind("Portal User")
    .bind("CommerceContact")
    .bind("hash")
    .execute(&mut *tx)
    .await;

    assert!(
        result.is_err(),
        "CommerceContact without commerce_id must fail CHECK"
    );
}

#[tokio::test]
async fn given_driver_profile_when_cnh_photo_file_id_then_references_media_files() {
    let pools = setup_pools().await;
    let tenant = TenantId::generate();
    infra_postgres::shared::insert_tenant(&pools.admin, tenant, "Tenant")
        .await
        .expect("tenant");

    let driver_id = Uuid::now_v7();
    identity::insert_user(
        &pools.app,
        tenant,
        identity::InsertUserParams {
            id: driver_id,
            email: "driver@test.com",
            name: "Driver One",
            role: "Driver",
            password_hash: "hash",
            commerce_id: None,
            profile_file_id: None,
        },
    )
    .await
    .expect("driver user");

    let file_id = Uuid::now_v7();
    media::insert_file(
        &pools.app,
        tenant,
        media::FileInsert {
            id: file_id,
            entity_type: "User".to_owned(),
            entity_id: driver_id,
            bucket: "media".to_owned(),
            object_key: "drivers/cnh.webp".to_owned(),
            mime_type: "image/webp".to_owned(),
            size_bytes: 32,
            sha256: "b".repeat(64),
            uploaded_by_user_id: driver_id,
        },
    )
    .await
    .expect("cnh photo file");

    identity::insert_driver_profile(
        &pools.app,
        tenant,
        identity::DriverProfileInsert {
            user_id: driver_id,
            cnh_number: "12345678900".to_owned(),
            cnh_category: "B".to_owned(),
            cnh_photo_file_id: Some(file_id),
            vehicle_plate: "ABC1D23".to_owned(),
            vehicle_model: "Fiorino".to_owned(),
            vehicle_capacity_kg: Some(800.0),
        },
    )
    .await
    .expect("driver profile");

    let profile = identity::find_driver_profile_by_user_id(&pools.app, tenant, driver_id)
        .await
        .expect("find profile")
        .expect("profile exists");

    assert_eq!(profile.cnh_photo_file_id, Some(file_id));
}

#[tokio::test]
async fn given_invalid_cnh_photo_file_id_when_insert_driver_profile_then_fk_fails() {
    let pools = setup_pools().await;
    let tenant = TenantId::generate();
    infra_postgres::shared::insert_tenant(&pools.admin, tenant, "Tenant")
        .await
        .expect("tenant");

    let driver_id = Uuid::now_v7();
    identity::insert_user(
        &pools.app,
        tenant,
        identity::InsertUserParams {
            id: driver_id,
            email: "driver2@test.com",
            name: "Driver Two",
            role: "Driver",
            password_hash: "hash",
            commerce_id: None,
            profile_file_id: None,
        },
    )
    .await
    .expect("driver user");

    let result = identity::insert_driver_profile(
        &pools.app,
        tenant,
        identity::DriverProfileInsert {
            user_id: driver_id,
            cnh_number: "98765432100".to_owned(),
            cnh_category: "C".to_owned(),
            cnh_photo_file_id: Some(Uuid::now_v7()),
            vehicle_plate: "XYZ9A87".to_owned(),
            vehicle_model: "Sprinter".to_owned(),
            vehicle_capacity_kg: None,
        },
    )
    .await;

    assert!(result.is_err(), "missing media.files row must fail FK");
}

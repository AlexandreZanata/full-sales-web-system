//! Media tenant entity type — Phase 41.

use domain_shared::TenantId;
use infra_postgres::media::{self, FileInsert};
use infra_postgres::{PgPool, migrate};
use testcontainers::ImageExt;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;
use uuid::Uuid;

struct TestPools {
    _admin: PgPool,
    app: PgPool,
    _container: testcontainers::ContainerAsync<Postgres>,
}

async fn setup_pools() -> (TestPools, TenantId) {
    let container = Postgres::default()
        .with_tag("18-alpine")
        .start()
        .await
        .expect("start postgres");

    let host = container.get_host().await.expect("host");
    let port = container.get_host_port_ipv4(5432).await.expect("port");

    let admin_url = format!("postgres://postgres:postgres@{host}:{port}/postgres");
    let app_url = format!("postgres://app_user:app_password@{host}:{port}/postgres");

    let admin = infra_postgres::connect(&admin_url).await.expect("admin");
    migrate(&admin).await.expect("migrate");
    let app = infra_postgres::connect(&app_url).await.expect("app");

    let tenant = TenantId::generate();
    infra_postgres::shared::insert_tenant(&admin, tenant, "Tenant")
        .await
        .expect("tenant");

    (
        TestPools {
            _admin: admin,
            app,
            _container: container,
        },
        tenant,
    )
}

#[tokio::test]
async fn given_tenant_entity_type_when_insert_file_then_persisted() {
    let (pools, tenant) = setup_pools().await;
    let file_id = Uuid::now_v7();
    let user_id = Uuid::now_v7();

    media::insert_file(
        &pools.app,
        tenant,
        FileInsert {
            id: file_id,
            entity_type: "Tenant".into(),
            entity_id: tenant.as_uuid(),
            bucket: "media".into(),
            object_key: "tenant/logo.webp".into(),
            mime_type: "image/webp".into(),
            size_bytes: 32,
            sha256: "c".repeat(64),
            uploaded_by_user_id: user_id,
        },
    )
    .await
    .expect("insert tenant file");

    let row = media::find_file_by_id(&pools.app, tenant, file_id)
        .await
        .expect("find")
        .expect("row");
    assert_eq!(row.entity_type, "Tenant");
}

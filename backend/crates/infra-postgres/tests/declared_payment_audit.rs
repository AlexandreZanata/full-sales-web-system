//! RN-PAG3 — declared payment changes append audit.events.

use chrono::Utc;
use domain_shared::TenantId;
use infra_postgres::audit;
use infra_postgres::identity;
use infra_postgres::sales::{self, DeclarePaymentUpdate};
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

struct SaleFixture {
    tenant: TenantId,
    driver_session: SessionContext,
    other_driver_session: SessionContext,
    sale_id: Uuid,
}

async fn seed_field_sale(pools: &TestPools) -> SaleFixture {
    let tenant = TenantId::generate();
    infra_postgres::shared::insert_tenant(&pools.admin, tenant, "Tenant")
        .await
        .expect("tenant");

    let driver_id = Uuid::now_v7();
    let other_driver_id = Uuid::now_v7();
    let commerce_id = Uuid::now_v7();
    let sale_id = Uuid::now_v7();

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

    identity::insert_user(
        &pools.app,
        tenant,
        identity::InsertUserParams {
            id: other_driver_id,
            email: "other@test.com",
            name: "Other",
            role: "Driver",
            password_hash: "hash",
            commerce_id: None,
            profile_file_id: None,
        },
    )
    .await
    .expect("other");

    infra_postgres::commerces::insert_commerce(
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

    sales::insert_sale(&pools.app, tenant, sale_id, driver_id, commerce_id, "Cash")
        .await
        .expect("sale");

    SaleFixture {
        tenant,
        driver_session: SessionContext {
            tenant_id: tenant,
            role: "Driver".into(),
            user_id: driver_id,
            commerce_id: None,
        },
        other_driver_session: SessionContext {
            tenant_id: tenant,
            role: "Driver".into(),
            user_id: other_driver_id,
            commerce_id: None,
        },
        sale_id,
    }
}

#[tokio::test]
async fn given_driver_when_declare_payment_then_audit_event_appended() {
    let pools = setup_pools().await;
    let fixture = seed_field_sale(&pools).await;
    let before = audit::list_audit_event_ids(&pools.app, fixture.tenant)
        .await
        .expect("audit before");

    sales::declare_payment(
        &pools.app,
        &fixture.driver_session,
        &DeclarePaymentUpdate {
            sale_id: fixture.sale_id,
            driver_id: fixture.driver_session.user_id,
            method: "Pix".into(),
            received: true,
            declared_at: Utc::now(),
            notes: Some("received".into()),
        },
    )
    .await
    .expect("declare");

    let after = audit::list_audit_event_ids(&pools.app, fixture.tenant)
        .await
        .expect("audit after");
    assert_eq!(after.len(), before.len() + 1);

    let sale = sales::find_sale_by_id(&pools.app, fixture.tenant, fixture.sale_id)
        .await
        .expect("sale")
        .expect("row");
    assert_eq!(sale.declared_payment_method, "Pix");
    assert!(sale.declared_payment_received);
}

#[tokio::test]
async fn given_other_driver_when_declare_payment_then_not_found() {
    let pools = setup_pools().await;
    let fixture = seed_field_sale(&pools).await;

    let err = sales::declare_payment(
        &pools.app,
        &fixture.other_driver_session,
        &DeclarePaymentUpdate {
            sale_id: fixture.sale_id,
            driver_id: fixture.other_driver_session.user_id,
            method: "Cash".into(),
            received: true,
            declared_at: Utc::now(),
            notes: None,
        },
    )
    .await
    .expect_err("must fail");

    assert!(matches!(
        err,
        PostgresError::Database(sqlx::Error::RowNotFound)
    ));
}

//! E2E-001 — UC-001 happy path: login → create sale → confirm → stock reduced.

use std::sync::Arc;
use std::time::Duration;

use api_http::{AppState, full_app};
use axum::body::Body;
use http::{Request, StatusCode};
use infra_crypto::{JwtService, PasswordHasher};
use infra_postgres::{PgPool, migrate};
use infra_redis::InMemoryRefreshTokenStore;
use serde_json::json;
use testcontainers::ImageExt;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;
use tower::ServiceExt;
use uuid::Uuid;

struct TestEnv {
    state: AppState,
    tenant_id: domain_shared::TenantId,
    driver_id: Uuid,
    driver_token: String,
    commerce_id: Uuid,
    product_id: Uuid,
    initial_stock: i32,
    _container: testcontainers::ContainerAsync<Postgres>,
}

async fn setup() -> TestEnv {
    let container = Postgres::default()
        .with_tag("18-alpine")
        .start()
        .await
        .expect("start postgres");

    let host = container.get_host().await.expect("host");
    let port = container.get_host_port_ipv4(5432).await.expect("port");

    let admin_url = format!("postgres://postgres:postgres@{host}:{port}/postgres");
    let app_url = format!("postgres://app_user:app_password@{host}:{port}/postgres");

    let admin_pool = infra_postgres::connect(&admin_url)
        .await
        .expect("admin pool");
    migrate(&admin_pool).await.expect("migrate");

    let app_pool = infra_postgres::connect(&app_url).await.expect("app pool");

    let tenant_id = domain_shared::TenantId::generate();
    infra_postgres::shared::insert_tenant(&admin_pool, tenant_id, "E2E Tenant")
        .await
        .expect("tenant");

    let state = AppState {
        admin_pool: admin_pool.clone(),
        app_pool: app_pool.clone(),
        refresh_store: Arc::new(InMemoryRefreshTokenStore::new()),
        idempotency_store: AppState::in_memory_idempotency(),
        jwt: JwtService::new("test-secret", Duration::from_secs(900)),
        refresh_ttl: application::REFRESH_TOKEN_TTL,
    };

    let driver_id = seed_user(&app_pool, tenant_id, "driver@e2e.test", "Driver").await;
    let driver_token = login(&state, "driver@e2e.test", "password").await;

    let commerce_id = Uuid::now_v7();
    infra_postgres::commerces::insert_commerce(
        &app_pool,
        tenant_id,
        commerce_id,
        "11222333000181",
        "E2E Commerce Ltd",
        "E2E Shop",
        json!({"street": "Rua E2E"}),
    )
    .await
    .expect("commerce");

    let product_id = Uuid::now_v7();
    infra_postgres::inventory::insert_product(
        &app_pool,
        tenant_id,
        product_id,
        "E2E-SKU",
        "E2E Widget",
        1_000,
        "BRL",
    )
    .await
    .expect("product");

    let initial_stock = 10;
    infra_postgres::inventory::upsert_stock_balance(
        &app_pool,
        tenant_id,
        driver_id,
        product_id,
        initial_stock,
    )
    .await
    .expect("stock");

    TestEnv {
        state,
        tenant_id,
        driver_id,
        driver_token,
        commerce_id,
        product_id,
        initial_stock,
        _container: container,
    }
}

async fn seed_user(
    pool: &PgPool,
    tenant_id: domain_shared::TenantId,
    email: &str,
    role: &str,
) -> Uuid {
    let id = Uuid::now_v7();
    let hash = PasswordHasher::hash("password").expect("hash");
    infra_postgres::identity::insert_user(pool, tenant_id, id, email, "E2E User", role, &hash)
        .await
        .expect("user");
    id
}

async fn login(state: &AppState, email: &str, password: &str) -> String {
    let response = full_app(state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"email": email, "password": password}).to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let json: serde_json::Value = serde_json::from_slice(&body).expect("json");
    json["accessToken"].as_str().expect("token").to_owned()
}

// E2E-001 / UC-001: login → create sale → confirm → balance reduced
#[tokio::test]
async fn sale_happy_path() {
    let env = setup().await;
    let sold_qty = 2;

    let stock_before = infra_postgres::inventory::get_stock_quantity(
        &env.state.app_pool,
        env.tenant_id,
        env.driver_id,
        env.product_id,
    )
    .await
    .expect("stock before");
    assert_eq!(stock_before, Some(env.initial_stock));

    let create_body = json!({
        "commerceId": env.commerce_id,
        "items": [{ "productId": env.product_id, "quantity": sold_qty }],
        "paymentMethod": "cash"
    });

    let create = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/sales")
                .header("authorization", format!("Bearer {}", env.driver_token))
                .header("content-type", "application/json")
                .body(Body::from(create_body.to_string()))
                .expect("request"),
        )
        .await
        .expect("create response");
    assert_eq!(create.status(), StatusCode::CREATED);

    let create_bytes = axum::body::to_bytes(create.into_body(), usize::MAX)
        .await
        .expect("create bytes");
    let created: serde_json::Value = serde_json::from_slice(&create_bytes).expect("create json");
    assert_eq!(created["status"], "Pending");
    assert_eq!(created["totalAmount"], sold_qty as i64 * 1_000);
    let sale_id = created["id"].as_str().expect("sale id");

    let confirm = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/sales/{sale_id}/confirm"))
                .header("authorization", format!("Bearer {}", env.driver_token))
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("confirm response");
    assert_eq!(confirm.status(), StatusCode::OK);

    let confirm_bytes = axum::body::to_bytes(confirm.into_body(), usize::MAX)
        .await
        .expect("confirm bytes");
    let confirmed: serde_json::Value =
        serde_json::from_slice(&confirm_bytes).expect("confirm json");
    assert_eq!(confirmed["status"], "Confirmed");
    assert_eq!(confirmed["totalAmount"], sold_qty as i64 * 1_000);

    let stock_after = infra_postgres::inventory::get_stock_quantity(
        &env.state.app_pool,
        env.tenant_id,
        env.driver_id,
        env.product_id,
    )
    .await
    .expect("stock after");
    assert_eq!(
        stock_after,
        Some(env.initial_stock - sold_qty),
        "stock must decrease by sold quantity after confirm"
    );
}

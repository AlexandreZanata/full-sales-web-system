//! Contract: GET /v1/public/catalog/events SSE + admin product mutations.

use std::sync::Arc;
use std::time::Duration;

use api_http::catalog_events::{CATALOG_SSE_EVENT, CatalogEventHub, notify_product_changed};
use api_http::{AppState, full_app};
use axum::body::Body;
use http::{Request, StatusCode};
use infra_crypto::{JwtService, PasswordHasher};
use infra_postgres::migrate;
use infra_redis::InMemoryRefreshTokenStore;
use infra_storage::InMemoryObjectStorage;
use serde_json::json;
use testcontainers::ImageExt;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;
use tower::ServiceExt;
use uuid::Uuid;

struct SseEnv {
    state: AppState,
    admin_token: String,
    product_id: String,
    _container: testcontainers::ContainerAsync<Postgres>,
}

async fn setup() -> SseEnv {
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
    infra_postgres::shared::insert_tenant(&admin_pool, tenant_id, "SSE Tenant")
        .await
        .expect("tenant");

    let (refresh_store, platform_refresh_store) = AppState::in_memory_refresh_stores();
    let state = AppState {
        admin_pool: admin_pool.clone(),
        app_pool: app_pool.clone(),
        refresh_store,
        platform_refresh_store,
        idempotency_store: AppState::in_memory_idempotency(),
        rate_limiter: AppState::in_memory_rate_limiter(),
        login_rate_limit: AppState::default_login_rate_limit(),
        verify_rate_limit: AppState::default_verify_rate_limit(),
        jwt: JwtService::new("test-secret", Duration::from_secs(900)),
        refresh_ttl: application::REFRESH_TOKEN_TTL,
        storage: Arc::new(InMemoryObjectStorage::new()),
        report_signing_key: None,
        catalog_events: AppState::default_catalog_events(),
        cnpj_lookup_rate_limit: AppState::default_cnpj_lookup_rate_limit(),
        cnpj_lookup: AppState::mock_cnpj_lookup(),
        cnpj_miss_cache: AppState::in_memory_cnpj_miss_cache(),
        payment_gateway: AppState::mock_payment_gateway(),
        asaas_webhook_token: None,
    };

    let admin_id = Uuid::now_v7();
    let hash = PasswordHasher::hash("secret123").expect("hash");
    infra_postgres::identity::insert_user(
        &app_pool,
        tenant_id,
        infra_postgres::identity::InsertUserParams {
            id: admin_id,
            email: "admin@test.com",
            name: "Admin",
            role: "Admin",
            password_hash: &hash,
            commerce_id: None,
            profile_file_id: None,
        },
    )
    .await
    .expect("admin");

    let login = full_app(state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"email": "admin@test.com", "password": "secret123"}).to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(login.status(), StatusCode::OK);
    let body = axum::body::to_bytes(login.into_body(), usize::MAX)
        .await
        .expect("body");
    let json: serde_json::Value = serde_json::from_slice(&body).expect("json");
    let admin_token = json["accessToken"].as_str().expect("token").to_owned();

    let create = full_app(state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/products")
                .header("authorization", format!("Bearer {admin_token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "name": "SSE Widget",
                        "sku": "SSE-001",
                        "priceAmount": 1000,
                        "priceCurrency": "BRL"
                    })
                    .to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("create");
    assert_eq!(create.status(), StatusCode::CREATED);
    let create_body = axum::body::to_bytes(create.into_body(), usize::MAX)
        .await
        .expect("body");
    let created: serde_json::Value = serde_json::from_slice(&create_body).expect("json");
    let product_id = created["id"].as_str().expect("id").to_owned();

    SseEnv {
        state,
        admin_token,
        product_id,
        _container: container,
    }
}

#[tokio::test]
async fn contract_catalog_events_route_when_get_then_text_event_stream() {
    let env = setup().await;

    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/public/catalog/events")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);
    assert!(
        response
            .headers()
            .get("content-type")
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| value.contains("text/event-stream"))
    );
}

#[tokio::test]
async fn contract_catalog_events_when_product_updated_then_hub_receives_payload() {
    let env = setup().await;
    let product_id = Uuid::parse_str(&env.product_id).expect("uuid");
    let mut receiver = env.state.catalog_events.subscribe();

    let update = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!("/v1/products/{}", env.product_id))
                .header("authorization", format!("Bearer {}", env.admin_token))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"name": "SSE Widget Updated"}).to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("update");
    assert_eq!(update.status(), StatusCode::OK);

    let payload = receiver.recv().await.expect("event");
    assert!(payload.contains(CATALOG_SSE_EVENT));
    assert!(payload.contains("\"resource\":\"product\""));
    assert!(payload.contains("\"action\":\"updated\""));
    assert!(payload.contains("\"sku\":\"SSE-001\""));
    assert!(payload.contains(&format!("\"id\":\"{product_id}\"")));
}

#[test]
fn catalog_event_hub_when_publish_product_then_serializes_event() {
    let hub = CatalogEventHub::new();
    let mut receiver = hub.subscribe();
    let product_id = Uuid::now_v7();

    notify_product_changed(&hub, "updated", product_id, "SKU-42");

    let payload = receiver.try_recv().expect("event");
    assert!(payload.contains(CATALOG_SSE_EVENT));
    assert!(payload.contains("\"sku\":\"SKU-42\""));
}

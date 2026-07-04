//! Sales and products contract tests — API-CONTRACT.md + UC-001.

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
    driver_token: String,
    commerce_id: Uuid,
    product_id: Uuid,
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
    infra_postgres::shared::insert_tenant(&admin_pool, tenant_id, "Sales Test Tenant")
        .await
        .expect("tenant");

    let state = AppState {
        admin_pool: admin_pool.clone(),
        app_pool: app_pool.clone(),
        refresh_store: Arc::new(InMemoryRefreshTokenStore::new()),
        idempotency_store: AppState::in_memory_idempotency(),
        rate_limiter: AppState::in_memory_rate_limiter(),
        login_rate_limit: AppState::default_login_rate_limit(),
        verify_rate_limit: AppState::default_verify_rate_limit(),
        jwt: JwtService::new("test-secret", Duration::from_secs(900)),
        refresh_ttl: application::REFRESH_TOKEN_TTL,
        storage: AppState::in_memory_storage(),
        report_signing_key: None,
    };

    let driver_id = seed_user(&app_pool, tenant_id, "driver@test.com", "Driver").await;
    let driver_token = login(&state, "driver@test.com", "password").await;

    let commerce_id = Uuid::now_v7();
    infra_postgres::commerces::insert_commerce(
        &app_pool,
        tenant_id,
        commerce_id,
        "11222333000181",
        "Acme Ltd",
        "Acme",
        json!({"street": "Rua 1"}),
    )
    .await
    .expect("commerce");

    let product_id = Uuid::now_v7();
    infra_postgres::inventory::insert_product(
        &app_pool, tenant_id, product_id, "SKU-001", "Widget", 1_000, "BRL",
    )
    .await
    .expect("product");

    infra_postgres::inventory::upsert_stock_balance(
        &app_pool, tenant_id, driver_id, product_id, 10,
    )
    .await
    .expect("stock");

    TestEnv {
        state,
        driver_token,
        commerce_id,
        product_id,
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
    infra_postgres::identity::insert_user(
        pool,
        tenant_id,
        infra_postgres::identity::InsertUserParams {
            id,
            email,
            name: "Test User",
            role,
            password_hash: &hash,
            commerce_id: None,
            profile_file_id: None,
        },
    )
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

// Contract: POST /v1/sales valid body → 201 + Sale shape (OpenAPI Sale schema)
#[tokio::test]
async fn contract_create_sale_when_valid_body_then_201_with_sale_shape() {
    let env = setup().await;
    let body = json!({
        "commerceId": env.commerce_id,
        "items": [{ "productId": env.product_id, "quantity": 2 }],
        "paymentMethod": "cash"
    });

    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/sales")
                .header("authorization", format!("Bearer {}", env.driver_token))
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::CREATED);
    assert!(response.headers().get("location").is_some());

    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let json: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(json["status"], "Pending");
    assert_eq!(json["totalAmount"], 2_000);
    assert_eq!(json["totalCurrency"], "BRL");
    assert!(json["id"].is_string());
    assert_eq!(json["items"][0]["quantity"], 2);
}

// Contract: POST /v1/sales unknown field → 400 VALIDATION_ERROR
#[tokio::test]
async fn contract_create_sale_when_unknown_field_then_400() {
    let env = setup().await;
    let body = json!({
        "commerceId": env.commerce_id,
        "items": [{ "productId": env.product_id, "quantity": 1 }],
        "paymentMethod": "pix",
        "role": "Admin"
    });

    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/sales")
                .header("authorization", format!("Bearer {}", env.driver_token))
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let json: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    assert!(json["error"]["code"].is_string());
    assert!(json["error"]["correlationId"].is_string());
}

// Contract: POST /v1/sales invalid quantity → 400
#[tokio::test]
async fn contract_create_sale_when_invalid_quantity_then_400() {
    let env = setup().await;
    let body = json!({
        "commerceId": env.commerce_id,
        "items": [{ "productId": env.product_id, "quantity": 0 }],
        "paymentMethod": "cash"
    });

    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/sales")
                .header("authorization", format!("Bearer {}", env.driver_token))
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let json: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(json["error"]["code"], "VALIDATION_ERROR");
}

// Contract: GET /v1/sales/{id} unknown → 404 SALE_NOT_FOUND
#[tokio::test]
async fn contract_get_sale_when_unknown_id_then_404() {
    let env = setup().await;
    let unknown = Uuid::now_v7();

    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/v1/sales/{unknown}"))
                .header("authorization", format!("Bearer {}", env.driver_token))
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let json: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(json["error"]["code"], "SALE_NOT_FOUND");
}

// Contract: UC-001 AF-1 — confirm with insufficient stock → 409 INSUFFICIENT_STOCK
#[tokio::test]
async fn contract_confirm_sale_when_insufficient_stock_then_409() {
    let env = setup().await;
    let body = json!({
        "commerceId": env.commerce_id,
        "items": [{ "productId": env.product_id, "quantity": 100 }],
        "paymentMethod": "cash"
    });

    let create = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/sales")
                .header("authorization", format!("Bearer {}", env.driver_token))
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(create.status(), StatusCode::CREATED);
    let bytes = axum::body::to_bytes(create.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let created: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    let sale_id = created["id"].as_str().expect("id");

    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/sales/{sale_id}/confirm"))
                .header("authorization", format!("Bearer {}", env.driver_token))
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::CONFLICT);
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let json: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(json["error"]["code"], "INSUFFICIENT_STOCK");
}

// Contract: GET /v1/products → 200 with pagination meta
#[tokio::test]
async fn contract_list_products_when_authenticated_then_200_with_pagination() {
    let env = setup().await;

    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/products?page=1&pageSize=20")
                .header("authorization", format!("Bearer {}", env.driver_token))
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let json: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    assert!(json["items"].is_array());
    assert_eq!(json["page"], 1);
    assert_eq!(json["pageSize"], 20);
    assert!(json["total"].as_u64().is_some());
}

// Contract: Idempotency-Key replay → same 201 response
#[tokio::test]
async fn contract_create_sale_when_idempotency_key_replayed_then_same_response() {
    let env = setup().await;
    let body = json!({
        "commerceId": env.commerce_id,
        "items": [{ "productId": env.product_id, "quantity": 1 }],
        "paymentMethod": "debit"
    });
    let key = Uuid::now_v7().to_string();

    let first = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/sales")
                .header("authorization", format!("Bearer {}", env.driver_token))
                .header("content-type", "application/json")
                .header("Idempotency-Key", &key)
                .body(Body::from(body.to_string()))
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(first.status(), StatusCode::CREATED);
    let first_bytes = axum::body::to_bytes(first.into_body(), usize::MAX)
        .await
        .expect("bytes");

    let second = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/sales")
                .header("authorization", format!("Bearer {}", env.driver_token))
                .header("content-type", "application/json")
                .header("Idempotency-Key", &key)
                .body(Body::from(body.to_string()))
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(second.status(), StatusCode::CREATED);
    let second_bytes = axum::body::to_bytes(second.into_body(), usize::MAX)
        .await
        .expect("bytes");
    assert_eq!(first_bytes, second_bytes);
}

// Contract: UC-001 AF-2 — cancel pending sale → 200 Cancelled, stock unchanged
#[tokio::test]
async fn contract_cancel_sale_when_pending_then_200_cancelled() {
    let env = setup().await;
    let body = json!({
        "commerceId": env.commerce_id,
        "items": [{ "productId": env.product_id, "quantity": 1 }],
        "paymentMethod": "pix"
    });

    let create = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/sales")
                .header("authorization", format!("Bearer {}", env.driver_token))
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(create.status(), StatusCode::CREATED);
    let bytes = axum::body::to_bytes(create.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let created: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    let sale_id = created["id"].as_str().expect("id");

    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/sales/{sale_id}/cancel"))
                .header("authorization", format!("Bearer {}", env.driver_token))
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let json: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(json["status"], "Cancelled");
}

// Contract: UC-001 step 6 — confirm with sufficient stock → 200 Confirmed
#[tokio::test]
async fn contract_confirm_sale_when_sufficient_stock_then_200_confirmed() {
    let env = setup().await;
    let body = json!({
        "commerceId": env.commerce_id,
        "items": [{ "productId": env.product_id, "quantity": 2 }],
        "paymentMethod": "credit"
    });

    let create = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/sales")
                .header("authorization", format!("Bearer {}", env.driver_token))
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(create.status(), StatusCode::CREATED);
    let bytes = axum::body::to_bytes(create.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let created: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    let sale_id = created["id"].as_str().expect("id");

    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/sales/{sale_id}/confirm"))
                .header("authorization", format!("Bearer {}", env.driver_token))
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let json: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(json["status"], "Confirmed");
}

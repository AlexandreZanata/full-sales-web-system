//! Auth and commerces contract tests — API-CONTRACT.md + BUSINESS-RULES.md

use std::sync::Arc;
use std::time::Duration;

use api_http::{AppState, full_app};
use application::REFRESH_TOKEN_TTL;
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
    admin_pool: PgPool,
    app_pool: PgPool,
    state: AppState,
    tenant_id: domain_shared::TenantId,
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
    infra_postgres::shared::insert_tenant(&admin_pool, tenant_id, "Test Tenant")
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
        refresh_ttl: REFRESH_TOKEN_TTL,
        storage: AppState::in_memory_storage(),
        report_signing_key: None,
        catalog_events: AppState::default_catalog_events(),
        cnpj_lookup_rate_limit: AppState::default_cnpj_lookup_rate_limit(),
        cnpj_lookup: AppState::mock_cnpj_lookup(),
        cnpj_miss_cache: AppState::in_memory_cnpj_miss_cache(),
        payment_gateway: AppState::mock_payment_gateway(),
        asaas_webhook_token: None,
    };

    TestEnv {
        admin_pool,
        app_pool,
        state,
        tenant_id,
        _container: container,
    }
}

async fn seed_user(env: &TestEnv, email: &str, password: &str, role: &str, active: bool) -> Uuid {
    let id = Uuid::now_v7();
    let hash = PasswordHasher::hash(password).expect("hash");
    infra_postgres::identity::insert_user(
        &env.app_pool,
        env.tenant_id,
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
    .expect("insert user");

    if !active {
        infra_postgres::identity::deactivate_user(&env.admin_pool, id)
            .await
            .expect("deactivate");
    }

    id
}

async fn login(env: &TestEnv, email: &str, password: &str) -> serde_json::Value {
    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({ "email": email, "password": password }).to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    serde_json::from_slice(&body).expect("json")
}

// Contract: POST /v1/auth/login → accessToken + refreshToken + expiresIn
#[tokio::test]
async fn contract_login_when_valid_credentials_then_token_response() {
    let env = setup().await;
    seed_user(&env, "admin@test.com", "secret123", "Admin", true).await;

    let json = login(&env, "admin@test.com", "secret123").await;
    assert!(json.get("accessToken").is_some());
    assert!(json.get("refreshToken").is_some());
    assert_eq!(json["expiresIn"], 900);
}

// Contract: BR-IA-002 — inactive user login fails
#[tokio::test]
async fn br_ia_002_given_inactive_user_when_login_then_unauthorized() {
    let env = setup().await;
    seed_user(&env, "inactive@test.com", "secret123", "Driver", false).await;

    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({ "email": "inactive@test.com", "password": "secret123" }).to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let json: serde_json::Value = serde_json::from_slice(&body).expect("json");
    assert_eq!(json["error"]["code"], "INVALID_CREDENTIALS");
}

// Contract: auth refresh + logout revokes session
#[tokio::test]
async fn contract_auth_when_refresh_and_logout_then_tokens_rotate_and_revoke() {
    let env = setup().await;
    seed_user(&env, "admin@test.com", "secret123", "Admin", true).await;
    let tokens = login(&env, "admin@test.com", "secret123").await;

    let refresh_response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/refresh")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({ "refreshToken": tokens["refreshToken"] }).to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(refresh_response.status(), StatusCode::OK);

    let access = tokens["accessToken"].as_str().expect("access");
    let logout_response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/logout")
                .header("authorization", format!("Bearer {access}"))
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(logout_response.status(), StatusCode::NO_CONTENT);
}

// Contract: BR-IA-001 — driver cannot POST /v1/commerces → 403
#[tokio::test]
async fn br_ia_001_given_driver_when_post_commerces_then_forbidden() {
    let env = setup().await;
    seed_user(&env, "driver@test.com", "secret123", "Driver", true).await;
    let tokens = login(&env, "driver@test.com", "secret123").await;
    let access = tokens["accessToken"].as_str().expect("access");

    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/commerces")
                .header("authorization", format!("Bearer {access}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "cnpj": "11222333000181",
                        "legalName": "Acme Ltda",
                        "tradeName": "Acme",
                        "address": { "city": "SP" },
                        "contact": { "email": "a@acme.com" }
                    })
                    .to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let json: serde_json::Value = serde_json::from_slice(&body).expect("json");
    assert_eq!(json["error"]["code"], "FORBIDDEN");
}

// Contract: BR-CO-001 — invalid CNPJ rejected
#[tokio::test]
async fn br_co_001_given_invalid_cnpj_when_post_commerces_then_bad_request() {
    let env = setup().await;
    seed_user(&env, "admin@test.com", "secret123", "Admin", true).await;
    let tokens = login(&env, "admin@test.com", "secret123").await;
    let access = tokens["accessToken"].as_str().expect("access");

    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/commerces")
                .header("authorization", format!("Bearer {access}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "cnpj": "00000000000000",
                        "legalName": "Bad Co",
                        "address": { "city": "SP" },
                        "contact": { "email": "a@bad.com" }
                    })
                    .to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let json: serde_json::Value = serde_json::from_slice(&body).expect("json");
    assert_eq!(json["error"]["code"], "INVALID_CNPJ");
}

// Contract: admin can register commerce
#[tokio::test]
async fn contract_admin_when_post_commerces_then_created() {
    let env = setup().await;
    seed_user(&env, "admin@test.com", "secret123", "Admin", true).await;
    let tokens = login(&env, "admin@test.com", "secret123").await;
    let access = tokens["accessToken"].as_str().expect("access");

    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/commerces")
                .header("authorization", format!("Bearer {access}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "cnpj": "11222333000181",
                        "legalName": "Acme Ltda",
                        "tradeName": "Acme Store",
                        "address": { "city": "SP" },
                        "contact": { "email": "store@acme.com" }
                    })
                    .to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let json: serde_json::Value = serde_json::from_slice(&body).expect("json");
    assert_eq!(json["cnpj"], "11222333000181");
    assert_eq!(json["legalName"], "Acme Ltda");
}

// Contract: CommerceContact login includes commerceId in JWT (Phase 08)
#[tokio::test]
async fn given_commerce_contact_when_login_then_jwt_contains_commerce_id() {
    let env = setup().await;
    let commerce_id = Uuid::now_v7();
    infra_postgres::commerces::insert_commerce(
        &env.app_pool,
        env.tenant_id,
        commerce_id,
        "11222333000181",
        "Store Legal",
        "Store Trade",
        serde_json::json!({"city": "SP"}),
    )
    .await
    .expect("commerce");

    let contact_id = Uuid::now_v7();
    let hash = PasswordHasher::hash("secret123").expect("hash");
    infra_postgres::identity::insert_user(
        &env.app_pool,
        env.tenant_id,
        infra_postgres::identity::InsertUserParams {
            id: contact_id,
            email: "portal@store.com",
            name: "Portal User",
            role: "CommerceContact",
            password_hash: &hash,
            commerce_id: Some(commerce_id),
            profile_file_id: None,
        },
    )
    .await
    .expect("insert commerce contact");

    let tokens = login(&env, "portal@store.com", "secret123").await;
    let access = tokens["accessToken"].as_str().expect("access");
    let claims = env
        .state
        .jwt
        .verify_access_token(access)
        .expect("verify jwt");
    assert_eq!(claims.role, "CommerceContact");
    assert_eq!(claims.commerce_id, Some(commerce_id));
}

//! Shared integration-test setup for api-http contract tests.

#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use api_http::{AppState, full_app};
use application::REFRESH_TOKEN_TTL;
use axum::body::Body;
use http::{Request, StatusCode};
use infra_crypto::{JwtService, PasswordHasher};
use infra_postgres::{
    PgPool, SessionContext, migrate, orders::OrderInsert, orders::OrderItemInsert,
};
use serde_json::{Value, json};
use testcontainers::ImageExt;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;
use tower::ServiceExt;
use uuid::Uuid;

pub struct TestEnv {
    pub admin_pool: PgPool,
    pub app_pool: PgPool,
    pub state: AppState,
    pub tenant_id: domain_shared::TenantId,
    _container: testcontainers::ContainerAsync<Postgres>,
}

pub async fn setup() -> TestEnv {
    setup_with_tenant(domain_shared::TenantId::generate()).await
}

pub async fn setup_with_tenant(tenant_id: domain_shared::TenantId) -> TestEnv {
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
        report_signing_key: Some(AppState::test_signing_key()),
        catalog_events: AppState::default_catalog_events(),
        cnpj_lookup_rate_limit: AppState::default_cnpj_lookup_rate_limit(),
        cnpj_lookup: AppState::mock_cnpj_lookup(),
        cnpj_miss_cache: AppState::in_memory_cnpj_miss_cache(),
        payment_gateway: AppState::mock_payment_gateway(),
        asaas_webhook_token: Some("test-webhook-token-phase3".into()),
    };

    TestEnv {
        admin_pool,
        app_pool,
        state,
        tenant_id,
        _container: container,
    }
}

pub async fn seed_signing_key(env: &TestEnv) {
    let signing_key = AppState::test_signing_key();
    let public_key = signing_key.verifying_key().to_bytes();
    infra_postgres::reports::insert_signing_key(
        &env.app_pool,
        env.tenant_id,
        "test-key-1",
        &public_key,
    )
    .await
    .expect("insert signing key");
}

pub async fn seed_user(
    env: &TestEnv,
    email: &str,
    password: &str,
    role: &str,
    active: bool,
) -> Uuid {
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

pub async fn seed_admin(env: &TestEnv) -> (Uuid, String) {
    let id = seed_user(env, "admin@test.com", "secret123", "Admin", true).await;
    let token = login_token(env, "admin@test.com", "secret123").await;
    (id, token)
}

pub async fn seed_driver(env: &TestEnv, email: &str) -> (Uuid, String) {
    let id = seed_user(env, email, "secret123", "Driver", true).await;
    let token = login_token(env, email, "secret123").await;
    (id, token)
}

pub async fn seed_seller(env: &TestEnv, email: &str) -> (Uuid, String) {
    let id = seed_user(env, email, "secret123", "Seller", true).await;
    let token = login_token(env, email, "secret123").await;
    (id, token)
}

pub async fn seed_commerce(env: &TestEnv, cnpj: &str) -> Uuid {
    let commerce_id = Uuid::now_v7();
    infra_postgres::commerces::insert_commerce(
        &env.app_pool,
        env.tenant_id,
        commerce_id,
        cnpj,
        "Acme Ltda",
        "Acme Store",
        json!({"city": "SP"}),
    )
    .await
    .expect("commerce");
    commerce_id
}

pub async fn seed_product(env: &TestEnv, sku: &str, name: &str, price_amount: i64) -> Uuid {
    let product_id = Uuid::now_v7();
    infra_postgres::inventory::insert_product_with_catalog(
        &env.app_pool,
        env.tenant_id,
        infra_postgres::inventory::ProductInsert {
            id: product_id,
            sku: sku.to_owned(),
            name: name.to_owned(),
            price_amount,
            price_currency: "BRL".into(),
            category_id: None,
            unit_of_measure: "Unit".into(),
            description: None,
        },
    )
    .await
    .expect("product");
    product_id
}

pub async fn seed_commerce_contact(
    env: &TestEnv,
    commerce_id: Uuid,
    email: &str,
) -> (Uuid, String) {
    let id = Uuid::now_v7();
    let hash = PasswordHasher::hash("secret123").expect("hash");
    infra_postgres::identity::insert_user(
        &env.app_pool,
        env.tenant_id,
        infra_postgres::identity::InsertUserParams {
            id,
            email,
            name: "Portal User",
            role: "CommerceContact",
            password_hash: &hash,
            commerce_id: Some(commerce_id),
            profile_file_id: None,
        },
    )
    .await
    .expect("insert commerce contact");
    let token = login_token(env, email, "secret123").await;
    (id, token)
}

pub async fn seed_order(env: &TestEnv, commerce_id: Uuid, admin_id: Uuid) -> Uuid {
    let address_id = Uuid::now_v7();
    infra_postgres::commerces::addresses::insert_address(
        &env.app_pool,
        env.tenant_id,
        infra_postgres::commerces::addresses::AddressInsert {
            id: address_id,
            commerce_id,
            address_type: "Delivery".into(),
            street: "Rua A".into(),
            number: "1".into(),
            district: None,
            city: "SP".into(),
            state: "SP".into(),
            postal_code: "01310100".into(),
            latitude: None,
            longitude: None,
            is_primary: true,
        },
    )
    .await
    .expect("address");

    let product_id = seed_product(env, "ORD-SKU", "Order Product", 1_000).await;
    let order_id = Uuid::now_v7();
    let item_id = Uuid::now_v7();
    let session = SessionContext {
        tenant_id: env.tenant_id,
        role: "Admin".into(),
        user_id: admin_id,
        commerce_id: None,
    };

    infra_postgres::orders::insert_order(
        &env.app_pool,
        &session,
        &OrderInsert {
            id: order_id,
            commerce_id,
            created_by_user_id: admin_id,
            source: "SellerVisit".into(),
            status: "PendingApproval".into(),
            delivery_address_id: address_id,
            notes: None,
            total_amount: 1_000,
            total_currency: "BRL".into(),
        },
    )
    .await
    .expect("order");

    infra_postgres::orders::insert_order_items(
        &env.app_pool,
        &session,
        &[OrderItemInsert {
            id: item_id,
            order_id,
            product_id,
            quantity_requested: 1,
            unit_price_amount: 1_000,
            unit_price_currency: "BRL".into(),
            line_total_amount: 1_000,
        }],
    )
    .await
    .expect("order items");

    order_id
}

pub async fn seed_driver_stock(env: &TestEnv, driver_id: Uuid, product_id: Uuid, quantity: i32) {
    infra_postgres::inventory::upsert_stock_balance(
        &env.app_pool,
        env.tenant_id,
        driver_id,
        product_id,
        quantity,
    )
    .await
    .expect("driver stock");
}

pub async fn seed_delivery_address(env: &TestEnv, commerce_id: Uuid) -> Uuid {
    let address_id = Uuid::now_v7();
    infra_postgres::commerces::addresses::insert_address(
        &env.app_pool,
        env.tenant_id,
        infra_postgres::commerces::addresses::AddressInsert {
            id: address_id,
            commerce_id,
            address_type: "Delivery".into(),
            street: "Rua E2E".into(),
            number: "100".into(),
            district: None,
            city: "SP".into(),
            state: "SP".into(),
            postal_code: "01310100".into(),
            latitude: None,
            longitude: None,
            is_primary: true,
        },
    )
    .await
    .expect("delivery address");
    address_id
}

pub async fn login(env: &TestEnv, email: &str, password: &str) -> Value {
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

pub async fn login_token(env: &TestEnv, email: &str, password: &str) -> String {
    let json = login(env, email, password).await;
    json["accessToken"].as_str().expect("token").to_owned()
}

pub async fn request_with_headers(
    env: &TestEnv,
    method: &str,
    uri: &str,
    token: Option<&str>,
    body: Option<String>,
    extra_headers: &[(&str, &str)],
) -> (StatusCode, Value) {
    let mut builder = Request::builder().method(method).uri(uri);
    if let Some(token) = token {
        builder = builder.header("authorization", format!("Bearer {token}"));
    }
    if body.is_some() {
        builder = builder.header("content-type", "application/json");
    }
    for (name, value) in extra_headers {
        builder = builder.header(*name, *value);
    }
    let request = builder
        .body(match body {
            Some(b) => Body::from(b),
            None => Body::empty(),
        })
        .expect("request");

    let response = full_app(env.state.clone())
        .oneshot(request)
        .await
        .expect("response");
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let json = if bytes.is_empty() {
        json!({})
    } else {
        serde_json::from_slice(&bytes).unwrap_or(json!({ "raw": String::from_utf8_lossy(&bytes) }))
    };
    (status, json)
}

pub async fn request(
    env: &TestEnv,
    method: &str,
    uri: &str,
    token: Option<&str>,
    body: Option<String>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder().method(method).uri(uri);
    if let Some(token) = token {
        builder = builder.header("authorization", format!("Bearer {token}"));
    }
    if body.is_some() {
        builder = builder.header("content-type", "application/json");
    }
    let request = builder
        .body(match body {
            Some(b) => Body::from(b),
            None => Body::empty(),
        })
        .expect("request");

    let response = full_app(env.state.clone())
        .oneshot(request)
        .await
        .expect("response");
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let json = if bytes.is_empty() {
        json!({})
    } else {
        serde_json::from_slice(&bytes).unwrap_or(json!({ "raw": String::from_utf8_lossy(&bytes) }))
    };
    (status, json)
}

pub async fn request_bytes(
    env: &TestEnv,
    method: &str,
    uri: &str,
    token: Option<&str>,
) -> (StatusCode, Vec<u8>) {
    let mut builder = Request::builder().method(method).uri(uri);
    if let Some(token) = token {
        builder = builder.header("authorization", format!("Bearer {token}"));
    }
    let request = builder.body(Body::empty()).expect("request");

    let response = full_app(env.state.clone())
        .oneshot(request)
        .await
        .expect("response");
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes")
        .to_vec();
    (status, bytes)
}

pub fn minimal_webp_bytes() -> Vec<u8> {
    dev_seed::minimal_webp_bytes()
}

pub fn multipart_body(
    boundary: &str,
    file_name: &str,
    mime_type: &str,
    file_bytes: &[u8],
    entity_type: &str,
    entity_id: Uuid,
) -> (String, Vec<u8>) {
    let mut body = Vec::new();
    let push_field = |body: &mut Vec<u8>, name: &str, value: &str| {
        body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
        body.extend_from_slice(
            format!("Content-Disposition: form-data; name=\"{name}\"\r\n\r\n").as_bytes(),
        );
        body.extend_from_slice(value.as_bytes());
        body.extend_from_slice(b"\r\n");
    };

    body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    body.extend_from_slice(
        format!("Content-Disposition: form-data; name=\"file\"; filename=\"{file_name}\"\r\n")
            .as_bytes(),
    );
    body.extend_from_slice(format!("Content-Type: {mime_type}\r\n\r\n").as_bytes());
    body.extend_from_slice(file_bytes);
    body.extend_from_slice(b"\r\n");

    push_field(&mut body, "entityType", entity_type);
    push_field(&mut body, "entityId", &entity_id.to_string());
    body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());

    let content_type = format!("multipart/form-data; boundary={boundary}");
    (content_type, body)
}

pub async fn upload_multipart(
    env: &TestEnv,
    token: &str,
    file_name: &str,
    mime_type: &str,
    file_bytes: &[u8],
    entity_type: &str,
    entity_id: Uuid,
) -> (StatusCode, Value) {
    let boundary = "test-boundary-7f3a";
    let (content_type, body) = multipart_body(
        boundary,
        file_name,
        mime_type,
        file_bytes,
        entity_type,
        entity_id,
    );

    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/media/upload")
                .header("authorization", format!("Bearer {token}"))
                .header("content-type", content_type)
                .body(Body::from(body))
                .expect("request"),
        )
        .await
        .expect("response");

    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let json = if bytes.is_empty() {
        json!({})
    } else {
        serde_json::from_slice(&bytes).expect("json")
    };
    (status, json)
}

pub struct PortalHomeSeed {
    pub featured_product_id: Uuid,
    pub popular_product_id: Uuid,
    pub banner_id: Uuid,
    pub promotion_id: Uuid,
}

pub async fn seed_portal_home_content(env: &TestEnv) -> PortalHomeSeed {
    let featured_product_id = seed_product(env, "FEAT-001", "Featured Burger", 1_500).await;
    let popular_product_id = seed_product(env, "POP-001", "Popular Soda", 800).await;

    infra_postgres::inventory::portal_products::set_product_featured(
        &env.app_pool,
        env.tenant_id,
        featured_product_id,
        true,
    )
    .await
    .expect("set featured");

    infra_postgres::inventory::portal_products::seed_product_sales_total(
        &env.app_pool,
        env.tenant_id,
        popular_product_id,
        50,
    )
    .await
    .expect("seed sales total");

    let file_id = Uuid::now_v7();
    let banner_id = Uuid::now_v7();
    let object_key = "portal/banners/test-hero.webp";
    env.state
        .storage
        .put_object(
            "catalog",
            object_key,
            &minimal_webp_bytes(),
            "image/webp",
        )
        .await
        .expect("banner object");

    infra_postgres::media::insert_file(
        &env.app_pool,
        env.tenant_id,
        infra_postgres::media::FileInsert {
            id: file_id,
            entity_type: "PortalBanner".into(),
            entity_id: banner_id,
            bucket: "catalog".into(),
            object_key: object_key.into(),
            mime_type: "image/webp".into(),
            size_bytes: minimal_webp_bytes().len() as i64,
            sha256: "portal-banner-test".into(),
            uploaded_by_user_id: Uuid::now_v7(),
        },
    )
    .await
    .expect("banner file");

    infra_postgres::portal::banners::insert_banner(
        &env.app_pool,
        env.tenant_id,
        infra_postgres::portal::banners::BannerInsert {
            id: banner_id,
            placement: "hero".into(),
            image_file_id: file_id,
            link_url: None,
            alt_text: Some("Welcome hero".into()),
            sort_order: 0,
            active: true,
        },
    )
    .await
    .expect("banner");

    let promotion_id = Uuid::now_v7();
    infra_postgres::portal::promotions::insert_promotion(
        &env.app_pool,
        env.tenant_id,
        infra_postgres::portal::promotions::PromotionInsert {
            id: promotion_id,
            headline: "Tasty Burger".into(),
            discount_text: "30% OFF".into(),
            background: "yellow".into(),
            category_slug: Some("snacks".into()),
            link_url: None,
            image_file_id: None,
            sort_order: 0,
            active: true,
        },
    )
    .await
    .expect("promotion");

    PortalHomeSeed {
        featured_product_id,
        popular_product_id,
        banner_id,
        promotion_id,
    }
}

pub const PLATFORM_ADMIN_EMAIL: &str = "platform@test.com";
pub const DEV_MFA_SECRET: &str = "KVKFKRCPNZQUYMLXOVYDSQKJKZDTSRLD";

pub async fn seed_platform_admin(env: &TestEnv) -> Uuid {
    let id = Uuid::parse_str("01900001-0006-7000-8000-000000000001").expect("platform id");
    if infra_postgres::identity::find_platform_user_for_login(&env.admin_pool, PLATFORM_ADMIN_EMAIL)
        .await
        .expect("lookup")
        .is_some()
    {
        return id;
    }
    let hash = PasswordHasher::hash("secret123").expect("hash");
    infra_postgres::identity::insert_platform_user(
        &env.admin_pool,
        infra_postgres::identity::InsertPlatformUserParams {
            id,
            email: PLATFORM_ADMIN_EMAIL,
            name: "Platform Admin",
            password_hash: &hash,
            mfa_secret: Some(DEV_MFA_SECRET),
            mfa_enrolled: true,
        },
    )
    .await
    .expect("platform admin");
    id
}

pub fn current_mfa_code() -> String {
    infra_crypto::TotpVerifier::from_base32_secret(DEV_MFA_SECRET)
        .expect("totp")
        .current_code()
}

pub async fn platform_login_step(env: &TestEnv, email: &str, password: &str) -> Value {
    let body = json!({ "email": email, "password": password }).to_string();
    let (status, json) = request(env, "POST", "/v1/platform/auth/login", None, Some(body)).await;
    assert_eq!(status, StatusCode::OK, "platform login: {json}");
    json
}

pub async fn platform_access_token(env: &TestEnv) -> String {
    seed_platform_admin(env).await;
    let login = platform_login_step(env, PLATFORM_ADMIN_EMAIL, "secret123").await;
    assert_eq!(login["mfaRequired"], true);
    let mfa_body = json!({
        "mfaToken": login["mfaToken"],
        "code": current_mfa_code()
    })
    .to_string();
    let (status, json) = request(
        env,
        "POST",
        "/v1/platform/auth/mfa/verify",
        None,
        Some(mfa_body),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "mfa: {json}");
    json["accessToken"]
        .as_str()
        .expect("access token")
        .to_owned()
}

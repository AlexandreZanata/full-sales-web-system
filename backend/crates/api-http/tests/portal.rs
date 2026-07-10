//! Portal API contract tests — Phase 14.

use std::sync::Arc;
use std::time::Duration;

use api_http::{AppState, full_app};
use axum::body::Body;
use http::{Request, StatusCode};
use infra_crypto::{JwtService, PasswordHasher};
use infra_postgres::{PgPool, migrate};
use infra_redis::InMemoryRefreshTokenStore;
use infra_storage::InMemoryObjectStorage;
use serde_json::json;
use testcontainers::ImageExt;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;
use tower::ServiceExt;
use uuid::Uuid;

struct PortalEnv {
    state: AppState,
    tenant_id: domain_shared::TenantId,
    commerce_a: Uuid,
    commerce_b: Uuid,
    address_a: Uuid,
    product_id: Uuid,
    contact_a_token: String,
    contact_b_token: String,
    admin_token: String,
    app_pool: PgPool,
    _container: testcontainers::ContainerAsync<Postgres>,
}

async fn setup() -> PortalEnv {
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
    infra_postgres::shared::insert_tenant(&admin_pool, tenant_id, "Portal Tenant")
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
        credential_encryptor: Some(AppState::test_credential_encryptor()),
        settlement_cache: AppState::test_settlement_cache(),
        settlement_rate_limit: AppState::default_settlement_rate_limit(),
        velocity_counter: AppState::in_memory_velocity_counter(),
        dns_resolver: AppState::empty_dns_resolver(),
        health_config: AppState::health_config_from_env(),
        tenant_asaas_base_url: None,
    };

    let commerce_a = Uuid::now_v7();
    let commerce_b = Uuid::now_v7();
    let address_a = Uuid::now_v7();
    let product_id = Uuid::now_v7();
    let file_id = Uuid::now_v7();

    seed_user(&app_pool, tenant_id, "admin@test.com", "Admin", None).await;
    seed_user(
        &app_pool,
        tenant_id,
        "contact-a@test.com",
        "CommerceContact",
        Some(commerce_a),
    )
    .await;
    seed_user(
        &app_pool,
        tenant_id,
        "contact-b@test.com",
        "CommerceContact",
        Some(commerce_b),
    )
    .await;

    infra_postgres::commerces::insert_commerce(
        &app_pool,
        tenant_id,
        commerce_a,
        "11222333000181",
        "Commerce A Legal",
        "Commerce A",
        json!({"city": "SP"}),
    )
    .await
    .expect("commerce a");

    infra_postgres::commerces::insert_commerce(
        &app_pool,
        tenant_id,
        commerce_b,
        "11222333000182",
        "Commerce B Legal",
        "Commerce B",
        json!({"city": "SP"}),
    )
    .await
    .expect("commerce b");

    infra_postgres::commerces::addresses::insert_address(
        &app_pool,
        tenant_id,
        infra_postgres::commerces::addresses::AddressInsert {
            id: address_a,
            commerce_id: commerce_a,
            address_type: "Delivery".into(),
            street: "Rua Portal".into(),
            number: "10".into(),
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

    infra_postgres::inventory::insert_product_with_catalog(
        &app_pool,
        tenant_id,
        infra_postgres::inventory::ProductInsert {
            id: product_id,
            sku: "PORT-001".into(),
            name: "Portal Widget".into(),
            price_amount: 2_500,
            price_currency: "BRL".into(),
            category_id: None,
            unit_of_measure: "Unit".into(),
            description: None,
        },
    )
    .await
    .expect("product");

    infra_postgres::media::insert_file(
        &app_pool,
        tenant_id,
        infra_postgres::media::FileInsert {
            id: file_id,
            entity_type: "Product".into(),
            entity_id: product_id,
            bucket: "catalog".into(),
            object_key: "widget.jpg".into(),
            mime_type: "image/jpeg".into(),
            size_bytes: 512,
            sha256: "abc".into(),
            uploaded_by_user_id: Uuid::now_v7(),
        },
    )
    .await
    .expect("file");

    infra_postgres::inventory::product_images::insert_product_image(
        &app_pool,
        tenant_id,
        infra_postgres::inventory::product_images::ProductImageInsert {
            id: Uuid::now_v7(),
            product_id,
            file_id,
            sort_order: 0,
            is_primary: true,
        },
    )
    .await
    .expect("image");

    state
        .storage
        .put_object("catalog", "widget.jpg", b"fake-image", "image/jpeg")
        .await
        .expect("seed object");

    infra_postgres::inventory::upsert_stock_balance(
        &app_pool,
        tenant_id,
        Uuid::now_v7(),
        product_id,
        50,
    )
    .await
    .expect("stock");

    PortalEnv {
        contact_a_token: login(&state, "contact-a@test.com").await,
        contact_b_token: login(&state, "contact-b@test.com").await,
        admin_token: login(&state, "admin@test.com").await,
        state,
        tenant_id,
        commerce_a,
        commerce_b,
        address_a,
        product_id,
        app_pool,
        _container: container,
    }
}

async fn seed_user(
    pool: &PgPool,
    tenant_id: domain_shared::TenantId,
    email: &str,
    role: &str,
    commerce_id: Option<Uuid>,
) {
    let hash = PasswordHasher::hash("password").expect("hash");
    infra_postgres::identity::insert_user(
        pool,
        tenant_id,
        infra_postgres::identity::InsertUserParams {
            id: Uuid::now_v7(),
            email,
            name: "User",
            role,
            password_hash: &hash,
            commerce_id,
            profile_file_id: None,
        },
    )
    .await
    .expect("user");
}

async fn login(state: &AppState, email: &str) -> String {
    let response = full_app(state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"email": email, "password": "password"}).to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body");
    let json: serde_json::Value = serde_json::from_slice(&body).expect("json");
    json["accessToken"].as_str().expect("token").to_owned()
}

#[tokio::test]
async fn contract_portal_products_when_commerce_contact_then_200_with_image_url() {
    let env = setup().await;
    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/portal/products")
                .header("authorization", format!("Bearer {}", env.contact_a_token))
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body");
    let json: serde_json::Value = serde_json::from_slice(&body).expect("json");
    assert_eq!(json["data"][0]["sku"], "PORT-001");
    let image_url = json["data"][0]["primaryImageUrl"]
        .as_str()
        .expect("primaryImageUrl");
    assert!(
        image_url.starts_with("/v1/public/media/"),
        "expected browser-loadable public media URL, got {image_url}"
    );
}

#[tokio::test]
async fn contract_driver_products_when_commerce_contact_then_forbidden() {
    let env = setup().await;
    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/products")
                .header("authorization", format!("Bearer {}", env.contact_a_token))
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn given_other_commerce_contact_when_list_orders_then_empty() {
    let env = setup().await;

    let create = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/portal/orders")
                .header("authorization", format!("Bearer {}", env.contact_a_token))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "deliveryAddressId": env.address_a,
                        "items": [{ "productId": env.product_id, "quantity": 2 }]
                    })
                    .to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("create");
    assert_eq!(create.status(), StatusCode::CREATED);

    let list = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/portal/orders")
                .header("authorization", format!("Bearer {}", env.contact_b_token))
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("list");
    assert_eq!(list.status(), StatusCode::OK);
    let body = axum::body::to_bytes(list.into_body(), usize::MAX)
        .await
        .expect("body");
    let json: serde_json::Value = serde_json::from_slice(&body).expect("json");
    assert_eq!(json["data"].as_array().map(|items| items.len()), Some(0));
}

#[tokio::test]
async fn given_portal_flow_when_create_submit_approve_then_order_approved() {
    let env = setup().await;

    let create = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/portal/orders")
                .header("authorization", format!("Bearer {}", env.contact_a_token))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "deliveryAddressId": env.address_a,
                        "items": [{ "productId": env.product_id, "quantity": 3 }]
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
    let order: serde_json::Value = serde_json::from_slice(&create_body).expect("json");
    let order_id = order["id"].as_str().expect("id");
    assert_eq!(order["status"], "Draft");
    assert_eq!(order["totalAmount"], 7_500);

    let submit = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/portal/orders/{order_id}/submit"))
                .header("authorization", format!("Bearer {}", env.contact_a_token))
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("submit");
    assert_eq!(submit.status(), StatusCode::OK);
    let submit_body = axum::body::to_bytes(submit.into_body(), usize::MAX)
        .await
        .expect("body");
    let submitted: serde_json::Value = serde_json::from_slice(&submit_body).expect("json");
    assert_eq!(submitted["status"], "PendingApproval");

    let approve = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/orders/{order_id}/approve"))
                .header("authorization", format!("Bearer {}", env.admin_token))
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("approve");
    assert_eq!(approve.status(), StatusCode::OK);
    let approve_body = axum::body::to_bytes(approve.into_body(), usize::MAX)
        .await
        .expect("body");
    let approved: serde_json::Value = serde_json::from_slice(&approve_body).expect("json");
    assert_eq!(approved["status"], "Approved");
}

#[tokio::test]
async fn contract_reject_order_when_missing_reason_then_400() {
    let env = setup().await;

    let create = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/portal/orders")
                .header("authorization", format!("Bearer {}", env.contact_a_token))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "deliveryAddressId": env.address_a,
                        "items": [{ "productId": env.product_id, "quantity": 1 }]
                    })
                    .to_string(),
                ))
                .expect("request"),
        )
        .await
        .expect("create");
    let create_body = axum::body::to_bytes(create.into_body(), usize::MAX)
        .await
        .expect("body");
    let order: serde_json::Value = serde_json::from_slice(&create_body).expect("json");
    let order_id = order["id"].as_str().expect("id");

    full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/portal/orders/{order_id}/submit"))
                .header("authorization", format!("Bearer {}", env.contact_a_token))
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("submit");

    let reject = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/orders/{order_id}/reject"))
                .header("authorization", format!("Bearer {}", env.admin_token))
                .header("content-type", "application/json")
                .body(Body::from(json!({"reason": "   "}).to_string()))
                .expect("request"),
        )
        .await
        .expect("reject");
    assert_eq!(reject.status(), StatusCode::BAD_REQUEST);
    let body = axum::body::to_bytes(reject.into_body(), usize::MAX)
        .await
        .expect("body");
    let json: serde_json::Value = serde_json::from_slice(&body).expect("json");
    assert_eq!(json["error"]["code"], "REJECTION_REASON_REQUIRED");
}

async fn attach_gallery_image(
    env: &PortalEnv,
    product_id: Uuid,
    object_key: &str,
    sort_order: i32,
    is_primary: bool,
) {
    let file_id = Uuid::now_v7();
    infra_postgres::media::insert_file(
        &env.app_pool,
        env.tenant_id,
        infra_postgres::media::FileInsert {
            id: file_id,
            entity_type: "Product".into(),
            entity_id: product_id,
            bucket: "catalog".into(),
            object_key: object_key.into(),
            mime_type: "image/jpeg".into(),
            size_bytes: 512,
            sha256: "abc".into(),
            uploaded_by_user_id: Uuid::now_v7(),
        },
    )
    .await
    .expect("file");

    infra_postgres::inventory::product_images::insert_product_image(
        &env.app_pool,
        env.tenant_id,
        infra_postgres::inventory::product_images::ProductImageInsert {
            id: Uuid::now_v7(),
            product_id,
            file_id,
            sort_order,
            is_primary,
        },
    )
    .await
    .expect("image");

    env.state
        .storage
        .put_object("catalog", object_key, b"fake-image", "image/jpeg")
        .await
        .expect("seed object");
}

#[tokio::test]
async fn contract_portal_product_detail_when_gallery_then_primary_and_image_urls() {
    let env = setup().await;
    attach_gallery_image(&env, env.product_id, "gallery-2.jpg", 1, false).await;
    attach_gallery_image(&env, env.product_id, "gallery-3.jpg", 2, false).await;

    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/v1/portal/products/{}", env.product_id))
                .header("authorization", format!("Bearer {}", env.contact_a_token))
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body");
    let json: serde_json::Value = serde_json::from_slice(&body).expect("json");
    assert_eq!(json["sku"], "PORT-001");
    assert_eq!(json["unitOfMeasure"], "Unit");
    assert!(json["primaryImageUrl"].as_str().is_some());
    assert_eq!(
        json["imageUrls"].as_array().map(|items| items.len()),
        Some(2)
    );
}

#[tokio::test]
async fn contract_portal_product_detail_when_inactive_then_not_found() {
    let env = setup().await;
    infra_postgres::inventory::update_product(
        &env.app_pool,
        env.tenant_id,
        env.product_id,
        &infra_postgres::inventory::ProductUpdate {
            name: None,
            price_amount: None,
            price_currency: None,
            active: Some(false),
            category_id: None,
            unit_of_measure: None,
            description: None,
            is_featured: None,
        },
    )
    .await
    .expect("deactivate");

    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/v1/portal/products/{}", env.product_id))
                .header("authorization", format!("Bearer {}", env.contact_a_token))
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn contract_driver_portal_product_detail_when_not_commerce_contact_then_forbidden() {
    let env = setup().await;
    seed_user(
        &env.app_pool,
        env.tenant_id,
        "driver@test.com",
        "Driver",
        None,
    )
    .await;
    let driver_token = login(&env.state, "driver@test.com").await;

    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/v1/portal/products/{}", env.product_id))
                .header("authorization", format!("Bearer {}", driver_token))
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn contract_public_product_detail_when_other_tenant_product_then_not_found() {
    let env = setup().await;

    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/v1/public/products/{}", env.product_id))
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

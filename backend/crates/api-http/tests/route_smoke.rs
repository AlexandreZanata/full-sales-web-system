//! Phase 25 — Route registration smoke tests (ROUTE-MATRIX vs routes.rs).

#[path = "support/mod.rs"]
mod support;

use api_http::full_app;
use http::StatusCode;
use tower::ServiceExt;
use uuid::Uuid;

use support::{request, seed_admin, setup};

fn sample_id() -> Uuid {
    Uuid::now_v7()
}

/// Protected routes must return 401 without Bearer token (proves handler is mounted, not fallback 404).
#[tokio::test]
async fn route_smoke_protected_routes_return_unauthorized_not_not_found() {
    let env = setup().await;
    let id = sample_id();

    let protected: Vec<(&str, String)> = vec![
        ("POST", "/v1/auth/logout".into()),
        ("GET", "/v1/users".into()),
        ("POST", "/v1/users".into()),
        ("GET", format!("/v1/users/{id}")),
        ("PATCH", format!("/v1/users/{id}/deactivate")),
        ("PUT", format!("/v1/users/{id}/driver-profile")),
        ("PUT", format!("/v1/users/{id}/seller-profile")),
        ("GET", "/v1/commerces".into()),
        ("POST", "/v1/commerces".into()),
        ("GET", format!("/v1/commerces/{id}")),
        ("PATCH", format!("/v1/commerces/{id}/deactivate")),
        ("GET", format!("/v1/commerces/{id}/addresses")),
        ("POST", format!("/v1/commerces/{id}/addresses")),
        ("PATCH", format!("/v1/commerces/{id}/addresses/{id}")),
        ("PUT", format!("/v1/commerces/{id}/logo")),
        ("GET", "/v1/products".into()),
        ("POST", "/v1/products".into()),
        ("GET", format!("/v1/products/{id}")),
        ("PATCH", format!("/v1/products/{id}")),
        ("GET", format!("/v1/products/{id}/images")),
        ("POST", format!("/v1/products/{id}/images")),
        ("DELETE", format!("/v1/products/{id}/images/{id}")),
        ("GET", "/v1/categories".into()),
        ("POST", "/v1/categories".into()),
        ("GET", format!("/v1/categories/{id}")),
        ("PATCH", format!("/v1/categories/{id}")),
        ("DELETE", format!("/v1/categories/{id}")),
        ("POST", "/v1/categories/reorder".into()),
        ("PUT", format!("/v1/categories/{id}/image")),
        ("GET", "/v1/inventory/balances".into()),
        ("GET", format!("/v1/inventory/products/{id}/balance")),
        ("POST", "/v1/inventory/movements".into()),
        ("GET", format!("/v1/inventory/products/{id}/movements")),
        ("GET", "/v1/sales".into()),
        ("POST", "/v1/sales".into()),
        ("GET", format!("/v1/sales/{id}")),
        ("POST", format!("/v1/sales/{id}/confirm")),
        ("POST", format!("/v1/sales/{id}/cancel")),
        ("POST", format!("/v1/sales/{id}/declare-payment")),
        ("GET", "/v1/portal/products".into()),
        ("GET", format!("/v1/portal/products/{id}")),
        ("GET", "/v1/portal/orders".into()),
        ("POST", "/v1/portal/orders".into()),
        ("GET", format!("/v1/portal/orders/{id}")),
        ("PUT", format!("/v1/portal/orders/{id}")),
        ("DELETE", format!("/v1/portal/orders/{id}")),
        ("POST", format!("/v1/portal/orders/{id}/submit")),
        ("GET", "/v1/portal/categories".into()),
        ("GET", "/v1/portal/categories/bebidas".into()),
        ("GET", "/v1/orders".into()),
        ("GET", format!("/v1/orders/{id}")),
        ("POST", format!("/v1/orders/{id}/approve")),
        ("POST", format!("/v1/orders/{id}/reject")),
        ("POST", format!("/v1/orders/{id}/cancel")),
        ("POST", format!("/v1/orders/{id}/start-picking")),
        ("POST", format!("/v1/orders/{id}/delivery")),
        ("GET", "/v1/deliveries".into()),
        ("GET", format!("/v1/deliveries/{id}")),
        ("POST", format!("/v1/deliveries/{id}/start-transit")),
        ("POST", format!("/v1/deliveries/{id}/confirm")),
        ("POST", "/v1/media/upload".into()),
        ("GET", format!("/v1/media/{id}/url")),
        ("GET", format!("/v1/media/{id}/content")),
        ("GET", "/v1/reports".into()),
        ("POST", "/v1/reports".into()),
        ("GET", format!("/v1/reports/{id}")),
        ("GET", format!("/v1/reports/{id}/export?format=csv")),
        ("GET", "/v1/audit/events".into()),
        ("GET", "/v1/settings".into()),
        ("PATCH", "/v1/settings".into()),
        ("PUT", "/v1/settings/logo".into()),
    ];

    for (method, uri) in protected {
        let (status, body) = request(&env, method, &uri, None, None).await;
        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "protected route {method} {uri} should be 401, got {status} body={body:?}"
        );
    }
}

/// Public routes must not hit the global 404 fallback.
#[tokio::test]
async fn route_smoke_public_routes_are_mounted() {
    let env = setup().await;
    let id = sample_id();

    let (login_status, _) = request(
        &env,
        "POST",
        "/v1/auth/login",
        None,
        Some(r#"{"email":"x","password":"y"}"#.into()),
    )
    .await;
    assert_ne!(login_status, StatusCode::NOT_FOUND);

    let (refresh_status, _) = request(
        &env,
        "POST",
        "/v1/auth/refresh",
        None,
        Some(r#"{"refreshToken":"invalid"}"#.into()),
    )
    .await;
    assert_ne!(refresh_status, StatusCode::NOT_FOUND);

    let (verify_status, verify_body) =
        request(&env, "GET", &format!("/v1/reports/{id}/verify"), None, None).await;
    assert_eq!(verify_status, StatusCode::NOT_FOUND);
    assert_eq!(verify_body["error"]["code"], "REPORT_NOT_FOUND");

    let (products_status, _) = request(&env, "GET", "/v1/public/products", None, None).await;
    assert_eq!(products_status, StatusCode::OK);

    let (product_detail_status, product_detail_body) = request(
        &env,
        "GET",
        &format!("/v1/public/products/{id}"),
        None,
        None,
    )
    .await;
    assert_eq!(product_detail_status, StatusCode::NOT_FOUND);
    assert_eq!(product_detail_body["error"]["code"], "PRODUCT_NOT_FOUND");

    let (categories_status, _) = request(&env, "GET", "/v1/public/categories", None, None).await;
    assert_eq!(categories_status, StatusCode::OK);

    let (category_slug_status, category_slug_body) = request(
        &env,
        "GET",
        "/v1/public/categories/missing-slug",
        None,
        None,
    )
    .await;
    assert_eq!(category_slug_status, StatusCode::NOT_FOUND);
    assert_eq!(category_slug_body["error"]["code"], "CATEGORY_NOT_FOUND");

    let (media_status, media_body) = request(
        &env,
        "GET",
        &format!("/v1/public/media/{id}/content"),
        None,
        None,
    )
    .await;
    assert_eq!(media_status, StatusCode::NOT_FOUND);
    assert_eq!(media_body["error"]["code"], "MEDIA_NOT_FOUND");

    let sse_response = full_app(env.state.clone())
        .oneshot(
            http::Request::builder()
                .method("GET")
                .uri("/v1/public/catalog/events")
                .body(axum::body::Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");
    assert_eq!(sse_response.status(), StatusCode::OK);
}

/// Health/meta routes from ROUTE-MATRIX.
#[tokio::test]
async fn route_smoke_health_and_meta() {
    let env = setup().await;

    let (health_status, health_body) = request(&env, "GET", "/health", None, None).await;
    assert_eq!(health_status, StatusCode::OK);
    assert_eq!(health_body["status"], "ok");

    let (v1_status, v1_body) = request(&env, "GET", "/v1/", None, None).await;
    assert_eq!(v1_status, StatusCode::OK);
    assert_eq!(v1_body["version"], "1");
}

/// Audit events route — admin only, paginated.
#[tokio::test]
async fn route_smoke_audit_events_admin_only() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (unauth_status, _) = request(&env, "GET", "/v1/audit/events", None, None).await;
    assert_eq!(unauth_status, StatusCode::UNAUTHORIZED);

    let (status, body) = request(&env, "GET", "/v1/audit/events", Some(&admin_token), None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["items"].is_array());
}

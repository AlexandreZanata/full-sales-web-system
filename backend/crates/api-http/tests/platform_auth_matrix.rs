//! Phase 1F — PlatformAdmin authorization matrix extensions.

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;

use support::{platform_access_token, request, seed_admin, seed_driver, setup};

#[tokio::test]
async fn auth_matrix_driver_cannot_platform_login_with_tenant_credentials() {
    let env = setup().await;
    let (_, driver_token) = seed_driver(&env, "driver-platform@test.com").await;
    let (status, body) = request(&env, "GET", "/v1/platform/users", Some(&driver_token), None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["code"], "UNAUTHORIZED");
}

#[tokio::test]
async fn auth_matrix_platform_admin_can_platform_users_route() {
    let env = setup().await;
    let token = platform_access_token(&env).await;
    let (status, _) = request(&env, "GET", "/v1/platform/users", Some(&token), None).await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn auth_matrix_tenant_admin_forbidden_on_platform_routes() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;
    let (status, body) = request(
        &env,
        "POST",
        "/v1/platform/impersonate",
        Some(&admin_token),
        Some(r#"{"tenantId":"01900001-0000-7000-8000-000000000001","reason":"nope"}"#.into()),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["code"], "UNAUTHORIZED");
}

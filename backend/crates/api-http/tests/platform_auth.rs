//! Phase 1 — PlatformAdmin auth, RLS bypass, impersonation contract tests.

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{
    PLATFORM_ADMIN_EMAIL, current_mfa_code, platform_access_token, platform_login_step, request,
    seed_admin, seed_driver, seed_platform_admin, seed_user, setup, setup_with_tenant,
};

#[tokio::test]
async fn contract_platform_login_when_mfa_enrolled_then_mfa_required() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let body = platform_login_step(&env, PLATFORM_ADMIN_EMAIL, "secret123").await;
    assert_eq!(body["mfaRequired"], true);
    assert!(body["mfaToken"].is_string());
}

#[tokio::test]
async fn contract_platform_mfa_verify_when_valid_code_then_tokens() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let login = platform_login_step(&env, PLATFORM_ADMIN_EMAIL, "secret123").await;
    let mfa_body = json!({
        "mfaToken": login["mfaToken"],
        "code": current_mfa_code()
    })
    .to_string();
    let (status, body) = request(
        &env,
        "POST",
        "/v1/platform/auth/mfa/verify",
        None,
        Some(mfa_body),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["accessToken"].is_string());
    assert!(body["refreshToken"].is_string());
    assert_eq!(body["expiresIn"], 900);
}

#[tokio::test]
async fn contract_platform_login_when_bad_password_then_unauthorized() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let body = json!({
        "email": PLATFORM_ADMIN_EMAIL,
        "password": "wrong-password"
    })
    .to_string();
    let (status, resp) = request(&env, "POST", "/v1/platform/auth/login", None, Some(body)).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(resp["error"]["code"], "INVALID_CREDENTIALS");
}

#[tokio::test]
async fn contract_platform_login_when_rate_limited_then_429() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let body = json!({
        "email": PLATFORM_ADMIN_EMAIL,
        "password": "wrong"
    })
    .to_string();
    for _ in 0..5 {
        let _ = request(
            &env,
            "POST",
            "/v1/platform/auth/login",
            None,
            Some(body.clone()),
        )
        .await;
    }
    let (status, resp) = request(&env, "POST", "/v1/platform/auth/login", None, Some(body)).await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(resp["error"]["code"], "RATE_LIMITED");
}

#[tokio::test]
async fn contract_platform_admin_lists_users_from_two_tenants() {
    let tenant_a = domain_shared::TenantId::generate();
    let env = setup_with_tenant(tenant_a).await;
    seed_user(&env, "a-admin@test.com", "secret123", "Admin", true).await;
    seed_driver(&env, "a-driver@test.com").await;

    let tenant_b = domain_shared::TenantId::generate();
    infra_postgres::shared::insert_tenant(&env.admin_pool, tenant_b, "Tenant B")
        .await
        .expect("tenant b");
    seed_user_in_tenant(&env, tenant_b, "b-admin@test.com", "Admin").await;

    let token = platform_access_token(&env).await;
    let (status, body) = request(&env, "GET", "/v1/platform/users", Some(&token), None).await;
    assert_eq!(status, StatusCode::OK);
    let emails: Vec<&str> = body["data"]
        .as_array()
        .expect("data")
        .iter()
        .map(|row| row["email"].as_str().unwrap())
        .collect();
    assert!(emails.contains(&"a-admin@test.com"));
    assert!(emails.contains(&"a-driver@test.com"));
    assert!(emails.contains(&"b-admin@test.com"));
}

#[tokio::test]
async fn contract_tenant_admin_cannot_access_platform_users() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;
    let (status, body) = request(&env, "GET", "/v1/platform/users", Some(&admin_token), None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["code"], "UNAUTHORIZED");
}

#[tokio::test]
async fn contract_impersonation_token_can_list_tenant_users() {
    let env = setup().await;
    let (_, _) = seed_admin(&env).await;
    seed_driver(&env, "driver-imp@test.com").await;

    let platform_token = platform_access_token(&env).await;
    let impersonate_body = json!({
        "tenantId": env.tenant_id.as_uuid(),
        "reason": "support case 99"
    })
    .to_string();
    let (status, imp) = request(
        &env,
        "POST",
        "/v1/platform/impersonate",
        Some(&platform_token),
        Some(impersonate_body),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "impersonate: {imp}");
    let imp_token = imp["impersonationToken"].as_str().expect("token");

    let (status, users) = request(&env, "GET", "/v1/users", Some(imp_token), None).await;
    assert_eq!(status, StatusCode::OK, "users: {users}");
    assert!(
        users["data"]
            .as_array()
            .map(|a| !a.is_empty())
            .unwrap_or(false)
    );
}

#[tokio::test]
async fn contract_platform_token_without_impersonation_cannot_call_tenant_routes() {
    let env = setup().await;
    let token = platform_access_token(&env).await;
    let (status, body) = request(&env, "GET", "/v1/users", Some(&token), None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["code"], "UNAUTHORIZED");
}

async fn seed_user_in_tenant(
    env: &support::TestEnv,
    tenant_id: domain_shared::TenantId,
    email: &str,
    role: &str,
) {
    let id = uuid::Uuid::now_v7();
    let hash = infra_crypto::PasswordHasher::hash("secret123").expect("hash");
    infra_postgres::identity::insert_user(
        &env.app_pool,
        tenant_id,
        infra_postgres::identity::InsertUserParams {
            id,
            email,
            name: "Cross Tenant",
            role,
            password_hash: &hash,
            commerce_id: None,
            profile_file_id: None,
        },
    )
    .await
    .expect("user");
}

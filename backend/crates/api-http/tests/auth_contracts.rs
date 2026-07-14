//! Phase 17B — Auth route contracts (T-17-001..003). API-CONTRACT.md Auth section.

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{request, seed_admin, setup};

fn assert_token_envelope(body: &serde_json::Value) {
    assert!(body["accessToken"].as_str().is_some());
    assert!(body["refreshToken"].as_str().is_some());
    assert!(body["expiresIn"].as_u64().is_some());
}

// T-17-001 — POST /v1/auth/login
#[tokio::test]
async fn given_valid_credentials_when_login_then_200_token_envelope() {
    let env = setup().await;
    let _ = seed_admin(&env).await;

    let (status, body) = request(
        &env,
        "POST",
        "/v1/auth/login",
        None,
        Some(json!({ "email": "admin@test.com", "password": "secret123" }).to_string()),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_token_envelope(&body);
}

// T-17-001
#[tokio::test]
async fn given_wrong_password_when_login_then_401_invalid_credentials() {
    let env = setup().await;
    let _ = seed_admin(&env).await;

    let (status, body) = request(
        &env,
        "POST",
        "/v1/auth/login",
        None,
        Some(json!({ "email": "admin@test.com", "password": "nope" }).to_string()),
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["code"], "INVALID_CREDENTIALS");
}

// T-17-002 — POST /v1/auth/refresh
#[tokio::test]
async fn given_valid_refresh_when_refresh_then_200_new_tokens() {
    let env = setup().await;
    let _ = seed_admin(&env).await;
    let (login_status, login) = request(
        &env,
        "POST",
        "/v1/auth/login",
        None,
        Some(json!({ "email": "admin@test.com", "password": "secret123" }).to_string()),
    )
    .await;
    assert_eq!(login_status, StatusCode::OK);
    let refresh = login["refreshToken"].as_str().expect("refresh");

    let (status, body) = request(
        &env,
        "POST",
        "/v1/auth/refresh",
        None,
        Some(json!({ "refreshToken": refresh }).to_string()),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_token_envelope(&body);
}

// T-17-002
#[tokio::test]
async fn given_invalid_refresh_when_refresh_then_401() {
    let env = setup().await;

    let (status, body) = request(
        &env,
        "POST",
        "/v1/auth/refresh",
        None,
        Some(json!({ "refreshToken": "not-a-real-token" }).to_string()),
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["code"], "INVALID_CREDENTIALS");
}

// T-17-003 — POST /v1/auth/logout
#[tokio::test]
async fn given_bearer_when_logout_then_204_and_refresh_revoked() {
    let env = setup().await;
    let _ = seed_admin(&env).await;
    let (login_status, login) = request(
        &env,
        "POST",
        "/v1/auth/login",
        None,
        Some(json!({ "email": "admin@test.com", "password": "secret123" }).to_string()),
    )
    .await;
    assert_eq!(login_status, StatusCode::OK);
    let access = login["accessToken"].as_str().expect("access");
    let refresh = login["refreshToken"].as_str().expect("refresh").to_owned();

    let (status, _) = request(&env, "POST", "/v1/auth/logout", Some(access), None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (refresh_status, body) = request(
        &env,
        "POST",
        "/v1/auth/refresh",
        None,
        Some(json!({ "refreshToken": refresh }).to_string()),
    )
    .await;
    assert_eq!(refresh_status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["code"], "INVALID_CREDENTIALS");
}

// T-17-003
#[tokio::test]
async fn given_no_token_when_logout_then_401() {
    let env = setup().await;

    let (status, body) = request(&env, "POST", "/v1/auth/logout", None, None).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["code"], "UNAUTHORIZED");
}

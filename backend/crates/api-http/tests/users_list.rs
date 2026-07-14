//! Phase 68C — Users cursor list contract tests.

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{request, seed_admin, seed_driver, setup};

// T-17-005 — Contract: list pagination shape
#[tokio::test]
async fn contract_list_users_when_admin_then_cursor_envelope() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;
    seed_driver(&env, "driver1@test.com").await;

    let (status, body) = request(&env, "GET", "/v1/users?limit=10", Some(&admin_token), None).await;

    assert_eq!(status, StatusCode::OK);
    assert!(body["data"].is_array());
    assert_eq!(body["pagination"]["limit"], 10);
}

#[tokio::test]
async fn contract_list_users_when_filter_role_then_only_matching() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;
    seed_driver(&env, "driver1@test.com").await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/users?limit=50&filter[role]=Driver",
        Some(&admin_token),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    let data = body["data"].as_array().expect("data array");
    assert!(data.iter().all(|user| user["role"] == "Driver"));
}

// T-17-005
#[tokio::test]
async fn contract_list_users_when_invalid_filter_then_400() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/users?filter[unknown]=x",
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "invalid_filter_field");
}

#[tokio::test]
async fn contract_list_users_when_invalid_role_filter_then_400() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/users?filter[role]=NotARole",
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "invalid_filter_field");
}

// Contract: admin creates driver → 201
#[tokio::test]
async fn contract_admin_when_create_driver_then_201() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (status, body) = request(
        &env,
        "POST",
        "/v1/users",
        Some(&admin_token),
        Some(
            json!({
                "name": "New Driver",
                "email": "newdriver@test.com",
                "password": "secret123",
                "role": "Driver"
            })
            .to_string(),
        ),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["role"], "Driver");
    assert_eq!(body["email"], "newdriver@test.com");
    assert!(body["active"].as_bool().unwrap());
}

// Contract: driver cannot POST /users → 403
#[tokio::test]
async fn contract_driver_when_post_users_then_forbidden() {
    let env = setup().await;
    let (_, driver_token) = seed_driver(&env, "driver@test.com").await;

    let (status, body) = request(
        &env,
        "POST",
        "/v1/users",
        Some(&driver_token),
        Some(
            json!({
                "name": "Blocked",
                "email": "blocked@test.com",
                "password": "secret123",
                "role": "Driver"
            })
            .to_string(),
        ),
    )
    .await;

    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body["error"]["code"], "FORBIDDEN");
}

// Contract: deactivate → login fails
#[tokio::test]
async fn contract_deactivate_user_when_login_then_unauthorized() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;
    let (user_id, _) = seed_driver(&env, "todeactivate@test.com").await;

    let (deactivate_status, _) = request(
        &env,
        "PATCH",
        &format!("/v1/users/{user_id}/deactivate"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(deactivate_status, StatusCode::OK);

    let (login_status, login_body) = request(
        &env,
        "POST",
        "/v1/auth/login",
        None,
        Some(
            json!({
                "email": "todeactivate@test.com",
                "password": "secret123"
            })
            .to_string(),
        ),
    )
    .await;

    assert_eq!(login_status, StatusCode::UNAUTHORIZED);
    assert_eq!(login_body["error"]["code"], "INVALID_CREDENTIALS");
}

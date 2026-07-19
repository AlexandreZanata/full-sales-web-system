//! Phase 17B — Users create/get/deactivate contracts (T-17-004..007).

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use support::{request, seed_admin, seed_driver, setup};

fn assert_user_no_password(body: &serde_json::Value) {
    assert!(body.get("password").is_none());
    assert!(body.get("passwordHash").is_none());
    assert!(body["id"].as_str().is_some());
    assert!(body["email"].as_str().is_some());
    assert!(body["role"].as_str().is_some());
    assert!(body["active"].as_bool().is_some());
}

// T-17-004 — POST /v1/users
#[tokio::test]
async fn given_admin_when_create_user_then_201_without_password() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;

    let (status, body) = request(
        &env,
        "POST",
        "/v1/users",
        Some(&admin),
        Some(
            json!({
                "name": "New Seller",
                "email": "newseller@test.com",
                "password": "secret123",
                "role": "Seller"
            })
            .to_string(),
        ),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["role"], "Seller");
    assert_user_no_password(&body);
}

// T-17-004
#[tokio::test]
async fn given_no_token_when_create_user_then_401() {
    let env = setup().await;

    let (status, body) = request(
        &env,
        "POST",
        "/v1/users",
        None,
        Some(
            json!({
                "name": "X Y",
                "email": "x@test.com",
                "password": "secret123",
                "role": "Driver"
            })
            .to_string(),
        ),
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["code"], "UNAUTHORIZED");
}

// T-17-005 — GET /v1/users
#[tokio::test]
async fn given_no_token_when_list_users_then_401() {
    let env = setup().await;
    let (status, body) = request(&env, "GET", "/v1/users", None, None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["code"], "UNAUTHORIZED");
}

// T-17-006 — GET /v1/users/{id}
#[tokio::test]
async fn given_admin_when_get_user_then_200() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let (driver_id, _) = seed_driver(&env, "driver-get@test.com").await;

    let (status, body) = request(
        &env,
        "GET",
        &format!("/v1/users/{driver_id}"),
        Some(&admin),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["id"], driver_id.to_string());
    assert_user_no_password(&body);
}

// T-17-006
#[tokio::test]
async fn given_unknown_id_when_get_user_then_404_user_not_found() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let missing = Uuid::now_v7();

    let (status, body) = request(
        &env,
        "GET",
        &format!("/v1/users/{missing}"),
        Some(&admin),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "USER_NOT_FOUND");
}

// T-17-007 — PATCH /v1/users/{id}/deactivate
#[tokio::test]
async fn given_admin_when_deactivate_then_200_active_false() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let (user_id, _) = seed_driver(&env, "deact@test.com").await;

    let (status, body) = request(
        &env,
        "PATCH",
        &format!("/v1/users/{user_id}/deactivate"),
        Some(&admin),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["active"], false);
    assert_user_no_password(&body);
}

#[tokio::test]
async fn given_admin_when_patch_user_then_200_updated_name_email() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let (user_id, _) = seed_driver(&env, "edit-me@test.com").await;

    let (status, body) = request(
        &env,
        "PATCH",
        &format!("/v1/users/{user_id}"),
        Some(&admin),
        Some(
            json!({
                "name": "Edited Driver",
                "email": "edited-driver@test.com"
            })
            .to_string(),
        ),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["name"], "Edited Driver");
    assert_eq!(body["email"], "edited-driver@test.com");
    assert_user_no_password(&body);
}

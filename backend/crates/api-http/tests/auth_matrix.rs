//! Phase 25 — Authorization matrix sample contract tests.

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{request, seed_admin, seed_commerce, seed_commerce_contact, seed_driver, setup};

// Contract: driver POST /users → 403
#[tokio::test]
async fn auth_matrix_driver_post_users_forbidden() {
    let env = setup().await;
    let (_, driver_token) = seed_driver(&env, "driver@test.com").await;

    let (status, body) = request(
        &env,
        "POST",
        "/v1/users",
        Some(&driver_token),
        Some(
            json!({
                "name": "Blocked User",
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

// Contract: commerce contact GET /orders → 403
#[tokio::test]
async fn auth_matrix_commerce_contact_get_orders_forbidden() {
    let env = setup().await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let (_, contact_token) =
        seed_commerce_contact(&env, commerce_id, "contact@store.com").await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/orders",
        Some(&contact_token),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body["error"]["code"], "FORBIDDEN");
}

// Contract: admin can POST /users (allowed baseline)
#[tokio::test]
async fn auth_matrix_admin_post_users_allowed() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (status, _) = request(
        &env,
        "POST",
        "/v1/users",
        Some(&admin_token),
        Some(
            json!({
                "name": "Allowed Seller",
                "email": "seller@test.com",
                "password": "secret123",
                "role": "Seller"
            })
            .to_string(),
        ),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
}

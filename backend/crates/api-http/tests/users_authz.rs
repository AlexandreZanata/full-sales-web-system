//! Phase 17B — Wrong-role authz for Users routes (T-17-004..009 Authz).

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use support::{request, seed_admin, seed_driver, seed_seller, setup};

async fn assert_forbidden(
    env: &support::TestEnv,
    method: &str,
    uri: &str,
    token: &str,
    body: Option<String>,
) {
    let (status, resp) = request(env, method, uri, Some(token), body).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{method} {uri}");
    assert_eq!(resp["error"]["code"], "FORBIDDEN");
}

// T-17-004 / T-17-005 / T-17-006 / T-17-007 / T-17-008 / T-17-009
#[tokio::test]
async fn given_driver_when_users_admin_routes_then_403() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let (target_id, _) = seed_seller(&env, "target@test.com").await;
    let (_, driver) = seed_driver(&env, "driver-authz@test.com").await;

    assert_forbidden(
        &env,
        "POST",
        "/v1/users",
        &driver,
        Some(
            json!({
                "name": "Blocked User",
                "email": "blocked-authz@test.com",
                "password": "secret123",
                "role": "Driver"
            })
            .to_string(),
        ),
    )
    .await;
    assert_forbidden(&env, "GET", "/v1/users", &driver, None).await;
    assert_forbidden(
        &env,
        "GET",
        &format!("/v1/users/{target_id}"),
        &driver,
        None,
    )
    .await;
    assert_forbidden(
        &env,
        "PATCH",
        &format!("/v1/users/{target_id}/deactivate"),
        &driver,
        None,
    )
    .await;
    assert_forbidden(
        &env,
        "PUT",
        &format!("/v1/users/{target_id}/seller-profile"),
        &driver,
        Some(json!({ "operatingRegion": "RJ" }).to_string()),
    )
    .await;

    let _ = admin;
}

// T-17-008 / T-17-005
#[tokio::test]
async fn given_seller_when_users_admin_routes_then_403() {
    let env = setup().await;
    let (driver_id, _) = seed_driver(&env, "drv-for-seller@test.com").await;
    let (_, seller) = seed_seller(&env, "seller-authz@test.com").await;
    let missing = Uuid::now_v7();

    assert_forbidden(&env, "GET", "/v1/users", &seller, None).await;
    assert_forbidden(&env, "GET", &format!("/v1/users/{missing}"), &seller, None).await;
    assert_forbidden(
        &env,
        "PUT",
        &format!("/v1/users/{driver_id}/driver-profile"),
        &seller,
        Some(
            json!({
                "cnhNumber": "1",
                "cnhCategory": "B",
                "vehiclePlate": "AAA0A00",
                "vehicleModel": "X"
            })
            .to_string(),
        ),
    )
    .await;
}

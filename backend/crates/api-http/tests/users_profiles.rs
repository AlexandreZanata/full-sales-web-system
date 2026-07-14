//! Phase 17B — Driver/Seller profile contracts (T-17-008..009).

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{request, seed_admin, seed_driver, seed_seller, setup};

// T-17-008 — PUT /v1/users/{id}/driver-profile
#[tokio::test]
async fn given_admin_when_put_driver_profile_then_200() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let (driver_id, _) = seed_driver(&env, "driver-prof@test.com").await;

    let (status, body) = request(
        &env,
        "PUT",
        &format!("/v1/users/{driver_id}/driver-profile"),
        Some(&admin),
        Some(
            json!({
                "cnhNumber": "12345678900",
                "cnhCategory": "B",
                "vehiclePlate": "ABC1D23",
                "vehicleModel": "Fiat"
            })
            .to_string(),
        ),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["userId"], driver_id.to_string());
    assert_eq!(body["cnhNumber"], "12345678900");
    assert_eq!(body["vehiclePlate"], "ABC1D23");
}

// T-17-008
#[tokio::test]
async fn given_seller_user_when_put_driver_profile_then_400() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let (seller_id, _) = seed_seller(&env, "seller-not-driver@test.com").await;

    let (status, body) = request(
        &env,
        "PUT",
        &format!("/v1/users/{seller_id}/driver-profile"),
        Some(&admin),
        Some(
            json!({
                "cnhNumber": "1",
                "cnhCategory": "B",
                "vehiclePlate": "ABC1D23",
                "vehicleModel": "X"
            })
            .to_string(),
        ),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "INVALID_INPUT");
}

// T-17-009 — PUT /v1/users/{id}/seller-profile
#[tokio::test]
async fn given_admin_when_put_seller_profile_then_200() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let (seller_id, _) = seed_seller(&env, "seller-prof@test.com").await;

    let (status, body) = request(
        &env,
        "PUT",
        &format!("/v1/users/{seller_id}/seller-profile"),
        Some(&admin),
        Some(
            json!({
                "operatingRegion": "SP-capital",
                "monthlyTargetAmount": 50000
            })
            .to_string(),
        ),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["userId"], seller_id.to_string());
    assert_eq!(body["operatingRegion"], "SP-capital");
    assert_eq!(body["monthlyTargetAmount"], 50000);
}

// T-17-009
#[tokio::test]
async fn given_driver_user_when_put_seller_profile_then_400() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let (driver_id, _) = seed_driver(&env, "driver-not-seller@test.com").await;

    let (status, body) = request(
        &env,
        "PUT",
        &format!("/v1/users/{driver_id}/seller-profile"),
        Some(&admin),
        Some(json!({ "operatingRegion": "MG" }).to_string()),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "INVALID_INPUT");
}

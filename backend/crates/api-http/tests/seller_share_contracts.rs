//! Phase 19 — Public seller share resolve + seller-profile share fields.

#[path = "support/mod.rs"]
mod support;

use domain_shared::TenantId;
use http::StatusCode;
use serde_json::json;

use support::{request, seed_admin, seed_seller, setup_with_tenant};

const DEV_SEED_TENANT_ID: &str = "01900001-0000-7000-8000-000000000001";

async fn setup_public() -> support::TestEnv {
    let tenant_id = TenantId::parse(DEV_SEED_TENANT_ID).expect("dev seed tenant");
    setup_with_tenant(tenant_id).await
}

#[tokio::test]
async fn given_seller_with_code_when_public_get_then_200() {
    let env = setup_public().await;
    let (_, admin) = seed_admin(&env).await;
    let (seller_id, _) = seed_seller(&env, "share-seller@test.com").await;

    let (put_status, _) = request(
        &env,
        "PUT",
        &format!("/v1/users/{seller_id}/seller-profile"),
        Some(&admin),
        Some(
            json!({
                "operatingRegion": "SP",
                "publicCode": "maria-share",
                "contactPhone": "11999998888",
                "shareLinkActive": true
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(put_status, StatusCode::OK);

    let (status, body) = request(
        &env,
        "GET",
        "/v1/public/sellers/maria-share",
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["publicCode"], "maria-share");
    assert_eq!(body["contactPhone"], "11999998888");
    assert!(body["displayName"].as_str().is_some());
}

#[tokio::test]
async fn given_unknown_code_when_public_get_then_404() {
    let env = setup_public().await;
    let _ = seed_admin(&env).await;
    let (status, _) = request(&env, "GET", "/v1/public/sellers/does-not-exist", None, None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn given_inactive_share_when_public_get_then_404() {
    let env = setup_public().await;
    let (_, admin) = seed_admin(&env).await;
    let (seller_id, _) = seed_seller(&env, "inactive-share@test.com").await;

    let _ = request(
        &env,
        "PUT",
        &format!("/v1/users/{seller_id}/seller-profile"),
        Some(&admin),
        Some(
            json!({
                "publicCode": "off-seller",
                "shareLinkActive": false
            })
            .to_string(),
        ),
    )
    .await;

    let (status, _) = request(&env, "GET", "/v1/public/sellers/off-seller", None, None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn given_legacy_body_when_put_seller_profile_then_200_and_code_autofilled() {
    let env = setup_public().await;
    let (_, admin) = seed_admin(&env).await;
    let (seller_id, _) = seed_seller(&env, "legacy-prof@test.com").await;

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
    assert_eq!(body["operatingRegion"], "SP-capital");
    assert!(body["publicCode"].as_str().is_some());
    assert_eq!(body["shareLinkActive"], true);
}

#[tokio::test]
async fn given_seller_when_get_my_share_then_200() {
    let env = setup_public().await;
    let (_, admin) = seed_admin(&env).await;
    let (seller_id, seller_token) = seed_seller(&env, "me-share@test.com").await;

    let _ = request(
        &env,
        "PUT",
        &format!("/v1/users/{seller_id}/seller-profile"),
        Some(&admin),
        Some(json!({ "publicCode": "me-seller", "contactPhone": "11911112222" }).to_string()),
    )
    .await;

    let (status, body) = request(&env, "GET", "/v1/me/seller-share", Some(&seller_token), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["publicCode"], "me-seller");
    assert_eq!(body["sharePath"], "/s/me-seller");
    assert_eq!(body["shareUrl"], "http://127.0.0.1:5175/s/me-seller");
}

#[tokio::test]
async fn given_seller_when_patch_own_phone_then_200() {
    let env = setup_public().await;
    let (_, admin) = seed_admin(&env).await;
    let (seller_id, seller_token) = seed_seller(&env, "self-phone@test.com").await;

    let _ = request(
        &env,
        "PUT",
        &format!("/v1/users/{seller_id}/seller-profile"),
        Some(&admin),
        Some(json!({ "publicCode": "self-phone" }).to_string()),
    )
    .await;

    let (status, body) = request(
        &env,
        "PATCH",
        "/v1/me/seller-profile",
        Some(&seller_token),
        Some(json!({ "contactPhone": "11988887777" }).to_string()),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["contactPhone"], "11988887777");
    assert_eq!(body["publicCode"], "self-phone");

    let (get_status, get_body) =
        request(&env, "GET", "/v1/me/seller-profile", Some(&seller_token), None).await;
    assert_eq!(get_status, StatusCode::OK);
    assert_eq!(get_body["contactPhone"], "11988887777");
}

#[tokio::test]
async fn given_inactive_user_when_reactivate_then_active() {
    let env = setup_public().await;
    let (_, admin) = seed_admin(&env).await;
    let (seller_id, _) = seed_seller(&env, "reactivate-me@test.com").await;

    let (deact, _) = request(
        &env,
        "PATCH",
        &format!("/v1/users/{seller_id}/deactivate"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(deact, StatusCode::OK);

    let (status, body) = request(
        &env,
        "PATCH",
        &format!("/v1/users/{seller_id}/reactivate"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["active"], true);
}

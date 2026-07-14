//! Phase 68C — Commerces and addresses cursor list contract tests.

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;

use support::{request, seed_admin, seed_commerce, seed_driver, setup};

// T-17-011
#[tokio::test]
async fn contract_list_commerces_when_cursor_envelope_then_data_array() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;
    seed_commerce(&env, "11222333000181").await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/commerces?limit=20",
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["data"].is_array());
    assert_eq!(body["pagination"]["has_more"], false);
}

// T-17-011
#[tokio::test]
async fn contract_list_commerces_when_invalid_filter_then_400() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/commerces?filter[unknown]=x",
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "invalid_filter_field");
}

#[tokio::test]
async fn contract_list_commerces_when_limit_over_max_then_400() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/commerces?limit=200",
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "invalid_pagination");
}

// T-17-011 / T-17-015 — driver can GET list; cannot POST address
#[tokio::test]
async fn contract_driver_when_list_commerces_then_ok_but_post_address_forbidden() {
    let env = setup().await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let (_, driver_token) = seed_driver(&env, "driver@test.com").await;

    let (list_status, list_body) = request(
        &env,
        "GET",
        "/v1/commerces?limit=50",
        Some(&driver_token),
        None,
    )
    .await;
    assert_eq!(list_status, StatusCode::OK);
    assert!(list_body["data"].is_array());
    assert!(
        list_body["data"]
            .as_array()
            .unwrap()
            .iter()
            .any(|c| c["id"] == commerce_id.to_string())
    );

    let (post_status, post_body) = request(
        &env,
        "POST",
        &format!("/v1/commerces/{commerce_id}/addresses"),
        Some(&driver_token),
        Some(
            serde_json::json!({
                "addressType": "Delivery",
                "street": "Rua Nova",
                "number": "100",
                "city": "SP",
                "state": "SP",
                "postalCode": "01310100",
                "isPrimary": false
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(post_status, StatusCode::FORBIDDEN);
    assert_eq!(post_body["error"]["code"], "FORBIDDEN");
}

// T-17-014
#[tokio::test]
async fn contract_list_addresses_when_cursor_envelope_then_data_array() {
    let env = setup().await;
    let (_, admin_token) = seed_admin(&env).await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;

    let (status, body) = request(
        &env,
        "GET",
        &format!("/v1/commerces/{commerce_id}/addresses?limit=20"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["data"].is_array());
    assert_eq!(body["pagination"]["limit"], 20);
}

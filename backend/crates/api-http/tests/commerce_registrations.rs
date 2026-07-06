//! Contract tests: seller commerce registration workflow (Phase 69).

mod support;

use http::StatusCode;
use serde_json::{Value, json};
use support::{request, setup};

fn registration_payload(cnpj: &str) -> Value {
    json!({
        "cnpj": cnpj,
        "legalName": "Acme Comercio Ltda",
        "tradeName": "Acme Store",
        "contact": { "phone": "11999990000", "email": "acme@example.com" },
        "deliveryAddress": {
            "street": "Rua A",
            "number": "10",
            "district": "Centro",
            "city": "São Paulo",
            "state": "SP",
            "postalCode": "01001000",
            "isPrimary": true
        },
        "registrationMode": "manual"
    })
}

#[tokio::test]
async fn br_co_010_given_seller_when_submit_registration_then_pending() {
    let env = setup().await;
    let (_seller_id, token) = support::seed_seller(&env, "seller-reg@test.com").await;

    let (status, body) = request(
        &env,
        "POST",
        "/v1/commerces/registrations",
        Some(&token),
        Some(registration_payload("11222333000181").to_string()),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["registrationStatus"], "PendingReview");
    assert_eq!(body["active"], false);
}

#[tokio::test]
async fn br_co_011_given_seller_when_approve_registration_then_forbidden() {
    let env = setup().await;
    let (_seller_id, token) = support::seed_seller(&env, "seller-no-approve@test.com").await;

    let (create_status, created) = request(
        &env,
        "POST",
        "/v1/commerces/registrations",
        Some(&token),
        Some(registration_payload("11222333000181").to_string()),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let id = created["id"].as_str().expect("id");

    let (status, _) = request(
        &env,
        "POST",
        &format!("/v1/commerces/registrations/{id}/approve"),
        Some(&token),
        Some("{}".into()),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn contract_admin_when_approve_registration_then_active() {
    let env = setup().await;
    let (_seller_id, seller_token) = support::seed_seller(&env, "seller-approve@test.com").await;
    let (_admin_id, admin_token) = support::seed_admin(&env).await;

    let (_, created) = request(
        &env,
        "POST",
        "/v1/commerces/registrations",
        Some(&seller_token),
        Some(registration_payload("11222333000181").to_string()),
    )
    .await;
    let id = created["id"].as_str().expect("id");

    let (status, body) = request(
        &env,
        "POST",
        &format!("/v1/commerces/registrations/{id}/approve"),
        Some(&admin_token),
        Some("{}".into()),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["registrationStatus"], "Active");
    assert_eq!(body["active"], true);
}

#[tokio::test]
async fn contract_lookup_when_known_cnpj_then_prefill() {
    let env = setup().await;
    let (_seller_id, token) = support::seed_seller(&env, "seller-lookup@test.com").await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/commerces/cnpj-lookup?cnpj=11222333000181",
        Some(&token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["legalName"], "Acme Comercio Ltda");
}

#[tokio::test]
async fn contract_duplicate_cnpj_when_submit_then_conflict() {
    let env = setup().await;
    let (_seller_id, token) = support::seed_seller(&env, "seller-dup@test.com").await;

    support::seed_commerce(&env, "11222333000181").await;

    let (status, body) = request(
        &env,
        "POST",
        "/v1/commerces/registrations",
        Some(&token),
        Some(registration_payload("11222333000181").to_string()),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body["error"]["code"], "CNPJ_ALREADY_REGISTERED");
}

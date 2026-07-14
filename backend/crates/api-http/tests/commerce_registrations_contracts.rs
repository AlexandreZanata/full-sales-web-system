//! Phase 17C — Commerce registrations review flows (T-17-019..024).

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::{Value, json};

use support::{request, seed_admin, seed_seller, setup};

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

async fn submit(env: &support::TestEnv, token: &str, cnpj: &str) -> (StatusCode, Value) {
    request(
        env,
        "POST",
        "/v1/commerces/registrations",
        Some(token),
        Some(registration_payload(cnpj).to_string()),
    )
    .await
}

// T-17-019
#[tokio::test]
async fn given_seller_when_submit_registration_then_201_pending_review() {
    let env = setup().await;
    let (_, token) = seed_seller(&env, "seller-reg-17c@test.com").await;
    let (status, body) = submit(&env, &token, "11222333000181").await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["registrationStatus"], "PendingReview");
    assert_eq!(body["active"], true);
}

// T-17-020 / T-17-021
#[tokio::test]
async fn given_seller_when_list_and_get_own_registration_then_200() {
    let env = setup().await;
    let (_, token) = seed_seller(&env, "seller-list-reg@test.com").await;
    let (create_status, created) = submit(&env, &token, "11222333000181").await;
    assert_eq!(create_status, StatusCode::CREATED);
    let id = created["id"].as_str().expect("id");

    let (list_status, list) = request(
        &env,
        "GET",
        "/v1/commerces/registrations?limit=20",
        Some(&token),
        None,
    )
    .await;
    assert_eq!(list_status, StatusCode::OK);
    assert!(
        list["data"]
            .as_array()
            .expect("data")
            .iter()
            .any(|r| r["id"] == id)
    );

    let (get_status, got) = request(
        &env,
        "GET",
        &format!("/v1/commerces/registrations/{id}"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(get_status, StatusCode::OK);
    assert_eq!(got["id"], id);
}

// T-17-022
#[tokio::test]
async fn given_seller_when_patch_pending_registration_then_200() {
    let env = setup().await;
    let (_, token) = seed_seller(&env, "seller-patch-reg@test.com").await;
    let (_, created) = submit(&env, &token, "11222333000181").await;
    let id = created["id"].as_str().expect("id");

    let (status, body) = request(
        &env,
        "PATCH",
        &format!("/v1/commerces/registrations/{id}"),
        Some(&token),
        Some(json!({ "legalName": "Acme Atualizada Ltda", "tradeName": "Acme Novo" }).to_string()),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["legalName"], "Acme Atualizada Ltda");
    assert_eq!(body["tradeName"], "Acme Novo");
}

// T-17-023
#[tokio::test]
async fn given_admin_when_approve_pending_then_200_active() {
    let env = setup().await;
    let (_, seller) = seed_seller(&env, "seller-appr@test.com").await;
    let (_, admin) = seed_admin(&env).await;
    let (_, created) = submit(&env, &seller, "11222333000181").await;
    let id = created["id"].as_str().expect("id");

    let (status, body) = request(
        &env,
        "POST",
        &format!("/v1/commerces/registrations/{id}/approve"),
        Some(&admin),
        Some("{}".into()),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["registrationStatus"], "Active");
    assert_eq!(body["active"], true);
}

// T-17-024
#[tokio::test]
async fn given_admin_when_reject_pending_then_200_rejected() {
    let env = setup().await;
    let (_, seller) = seed_seller(&env, "seller-rej@test.com").await;
    let (_, admin) = seed_admin(&env).await;
    let (_, created) = submit(&env, &seller, "11222333000181").await;
    let id = created["id"].as_str().expect("id");

    let (status, body) = request(
        &env,
        "POST",
        &format!("/v1/commerces/registrations/{id}/reject"),
        Some(&admin),
        Some(json!({ "reason": "Incomplete docs" }).to_string()),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["registrationStatus"], "Rejected");
}

// T-17-019 authz
#[tokio::test]
async fn given_driver_when_submit_registration_then_403() {
    let env = setup().await;
    let (_, driver) = support::seed_driver(&env, "driver-reg@test.com").await;
    let (status, body) = submit(&env, &driver, "11222333000181").await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body["error"]["code"], "FORBIDDEN");
}

//! Phase 17C — Commerces Admin-only authz (T-17-010..017).

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{request, seed_commerce, seed_commerce_contact, seed_driver, setup};

// T-17-010 / T-17-013 / T-17-015 / T-17-017
#[tokio::test]
async fn given_driver_when_admin_commerce_mutations_then_403() {
    let env = setup().await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let (_, driver) = seed_driver(&env, "driver-co-authz@test.com").await;

    for (method, uri, body) in [
        (
            "POST",
            "/v1/commerces".to_owned(),
            Some(
                json!({
                    "cnpj": "11222333000181",
                    "legalName": "X Y",
                    "address": { "city": "SP" },
                    "contact": { "email": "x@y.com" }
                })
                .to_string(),
            ),
        ),
        (
            "PATCH",
            format!("/v1/commerces/{commerce_id}/deactivate"),
            None,
        ),
        (
            "POST",
            format!("/v1/commerces/{commerce_id}/addresses"),
            Some(
                json!({
                    "addressType": "Billing",
                    "street": "Rua",
                    "number": "1",
                    "city": "SP",
                    "state": "SP",
                    "postalCode": "01001000"
                })
                .to_string(),
            ),
        ),
        (
            "PUT",
            format!("/v1/commerces/{commerce_id}/logo"),
            Some(json!({ "fileId": uuid::Uuid::now_v7() }).to_string()),
        ),
    ] {
        let (status, resp) = request(&env, method, &uri, Some(&driver), body).await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{method} {uri}");
        assert_eq!(resp["error"]["code"], "FORBIDDEN");
    }
}

// T-17-011 / T-17-012 — CommerceContact cannot read admin commerce list
#[tokio::test]
async fn given_commerce_contact_when_list_commerces_then_403() {
    let env = setup().await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let (_, token) = seed_commerce_contact(&env, commerce_id, "contact-co@test.com").await;

    let (status, body) = request(&env, "GET", "/v1/commerces", Some(&token), None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body["error"]["code"], "FORBIDDEN");
}

// T-17-018
#[tokio::test]
async fn given_no_token_when_cnpj_lookup_then_401() {
    let env = setup().await;
    let (status, body) = request(
        &env,
        "GET",
        "/v1/commerces/cnpj-lookup?cnpj=11222333000181",
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["code"], "UNAUTHORIZED");
}

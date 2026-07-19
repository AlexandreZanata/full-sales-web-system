//! Phase 17C — Commerces CRUD / addresses / logo (T-17-010..017).
//! T-17-173 PATCH /v1/commerces/{id}/activate

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use support::{minimal_webp_bytes, request, seed_admin, seed_commerce, setup, upload_multipart};

fn commerce_body(cnpj: &str) -> String {
    json!({
        "cnpj": cnpj,
        "legalName": "Acme Ltda",
        "tradeName": "Acme",
        "address": { "city": "SP", "state": "SP" },
        "contact": { "email": "a@acme.com" }
    })
    .to_string()
}

// T-17-010
#[tokio::test]
async fn given_admin_when_post_commerce_then_201() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let (status, body) = request(
        &env,
        "POST",
        "/v1/commerces",
        Some(&admin),
        Some(commerce_body("11222333000181")),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["cnpj"], "11222333000181");
    assert_eq!(body["legalName"], "Acme Ltda");
}

// T-17-010
#[tokio::test]
async fn given_no_token_when_post_commerce_then_401() {
    let env = setup().await;
    let (status, body) = request(
        &env,
        "POST",
        "/v1/commerces",
        None,
        Some(commerce_body("11222333000181")),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["code"], "UNAUTHORIZED");
}

// T-17-012
#[tokio::test]
async fn given_admin_when_get_commerce_then_200() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let id = seed_commerce(&env, "11222333000181").await;
    let (status, body) = request(
        &env,
        "GET",
        &format!("/v1/commerces/{id}"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["id"], id.to_string());
}

// T-17-012
#[tokio::test]
async fn given_unknown_id_when_get_commerce_then_404() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let missing = Uuid::now_v7();
    let (status, body) = request(
        &env,
        "GET",
        &format!("/v1/commerces/{missing}"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "COMMERCE_NOT_FOUND");
}

// T-17-013
#[tokio::test]
async fn given_admin_when_deactivate_commerce_then_200_inactive() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let id = seed_commerce(&env, "11222333000181").await;
    let (status, body) = request(
        &env,
        "PATCH",
        &format!("/v1/commerces/{id}/deactivate"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["active"], false);
}

#[tokio::test]
async fn given_admin_when_activate_inactive_commerce_then_200_active() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let id = seed_commerce(&env, "11222333000181").await;
    let (deact_status, _) = request(
        &env,
        "PATCH",
        &format!("/v1/commerces/{id}/deactivate"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(deact_status, StatusCode::OK);

    let (status, body) = request(
        &env,
        "PATCH",
        &format!("/v1/commerces/{id}/activate"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["active"], true);
}

// T-17-015 / T-17-016
#[tokio::test]
async fn given_admin_when_create_and_patch_address_then_ok() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;

    let (create_status, created) = request(
        &env,
        "POST",
        &format!("/v1/commerces/{commerce_id}/addresses"),
        Some(&admin),
        Some(
            json!({
                "addressType": "Delivery",
                "street": "Rua A",
                "number": "10",
                "district": "Centro",
                "city": "SP",
                "state": "SP",
                "postalCode": "01001000",
                "isPrimary": true
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let address_id = created["id"].as_str().expect("id");

    let (patch_status, patched) = request(
        &env,
        "PATCH",
        &format!("/v1/commerces/{commerce_id}/addresses/{address_id}"),
        Some(&admin),
        Some(json!({ "street": "Rua B", "number": "20" }).to_string()),
    )
    .await;
    assert_eq!(patch_status, StatusCode::OK);
    assert_eq!(patched["street"], "Rua B");
    assert_eq!(patched["number"], "20");
}

// T-17-017
#[tokio::test]
async fn given_admin_when_put_commerce_logo_then_200() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let commerce_id = seed_commerce(&env, "11222333000181").await;
    let webp = minimal_webp_bytes();
    let (upload_status, upload) = upload_multipart(
        &env,
        &admin,
        "logo.webp",
        "image/webp",
        &webp,
        "Commerce",
        commerce_id,
    )
    .await;
    assert!(
        upload_status == StatusCode::OK || upload_status == StatusCode::CREATED,
        "{upload_status} {upload}"
    );
    let file_id = upload["id"].as_str().expect("file id");

    let (status, body) = request(
        &env,
        "PUT",
        &format!("/v1/commerces/{commerce_id}/logo"),
        Some(&admin),
        Some(json!({ "fileId": file_id }).to_string()),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["id"], commerce_id.to_string());
}

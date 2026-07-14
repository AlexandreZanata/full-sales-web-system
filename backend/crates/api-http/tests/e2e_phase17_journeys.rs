//! Phase 17K — Multistep journeys (registration; media → settings logo).
//! Companion journeys: `e2e.rs` (sale), `e2e_journeys.rs` (portal→report),
//! `platform_saas` (provision→suspend→webhook), `tenant_domains`, fraud/blocklist.
#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{minimal_webp_bytes, request, seed_admin, seed_seller, setup, upload_multipart};

fn registration_payload(cnpj: &str) -> String {
    json!({
        "cnpj": cnpj,
        "legalName": "17K Comercio LTDA",
        "tradeName": "17K Shop",
        "contact": { "phone": "11999990000", "email": "contact-17k@test.com" },
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
    .to_string()
}

// Registration submit → approve → list commerces
#[tokio::test]
async fn e2e_17k_registration_approve_list_commerces() {
    let env = setup().await;
    let (_, seller) = seed_seller(&env, "seller-17k@test.com").await;
    let (_, admin) = seed_admin(&env).await;

    let (create_st, created) = request(
        &env,
        "POST",
        "/v1/commerces/registrations",
        Some(&seller),
        Some(registration_payload("11444777000161")),
    )
    .await;
    assert_eq!(create_st, StatusCode::CREATED, "{created}");
    assert_eq!(created["registrationStatus"], "PendingReview");
    let id = created["id"].as_str().expect("id");

    let (ap_st, ap) = request(
        &env,
        "POST",
        &format!("/v1/commerces/registrations/{id}/approve"),
        Some(&admin),
        Some("{}".into()),
    )
    .await;
    assert_eq!(ap_st, StatusCode::OK, "{ap}");
    assert_eq!(ap["registrationStatus"], "Active");

    let (list_st, list) = request(&env, "GET", "/v1/commerces?limit=50", Some(&admin), None).await;
    assert_eq!(list_st, StatusCode::OK, "{list}");
    let rows = list["data"].as_array().or_else(|| list["items"].as_array());
    assert!(rows.is_some_and(|r| !r.is_empty()), "{list}");
}

// Media upload → settings logo
#[tokio::test]
async fn e2e_17k_media_upload_settings_logo() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let tenant = env.tenant_id.as_uuid();
    let (up_st, up) = upload_multipart(
        &env,
        &admin,
        "logo.webp",
        "image/webp",
        &minimal_webp_bytes(),
        "Tenant",
        tenant,
    )
    .await;
    assert!(up_st.is_success(), "{up}");
    let file_id = up["id"].as_str().expect("file");

    let (logo_st, logo) = request(
        &env,
        "PUT",
        "/v1/settings/logo",
        Some(&admin),
        Some(json!({ "fileId": file_id }).to_string()),
    )
    .await;
    assert_eq!(logo_st, StatusCode::OK, "{logo}");
    assert_eq!(logo["logoFileId"], file_id);
}

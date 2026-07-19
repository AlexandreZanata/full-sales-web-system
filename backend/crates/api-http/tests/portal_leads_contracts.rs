//! Portal commerce lead contracts (public submit + admin review).

#[path = "support/mod.rs"]
mod support;

use domain_shared::TenantId;
use http::StatusCode;
use serde_json::json;

use support::{request, seed_admin, setup_with_tenant};

const DEV_SEED_TENANT_ID: &str = "01900001-0000-7000-8000-000000000001";

async fn setup_public() -> support::TestEnv {
    let tenant_id = TenantId::parse(DEV_SEED_TENANT_ID).expect("dev seed tenant");
    setup_with_tenant(tenant_id).await
}

#[tokio::test]
async fn given_valid_lead_when_public_post_then_201() {
    let env = setup_public().await;
    let (status, body) = request(
        &env,
        "POST",
        "/v1/public/commerce-leads",
        None,
        Some(
            json!({
                "contactName": "Ana Silva",
                "phone": "11987654321",
                "commerceName": "Mercado Ana",
                "email": "ana@mercado.test"
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["status"], "pending");
    assert_eq!(body["contactName"], "Ana Silva");
}

#[tokio::test]
async fn given_lead_when_admin_approves_then_approved() {
    let env = setup_public().await;
    let (_, admin) = seed_admin(&env).await;

    let (create_status, created) = request(
        &env,
        "POST",
        "/v1/public/commerce-leads",
        None,
        Some(
            json!({
                "contactName": "Bruno",
                "phone": "11911112222",
                "commerceName": "Loja Bruno",
                "email": "bruno@loja.test"
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(create_status, StatusCode::CREATED);
    let id = created["id"].as_str().expect("id");

    let (list_status, list) = request(
        &env,
        "GET",
        "/v1/commerces/portal-leads?status=pending",
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(list_status, StatusCode::OK);
    assert!(
        list.as_array()
            .expect("array")
            .iter()
            .any(|row| row["id"] == id)
    );

    let (patch_status, patched) = request(
        &env,
        "PATCH",
        &format!("/v1/commerces/portal-leads/{id}"),
        Some(&admin),
        Some(json!({ "status": "approved" }).to_string()),
    )
    .await;
    assert_eq!(patch_status, StatusCode::OK);
    assert_eq!(patched["status"], "approved");
}

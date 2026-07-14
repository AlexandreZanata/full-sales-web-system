//! Phase 17H — Settings + logo + LGPD export (T-17-089..092, T-17-142..143).
#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{minimal_webp_bytes, request, seed_admin, seed_driver, setup, upload_multipart};

// T-17-089 / T-17-091 / T-17-092
#[tokio::test]
async fn given_admin_when_settings_and_logo_then_ok() {
    let env = setup().await;
    let (admin_id, admin) = seed_admin(&env).await;
    let tenant_id = env.tenant_id.as_uuid();

    let (get_st, got) = request(&env, "GET", "/v1/settings", Some(&admin), None).await;
    assert_eq!(get_st, StatusCode::OK);
    assert!(got["displayName"].is_string());

    let (patch_st, patched) = request(
        &env,
        "PATCH",
        "/v1/settings",
        Some(&admin),
        Some(json!({ "displayName": "17H Tenant" }).to_string()),
    )
    .await;
    assert_eq!(patch_st, StatusCode::OK);
    assert_eq!(patched["displayName"], "17H Tenant");

    let (up_st, up) = upload_multipart(
        &env,
        &admin,
        "logo.webp",
        "image/webp",
        &minimal_webp_bytes(),
        "Tenant",
        tenant_id,
    )
    .await;
    assert!(up_st.is_success(), "{up}");
    let file_id = up["id"].as_str().expect("file");
    let _ = admin_id;

    let (logo_st, logo) = request(
        &env,
        "PUT",
        "/v1/settings/logo",
        Some(&admin),
        Some(json!({ "fileId": file_id }).to_string()),
    )
    .await;
    assert_eq!(logo_st, StatusCode::OK);
    assert_eq!(logo["logoFileId"], file_id);
}

// T-17-091 authz / T-17-142 / T-17-143
#[tokio::test]
async fn given_driver_or_export_when_settings_then_expected() {
    let env = setup().await;
    let (_, admin) = seed_admin(&env).await;
    let (_, driver) = seed_driver(&env, "settings-drv@test.com").await;

    let (drv_st, drv) = request(
        &env,
        "PATCH",
        "/v1/settings",
        Some(&driver),
        Some(json!({ "displayName": "Nope" }).to_string()),
    )
    .await;
    assert_eq!(drv_st, StatusCode::FORBIDDEN);
    assert_eq!(drv["error"]["code"], "FORBIDDEN");

    let (start_st, job) =
        request(&env, "POST", "/v1/settings/data-export", Some(&admin), None).await;
    assert_eq!(start_st, StatusCode::ACCEPTED);
    let job_id = job["id"].as_str().expect("job id");

    let (get_st, status_body) = request(
        &env,
        "GET",
        &format!("/v1/settings/data-export/{job_id}"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(get_st, StatusCode::OK);
    assert!(status_body["status"].is_string());

    let (denied, denied_body) = request(
        &env,
        "POST",
        "/v1/settings/data-export",
        Some(&driver),
        None,
    )
    .await;
    assert_eq!(denied, StatusCode::FORBIDDEN);
    assert_eq!(denied_body["error"]["code"], "FORBIDDEN");
}

//! Phase 17I — Reports contracts (T-17-095..099).
#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use support::{request, seed_admin, seed_driver, seed_signing_key, setup};

async fn create_report(env: &support::TestEnv, admin: &str, driver_id: Uuid) -> String {
    let (st, body) = request(
        env,
        "POST",
        "/v1/reports",
        Some(admin),
        Some(
            json!({
                "reportType": "DailyDriver",
                "periodStart": "2026-01-01T00:00:00Z",
                "periodEnd": "2026-01-31T23:59:59Z",
                "driverId": driver_id
            })
            .to_string(),
        ),
    )
    .await;
    assert_eq!(st, StatusCode::CREATED);
    body["id"].as_str().expect("id").to_owned()
}

// T-17-095 / T-17-096 / T-17-097 / T-17-098 / T-17-099
#[tokio::test]
async fn given_admin_when_report_lifecycle_then_ok() {
    let env = setup().await;
    seed_signing_key(&env).await;
    let (_, admin) = seed_admin(&env).await;
    let (driver_id, _) = seed_driver(&env, "rep-17i@test.com").await;
    let id = create_report(&env, &admin, driver_id).await;

    let (list_st, list) = request(
        &env,
        "GET",
        "/v1/reports?page=1&pageSize=10",
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(list_st, StatusCode::OK);
    assert!(
        list["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["id"] == id)
    );

    let (get_st, got) = request(
        &env,
        "GET",
        &format!("/v1/reports/{id}"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(get_st, StatusCode::OK);
    assert_eq!(got["id"], id);

    let (export_st, _) = request(
        &env,
        "GET",
        &format!("/v1/reports/{id}/export?format=csv"),
        Some(&admin),
        None,
    )
    .await;
    assert!(export_st.is_success(), "{export_st}");

    let (verify_st, verify) =
        request(&env, "GET", &format!("/v1/reports/{id}/verify"), None, None).await;
    assert_eq!(verify_st, StatusCode::OK);
    assert_eq!(verify["valid"], true);
}

// T-17-097 / T-17-098 errors + authz
#[tokio::test]
async fn given_report_when_not_found_or_driver_list_then_errors() {
    let env = setup().await;
    seed_signing_key(&env).await;
    let (_, admin) = seed_admin(&env).await;
    let (_, driver) = seed_driver(&env, "rep-drv@test.com").await;
    let missing = Uuid::now_v7();

    let (nf_st, nf) = request(
        &env,
        "GET",
        &format!("/v1/reports/{missing}"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(nf_st, StatusCode::NOT_FOUND);
    assert_eq!(nf["error"]["code"], "REPORT_NOT_FOUND");

    let (drv_st, drv) = request(&env, "GET", "/v1/reports", Some(&driver), None).await;
    assert_eq!(drv_st, StatusCode::FORBIDDEN);
    assert_eq!(drv["error"]["code"], "FORBIDDEN");

    let (export_st, export) = request(
        &env,
        "GET",
        &format!("/v1/reports/{missing}/export?format=csv"),
        Some(&admin),
        None,
    )
    .await;
    assert_eq!(export_st, StatusCode::NOT_FOUND);
    assert_eq!(export["error"]["code"], "REPORT_NOT_FOUND");
}

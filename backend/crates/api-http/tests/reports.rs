//! Phase 24 — Reports contract tests (E2E-002).

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use serde_json::json;

use support::{request, seed_admin, seed_driver, seed_signing_key, setup};

// E2E-002: generate → verify valid true
#[tokio::test]
async fn e2e_002_generate_report_when_verify_then_valid_true() {
    let env = setup().await;
    seed_signing_key(&env).await;
    let (_, admin_token) = seed_admin(&env).await;
    let (driver_id, _) = seed_driver(&env, "driver@test.com").await;

    let (gen_status, gen_body) = request(
        &env,
        "POST",
        "/v1/reports",
        Some(&admin_token),
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
    assert_eq!(gen_status, StatusCode::CREATED);
    let report_id = gen_body["id"].as_str().expect("report id");

    let (verify_status, verify_body) = request(
        &env,
        "GET",
        &format!("/v1/reports/{report_id}/verify"),
        None,
        None,
    )
    .await;

    assert_eq!(verify_status, StatusCode::OK);
    assert_eq!(verify_body["valid"], true);
    assert_eq!(verify_body["reportId"], report_id);
}

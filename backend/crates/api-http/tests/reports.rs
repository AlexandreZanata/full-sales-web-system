//! Phase 24 — Reports contract tests (E2E-002).

#[path = "support/mod.rs"]
mod support;

use axum::body::Body;
use http::{Request, StatusCode};
use serde_json::json;
use tower::ServiceExt;

use api_http::full_app;
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

#[tokio::test]
async fn contract_export_report_csv_when_generated_then_contains_driver_and_sale_count() {
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

    let response = full_app(env.state.clone())
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/v1/reports/{report_id}/export?format=csv"))
                .header("authorization", format!("Bearer {admin_token}"))
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get("content-type")
            .and_then(|value| value.to_str().ok()),
        Some("text/csv; charset=utf-8")
    );

    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("bytes");
    let text = String::from_utf8(bytes.to_vec()).expect("utf8");
    assert!(text.contains(&format!("driverId,{driver_id}")));
    assert!(text.contains("salesCount,"));
}

#[tokio::test]
async fn contract_export_report_when_unknown_format_then_unsupported_format() {
    let env = setup().await;
    seed_signing_key(&env).await;
    let (_, admin_token) = seed_admin(&env).await;
    let (driver_id, _) = seed_driver(&env, "driver@test.com").await;

    let (_, gen_body) = request(
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
    let report_id = gen_body["id"].as_str().expect("report id");

    let (status, body) = request(
        &env,
        "GET",
        &format!("/v1/reports/{report_id}/export?format=doc"),
        Some(&admin_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "UNSUPPORTED_FORMAT");
}

#[tokio::test]
async fn contract_list_reports_when_admin_then_offset_envelope() {
    let env = setup().await;
    seed_signing_key(&env).await;
    let (_, admin_token) = seed_admin(&env).await;
    let (driver_id, _) = seed_driver(&env, "driver-reports-list@test.com").await;

    let (gen_status, _) = request(
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

    let (status, body) = request(
        &env,
        "GET",
        "/v1/reports?page=1&pageSize=10",
        Some(&admin_token),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert!(body["items"].is_array());
    assert_eq!(body["page"], 1);
    assert_eq!(body["pageSize"], 10);
    assert!(body["total"].as_u64().unwrap_or(0) >= 1);
}

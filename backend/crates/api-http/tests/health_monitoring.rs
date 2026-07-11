//! Phase 9 — system health monitoring contract tests.

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use support::{platform_access_token, request, seed_platform_admin, setup};

#[tokio::test]
async fn contract_ready_when_all_up_then_returns_200() {
    let env = setup().await;
    let (status, body) = request(&env, "GET", "/health/ready", None, None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "ready");
    assert_eq!(body["components"]["postgres"]["status"], "up");
}

#[tokio::test]
async fn contract_ready_when_postgres_down_then_returns_503() {
    let env = setup().await;
    env.admin_pool.close().await;
    env.app_pool.close().await;

    let (status, body) = request(&env, "GET", "/health/ready", None, None).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE, "{body}");
    assert_eq!(body["status"], "not_ready");
    assert_eq!(body["components"]["postgres"]["status"], "down");
}

#[tokio::test]
async fn contract_platform_health_matrix_when_authenticated_then_returns_probes() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let token = platform_access_token(&env).await;

    let (status, body) = request(
        &env,
        "GET",
        "/v1/platform/health/matrix",
        Some(&token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert!(body["probes"]["postgres"].is_object(), "{body}");
    assert!(
        body["probes"]["postgres"]["uptime24hPct"].is_number(),
        "{body}"
    );
}

#[tokio::test]
async fn contract_public_status_when_get_then_returns_components() {
    let env = setup().await;
    let (status, body) = request(&env, "GET", "/v1/status", None, None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert!(body["components"]["api"].is_object(), "{body}");
    assert!(body["components"]["payments"].is_object(), "{body}");
}

#[tokio::test]
async fn contract_health_history_when_valid_probe_then_returns_series() {
    let env = setup().await;
    seed_platform_admin(&env).await;
    let token = platform_access_token(&env).await;

    api_http::health::run_probe_cycle(
        &env.state,
        &api_http::health::AlertConfig {
            webhook_url: None,
            consecutive_threshold: 3,
        },
    )
    .await
    .expect("probe cycle");

    let since_raw = (chrono::Utc::now() - chrono::Duration::hours(1))
        .format("%Y-%m-%dT%H:%M:%SZ")
        .to_string();
    let since = since_raw.replace(':', "%3A");
    let path = format!("/v1/platform/health/history?probe=postgres&since={since}");
    let (status, body) = request(&env, "GET", &path, Some(&token), None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["probe"], "postgres");
    assert!(
        body["data"].as_array().is_some_and(|rows| !rows.is_empty()),
        "{body}"
    );
}

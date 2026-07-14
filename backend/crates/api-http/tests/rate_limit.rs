//! Phase 26 — Rate limiting contract tests (TS-E2E-003 / verify public endpoint).

#[path = "support/mod.rs"]
mod support;

use http::StatusCode;
use infra_redis::RateLimitPolicy;
use serde_json::json;
use std::time::Duration;
use uuid::Uuid;

use support::{request_with_headers, seed_admin, setup};

async fn login_with_ip(
    env: &support::TestEnv,
    ip: &str,
    email: &str,
    password: &str,
) -> (StatusCode, serde_json::Value) {
    request_with_headers(
        env,
        "POST",
        "/v1/auth/login",
        None,
        Some(json!({ "email": email, "password": password }).to_string()),
        &[("x-forwarded-for", ip)],
    )
    .await
}

// T-17-001 — Contract: TS-E2E-003 — login rate limit after N failures
#[tokio::test]
async fn given_repeated_login_failures_when_exceeds_limit_then_rate_limited() {
    let env = setup().await;
    let _ = seed_admin(&env).await;
    let ip = "203.0.113.10";

    for _ in 0..5 {
        let (status, body) = login_with_ip(&env, ip, "admin@test.com", "wrong-password").await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(body["error"]["code"], "INVALID_CREDENTIALS");
    }

    let (status, body) = login_with_ip(&env, ip, "admin@test.com", "wrong-password").await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(body["error"]["code"], "RATE_LIMITED");
}

// Contract: verify endpoint rate limited by IP (ADR-007)
#[tokio::test]
async fn given_repeated_verify_requests_when_exceeds_limit_then_rate_limited() {
    let mut env = setup().await;
    env.state.verify_rate_limit = RateLimitPolicy {
        max: 3,
        window: Duration::from_secs(60),
    };
    let ip = "203.0.113.20";
    let report_id = Uuid::now_v7();

    for _ in 0..3 {
        let (status, _) = request_with_headers(
            &env,
            "GET",
            &format!("/v1/reports/{report_id}/verify"),
            None,
            None,
            &[("x-forwarded-for", ip)],
        )
        .await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    let (status, body) = request_with_headers(
        &env,
        "GET",
        &format!("/v1/reports/{report_id}/verify"),
        None,
        None,
        &[("x-forwarded-for", ip)],
    )
    .await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(body["error"]["code"], "RATE_LIMITED");
}

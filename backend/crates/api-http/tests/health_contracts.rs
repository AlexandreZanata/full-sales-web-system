//! Phase 17I — Health probes (T-17-102..104, T-17-162).
#[path = "support/mod.rs"]
mod support;

use http::StatusCode;

use support::{request, setup};

// T-17-102 / T-17-103 / T-17-104 / T-17-162
#[tokio::test]
async fn given_no_auth_when_health_ready_status_v1_then_200() {
    let env = setup().await;

    let (h_st, h) = request(&env, "GET", "/health", None, None).await;
    assert_eq!(h_st, StatusCode::OK);
    assert_eq!(h["status"], "ok");

    let (r_st, r) = request(&env, "GET", "/health/ready", None, None).await;
    assert!(
        r_st == StatusCode::OK || r_st == StatusCode::SERVICE_UNAVAILABLE,
        "{r}"
    );
    assert!(r["status"].is_string());
    assert!(r["components"].is_object());

    let (s_st, s) = request(&env, "GET", "/v1/status", None, None).await;
    assert_eq!(s_st, StatusCode::OK);
    assert!(s["status"].is_string() || s.is_object());

    let (v_st, v) = request(&env, "GET", "/v1/", None, None).await;
    assert_eq!(v_st, StatusCode::OK);
    assert_eq!(v["version"], "1");
}

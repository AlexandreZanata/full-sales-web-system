//! Integration tests for HTTP contract (API-CONTRACT.md).

use api_http::app;
use axum::body::Body;
use http::{Request, StatusCode};
use tower::ServiceExt;

// Contract: GET /health → 200 { "status": "ok" }
#[tokio::test]
async fn contract_health_when_get_then_returns_200_ok() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .expect("valid request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body bytes");
    let json: serde_json::Value = serde_json::from_slice(&body).expect("json body");
    assert_eq!(json["status"], "ok");
}

// Contract: every response includes x-request-id for tracing correlation.
#[tokio::test]
async fn contract_health_when_get_then_includes_request_id_header() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .expect("valid request"),
        )
        .await
        .expect("response");

    assert!(
        response.headers().get("x-request-id").is_some(),
        "x-request-id header must be present"
    );
}

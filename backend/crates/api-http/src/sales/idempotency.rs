use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    response::Response,
};
use infra_redis::IdempotencyRecord;

use crate::error::ApiError;

pub fn idempotency_key(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Idempotency-Key")
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .filter(|k| !k.is_empty())
        .map(str::to_owned)
}

pub fn replay_idempotency(record: IdempotencyRecord) -> Result<Response, ApiError> {
    let mut builder = Response::builder().status(record.status_code);
    if let Some(location) = record.location
        && let Ok(value) = HeaderValue::from_str(&location)
    {
        builder = builder.header(http::header::LOCATION, value);
    }
    builder
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(axum::body::Body::from(record.body))
        .map_err(|_| ApiError::internal())
}

pub fn created_json_response(body_bytes: Vec<u8>, location: &str) -> Result<Response, ApiError> {
    let mut response_builder = Response::builder().status(StatusCode::CREATED);
    if let Ok(value) = HeaderValue::from_str(location) {
        response_builder = response_builder.header(http::header::LOCATION, value);
    }
    response_builder
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(axum::body::Body::from(body_bytes))
        .map_err(|_| ApiError::internal())
}

use axum::{
    Json,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;

/// RFC 9457 / API-CONTRACT.md error envelope.
#[derive(Debug, Clone, Serialize)]
pub struct ApiErrorBody {
    pub error: ApiErrorDetail,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApiErrorDetail {
    pub code: String,
    pub message: String,
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
}

#[derive(Debug, Clone)]
pub struct ApiError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: &'static str,
}

impl ApiError {
    pub fn not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "NOT_FOUND",
            message: "Resource not found",
        }
    }

    pub fn into_response(self, correlation_id: String) -> Response {
        let body = ApiErrorBody {
            error: ApiErrorDetail {
                code: self.code.to_owned(),
                message: self.message.to_owned(),
                correlation_id,
            },
        };
        (self.status, Json(body)).into_response()
    }
}

pub async fn not_found_handler(headers: HeaderMap) -> Response {
    correlation_id_from_headers(&headers)
        .map(|id| ApiError::not_found().into_response(id))
        .unwrap_or_else(|| ApiError::not_found().into_response("unknown".to_owned()))
}

fn correlation_id_from_headers(headers: &HeaderMap) -> Option<String> {
    headers
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_not_found_error_when_serialized_then_contract_shape() {
        let body = ApiErrorBody {
            error: ApiErrorDetail {
                code: "NOT_FOUND".into(),
                message: "Resource not found".into(),
                correlation_id: "550e8400-e29b-41d4-a716-446655440000".into(),
            },
        };
        let json = serde_json::to_value(body).expect("serialize");
        assert_eq!(json["error"]["code"], "NOT_FOUND");
        assert_eq!(
            json["error"]["correlationId"],
            "550e8400-e29b-41d4-a716-446655440000"
        );
    }
}

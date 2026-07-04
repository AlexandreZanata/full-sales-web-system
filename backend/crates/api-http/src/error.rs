use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

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

    pub fn unauthorized() -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            code: "UNAUTHORIZED",
            message: "Authentication required",
        }
    }

    pub fn invalid_credentials() -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            code: "INVALID_CREDENTIALS",
            message: "Invalid credentials",
        }
    }

    pub fn forbidden() -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            code: "FORBIDDEN",
            message: "Forbidden",
        }
    }

    pub fn invalid_cnpj() -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "INVALID_CNPJ",
            message: "Invalid CNPJ check digits",
        }
    }

    pub fn bad_request(code: &'static str, message: &'static str) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code,
            message,
        }
    }

    pub fn sale_not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "SALE_NOT_FOUND",
            message: "Sale not found",
        }
    }

    pub fn insufficient_stock() -> Self {
        Self {
            status: StatusCode::CONFLICT,
            code: "INSUFFICIENT_STOCK",
            message: "Insufficient stock to confirm sale",
        }
    }

    pub fn invalid_transition() -> Self {
        Self {
            status: StatusCode::CONFLICT,
            code: "INVALID_SALE_TRANSITION",
            message: "Sale cannot transition to requested state",
        }
    }

    pub fn commerce_not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "COMMERCE_NOT_FOUND",
            message: "Commerce not found",
        }
    }

    pub fn product_not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "PRODUCT_NOT_FOUND",
            message: "Product not found",
        }
    }

    pub fn inactive_commerce() -> Self {
        Self {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            code: "COMMERCE_INACTIVE",
            message: "Commerce is inactive",
        }
    }

    pub fn inactive_product() -> Self {
        Self {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            code: "INACTIVE_PRODUCT",
            message: "Inactive product cannot be added to sale",
        }
    }

    pub fn order_not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "ORDER_NOT_FOUND",
            message: "Order not found",
        }
    }

    pub fn invalid_order_transition() -> Self {
        Self {
            status: StatusCode::CONFLICT,
            code: "INVALID_ORDER_TRANSITION",
            message: "Order cannot transition to requested state",
        }
    }

    pub fn invalid_delivery_address() -> Self {
        Self {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            code: "INVALID_DELIVERY_ADDRESS",
            message: "Delivery address is invalid for this commerce",
        }
    }

    pub fn rejection_reason_required() -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "REJECTION_REASON_REQUIRED",
            message: "Rejection reason is required",
        }
    }

    pub fn user_not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "USER_NOT_FOUND",
            message: "User not found",
        }
    }

    pub fn unauthorized_payment_declaration() -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            code: "UNAUTHORIZED_PAYMENT_DECLARATION",
            message: "Only the responsible driver may declare payment",
        }
    }

    pub fn insufficient_balance() -> Self {
        Self {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            code: "INSUFFICIENT_BALANCE",
            message: "Insufficient stock balance for adjustment",
        }
    }

    pub fn media_not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "MEDIA_NOT_FOUND",
            message: "Media file not found",
        }
    }

    pub fn invalid_mime() -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "INVALID_MIME",
            message: "File mime type is not allowed",
        }
    }

    pub fn file_too_large() -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "FILE_TOO_LARGE",
            message: "File exceeds maximum upload size",
        }
    }

    pub fn delivery_not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "DELIVERY_NOT_FOUND",
            message: "Delivery not found",
        }
    }

    pub fn proof_required() -> Self {
        Self {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            code: "PROOF_REQUIRED",
            message: "Proof photo is required for delivery confirmation",
        }
    }

    pub fn invalid_delivery_transition() -> Self {
        Self {
            status: StatusCode::CONFLICT,
            code: "INVALID_DELIVERY_TRANSITION",
            message: "Delivery cannot transition to requested state",
        }
    }

    pub fn report_not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "REPORT_NOT_FOUND",
            message: "Report not found",
        }
    }

    pub fn signing_key_unavailable() -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code: "SIGNING_KEY_UNAVAILABLE",
            message: "Report signing key is not configured",
        }
    }

    pub fn internal() -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "INTERNAL_ERROR",
            message: "Internal server error",
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

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        self.into_response("unknown".to_owned())
    }
}

impl axum::response::IntoResponse for &ApiError {
    fn into_response(self) -> Response {
        self.clone().into_response("unknown".to_owned())
    }
}

pub async fn not_found_handler(headers: http::HeaderMap) -> Response {
    correlation_id_from_headers(&headers)
        .map(|id| ApiError::not_found().into_response(id))
        .unwrap_or_else(|| ApiError::not_found().into_response("unknown".to_owned()))
}

fn correlation_id_from_headers(headers: &http::HeaderMap) -> Option<String> {
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

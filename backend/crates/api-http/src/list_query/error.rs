use application::list_query::ListQueryError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ListQueryErrorBody {
    pub error: ListQueryErrorDetail,
}

#[derive(Debug, Clone, Serialize)]
pub struct ListQueryErrorDetail {
    pub code: String,
    pub message: String,
    pub field: String,
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
}

#[derive(Debug, Clone)]
pub struct ListQueryApiError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: &'static str,
    pub field: String,
}

impl ListQueryApiError {
    pub fn from_list_query_error(err: ListQueryError) -> Self {
        match err {
            ListQueryError::InvalidPagination { field, message } => Self {
                status: StatusCode::BAD_REQUEST,
                code: "invalid_pagination",
                message,
                field: field.to_owned(),
            },
            ListQueryError::InvalidFilterField { field } => Self {
                status: StatusCode::BAD_REQUEST,
                code: "invalid_filter_field",
                message: "filter field is not allowed",
                field,
            },
            ListQueryError::InvalidFilterOperator { field, op } => Self {
                status: StatusCode::BAD_REQUEST,
                code: "invalid_filter_field",
                message: "filter operator is not allowed for this field",
                field: format!("{field}[{op}]"),
            },
            ListQueryError::InvalidSortField { field } => Self {
                status: StatusCode::BAD_REQUEST,
                code: "invalid_sort_field",
                message: "sort field is not allowed",
                field,
            },
        }
    }

    pub fn into_response(self, correlation_id: String) -> Response {
        let body = ListQueryErrorBody {
            error: ListQueryErrorDetail {
                code: self.code.to_owned(),
                message: self.message.to_owned(),
                field: self.field,
                correlation_id,
            },
        };
        (self.status, Json(body)).into_response()
    }
}

impl IntoResponse for ListQueryApiError {
    fn into_response(self) -> Response {
        self.into_response("unknown".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use application::list_query::ListPagination;

    #[test]
    fn given_invalid_limit_when_mapped_then_invalid_pagination_contract() {
        let err = ListPagination::new(Some(200), None).unwrap_err();
        let api = ListQueryApiError::from_list_query_error(err);
        assert_eq!(api.code, "invalid_pagination");
        assert_eq!(api.field, "limit");
    }

    #[test]
    fn given_invalid_filter_field_when_serialized_then_contract_shape() {
        let api = ListQueryApiError::from_list_query_error(ListQueryError::InvalidFilterField {
            field: "unknown".into(),
        });
        let body = ListQueryErrorBody {
            error: ListQueryErrorDetail {
                code: api.code.to_owned(),
                message: api.message.to_owned(),
                field: api.field,
                correlation_id: "req-1".into(),
            },
        };
        let json = serde_json::to_value(body).expect("serialize");
        assert_eq!(json["error"]["code"], "invalid_filter_field");
        assert_eq!(json["error"]["field"], "unknown");
        assert_eq!(json["error"]["correlationId"], "req-1");
    }
}

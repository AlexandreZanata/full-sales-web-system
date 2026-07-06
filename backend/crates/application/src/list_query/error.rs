use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ListQueryError {
    #[error("invalid pagination on field {field}: {message}")]
    InvalidPagination {
        field: &'static str,
        message: &'static str,
    },

    #[error("invalid filter field: {field}")]
    InvalidFilterField { field: String },

    #[error("invalid filter operator on field {field}: {op}")]
    InvalidFilterOperator { field: String, op: String },

    #[error("invalid sort field: {field}")]
    InvalidSortField { field: String },
}

impl ListQueryError {
    pub fn invalid_pagination(field: &'static str, message: &'static str) -> Self {
        Self::InvalidPagination { field, message }
    }
}

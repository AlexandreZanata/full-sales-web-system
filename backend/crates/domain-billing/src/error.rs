use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum BillingError {
    #[error("invalid Asaas credentials")]
    InvalidCredentials,

    #[error("customer not found")]
    CustomerNotFound,

    #[error("subscription inactive")]
    SubscriptionInactive,

    #[error("rate limited")]
    RateLimited,

    #[error("upstream unavailable")]
    UpstreamUnavailable,

    #[error("circuit breaker open")]
    CircuitOpen,

    #[error("invalid request: {0}")]
    InvalidRequest(String),
}

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

    #[error("invalid subscription status")]
    InvalidSubscriptionStatus,

    #[error("invalid invoice status")]
    InvalidInvoiceStatus,

    #[error("invalid billing interval")]
    InvalidBillingInterval,

    #[error("invalid plan code")]
    InvalidPlanCode,

    #[error("invalid subscription transition from {from} to {to}")]
    InvalidSubscriptionTransition { from: String, to: String },

    #[error("invalid invoice transition from {from} to {to}")]
    InvalidInvoiceTransition { from: String, to: String },

    #[error("subscription not found")]
    SubscriptionNotFound,

    #[error("invoice not found")]
    InvoiceNotFound,

    #[error("plan does not allow online payments")]
    PlanDoesNotAllowOnlinePayments,

    #[error("tenant Asaas not connected")]
    TenantAsaasNotConnected,
}

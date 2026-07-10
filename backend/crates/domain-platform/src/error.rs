use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlatformError {
    #[error("invalid tenant status")]
    InvalidTenantStatus,

    #[error("invalid tenant transition: {from} → {to}")]
    InvalidTenantTransition { from: String, to: String },

    #[error("tenant mutations blocked in status {0}")]
    TenantMutationsBlocked(String),

    #[error("invalid legal name")]
    InvalidLegalName,

    #[error("suspend reason required")]
    SuspendReasonRequired,

    #[error("suspend reason too short")]
    SuspendReasonTooShort,
}

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DomainError {
    #[error("invalid hostname")]
    InvalidHostname,

    #[error("hostname reserved for platform use")]
    ReservedHostname,

    #[error("invalid domain transition from {from} to {to}")]
    InvalidTransition { from: String, to: String },

    #[error("domain must be verified before activation")]
    NotVerified,

    #[error("primary domain must be verified or active")]
    CannotSetPrimary,
}

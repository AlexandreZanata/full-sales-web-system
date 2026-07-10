use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum FraudError {
    #[error("invalid fraud event status")]
    InvalidEventStatus,

    #[error("invalid fraud event transition from {from} to {to}")]
    InvalidEventTransition { from: String, to: String },

    #[error("invalid blocklist entry")]
    InvalidBlocklistEntry,

    #[error("fraud event not found")]
    EventNotFound,

    #[error("blocklist entry not found")]
    BlocklistEntryNotFound,

    #[error("fraud blocked")]
    Blocked,
}

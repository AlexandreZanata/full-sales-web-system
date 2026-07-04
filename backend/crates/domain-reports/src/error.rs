use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ReportError {
    #[error("report period is invalid")]
    InvalidPeriod,
    #[error("sale amount must be non-negative")]
    NegativeSaleAmount,
}

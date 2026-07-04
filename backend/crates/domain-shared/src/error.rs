use thiserror::Error;

/// Base domain errors for shared value objects.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DomainError {
    #[error("money amount cannot be negative")]
    NegativeMoneyAmount,

    #[error("invalid currency code: must be 3 uppercase ASCII letters")]
    InvalidCurrency,

    #[error("currency mismatch: {0} vs {1}")]
    CurrencyMismatch(String, String),

    #[error("money amount overflow")]
    MoneyOverflow,

    #[error("invalid tenant id")]
    InvalidTenantId,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::DomainError;

    #[test]
    fn given_domain_errors_when_display_then_human_readable() {
        assert_eq!(
            DomainError::NegativeMoneyAmount.to_string(),
            "money amount cannot be negative"
        );
        assert_eq!(
            DomainError::InvalidCurrency.to_string(),
            "invalid currency code: must be 3 uppercase ASCII letters"
        );
        assert_eq!(
            DomainError::CurrencyMismatch("BRL".into(), "USD".into()).to_string(),
            "currency mismatch: BRL vs USD"
        );
        assert_eq!(
            DomainError::MoneyOverflow.to_string(),
            "money amount overflow"
        );
        assert_eq!(
            DomainError::InvalidTenantId.to_string(),
            "invalid tenant id"
        );
    }
}

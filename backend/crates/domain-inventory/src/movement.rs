//! Adjustment reason validation — ENTITY-SPEC-stock-movement §4.

use crate::error::InventoryError;

const MAX_REASON_LEN: usize = 500;

/// Validates `reason` for `Adjustment` movements (required, 1..500 chars).
pub fn validate_adjustment_reason(reason: Option<&str>) -> Result<(), InventoryError> {
    match reason.map(str::trim).filter(|s| !s.is_empty()) {
        Some(r) if r.len() <= MAX_REASON_LEN => Ok(()),
        Some(_) => Err(InventoryError::ReasonTooLong),
        None => Err(InventoryError::MissingReason),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_adjustment_when_reason_missing_then_missing_reason() {
        assert_eq!(
            validate_adjustment_reason(None),
            Err(InventoryError::MissingReason)
        );
    }

    #[test]
    fn given_adjustment_when_reason_valid_then_ok() {
        assert!(validate_adjustment_reason(Some("cycle count correction")).is_ok());
    }

    #[test]
    fn given_adjustment_when_reason_too_long_then_error() {
        let long = "x".repeat(501);
        assert_eq!(
            validate_adjustment_reason(Some(&long)),
            Err(InventoryError::ReasonTooLong)
        );
    }
}

use thiserror::Error;

use crate::sale_status::SaleStatus;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SaleError {
    #[error("invalid sale id")]
    InvalidSaleId,

    #[error("invalid payment method")]
    InvalidPaymentMethod,

    #[error("invalid declared payment method")]
    InvalidDeclaredPaymentMethod,

    #[error("only the responsible driver may declare payment for this sale")]
    UnauthorizedPaymentDeclaration,

    #[error("inactive commerce cannot be referenced in new sale")]
    InactiveCommerce,

    #[error("inactive product cannot be added to sale")]
    InactiveProduct,

    #[error("cannot confirm sale without items")]
    EmptySale,

    #[error("invalid sale transition: {from} → {to}")]
    InvalidTransition { from: SaleStatus, to: SaleStatus },

    #[error("insufficient stock for product")]
    InsufficientStock,
}

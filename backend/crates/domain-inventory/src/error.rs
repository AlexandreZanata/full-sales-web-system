use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InventoryError {
    #[error("invalid product id")]
    InvalidProductId,

    #[error("invalid sku: must be a non-empty alphanumeric identifier")]
    InvalidSku,

    #[error("quantity must be a positive integer")]
    InvalidQuantity,

    #[error("inactive product cannot be added to sale")]
    InactiveProduct,
}

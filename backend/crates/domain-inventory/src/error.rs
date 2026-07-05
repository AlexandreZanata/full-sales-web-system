use thiserror::Error;

use crate::reservation_status::ReservationStatus;

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

    #[error("adjustment reason is required")]
    MissingReason,

    #[error("adjustment reason exceeds maximum length")]
    ReasonTooLong,

    #[error("invalid unit of measure")]
    InvalidUnitOfMeasure,

    #[error("invalid category name")]
    InvalidCategoryName,

    #[error("invalid category slug")]
    InvalidCategorySlug,

    #[error("category not found")]
    CategoryNotFound,

    #[error("invalid product image id")]
    InvalidProductImageId,

    #[error("duplicate primary image for product")]
    DuplicatePrimaryImage,

    #[error("invalid reservation id")]
    InvalidReservationId,

    #[error("invalid reservation status")]
    InvalidReservationStatus,

    #[error("insufficient available stock for reservation")]
    InsufficientAvailableStock,

    #[error("invalid reservation transition from {from} to {to}")]
    InvalidReservationTransition {
        from: ReservationStatus,
        to: ReservationStatus,
    },
}

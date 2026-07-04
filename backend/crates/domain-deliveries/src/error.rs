use thiserror::Error;

use crate::delivery_status::DeliveryStatus;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DeliveryError {
    #[error("invalid delivery id")]
    InvalidDeliveryId,

    #[error("invalid delivery status")]
    InvalidDeliveryStatus,

    #[error("proof photo is required for delivery confirmation")]
    ProofRequired,

    #[error("driver is not assigned to this delivery")]
    DriverNotAssigned,

    #[error("invalid delivery transition: {from} → {to}")]
    InvalidTransition { from: DeliveryStatus, to: DeliveryStatus },
}

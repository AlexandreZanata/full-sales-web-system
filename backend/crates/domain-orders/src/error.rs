use thiserror::Error;

use crate::order_status::OrderStatus;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum OrderError {
    #[error("invalid order id")]
    InvalidOrderId,

    #[error("invalid order item id")]
    InvalidOrderItemId,

    #[error("invalid order source")]
    InvalidOrderSource,

    #[error("invalid order status")]
    InvalidOrderStatus,

    #[error("inactive commerce cannot receive new orders")]
    InactiveCommerce,

    #[error("invalid delivery address for order")]
    InvalidDeliveryAddress,

    #[error("inactive product cannot be added to order")]
    InactiveProduct,

    #[error("cannot submit order without items")]
    EmptyOrder,

    #[error("rejection reason is required")]
    RejectionReasonRequired,

    #[error("invalid order transition: {from} → {to}")]
    InvalidTransition { from: OrderStatus, to: OrderStatus },

    #[error("invalid delivered quantity for order item")]
    InvalidDeliveredQuantity,

    #[error("order item not found")]
    OrderItemNotFound,

    #[error("insufficient available stock for order approval")]
    InsufficientAvailableStock,
}

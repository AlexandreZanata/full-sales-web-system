//! Orders domain — Order aggregate, OrderItem, and state machine.

pub mod error;
pub mod order;
pub mod order_id;
pub mod order_item;
pub mod order_item_id;
pub mod order_source;
pub mod order_status;
pub mod reservation_port;

pub use error::OrderError;
pub use order::{AddOrderItemInput, DeliveredItemInput, Order, OrderCreateInput};
pub use order_id::OrderId;
pub use order_item::OrderItem;
pub use order_item_id::OrderItemId;
pub use order_source::OrderSource;
pub use order_status::OrderStatus;
pub use reservation_port::{InMemoryReservationPort, StockReservationPort, StockSnapshot};

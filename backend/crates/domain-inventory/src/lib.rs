//! Inventory domain — Product catalog entity and value objects.

pub mod error;
pub mod movement;
pub mod product;
pub mod product_id;
pub mod quantity;
pub mod sku;

pub use error::InventoryError;
pub use movement::validate_adjustment_reason;
pub use product::{Product, ProductCreateInput};
pub use product_id::ProductId;
pub use quantity::Quantity;
pub use sku::Sku;

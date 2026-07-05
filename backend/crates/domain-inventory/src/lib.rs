//! Inventory domain — Product catalog entity and value objects.

pub mod available_stock;
pub mod category_slug;
pub mod error;
pub mod movement;
pub mod product;
pub mod product_category;
pub mod product_id;
pub mod product_image;
pub mod product_image_id;
pub mod quantity;
pub mod reservation_id;
pub mod reservation_status;
pub mod sku;
pub mod stock_reservation;
pub mod unit_of_measure;

pub use available_stock::{
    AvailableStockInput, compute_available, ensure_can_reserve, tenant_available_stock,
};
pub use error::InventoryError;
pub use movement::validate_adjustment_reason;
pub use category_slug::{slugify_name, unique_slug};
pub use product::{Product, ProductCreateInput};
pub use product_category::{ProductCategory, ProductCategoryCreateInput};
pub use product_id::ProductId;
pub use product_image::{CreateProductImageInput, ProductImage};
pub use product_image_id::ProductImageId;
pub use quantity::Quantity;
pub use reservation_id::ReservationId;
pub use reservation_status::ReservationStatus;
pub use sku::Sku;
pub use stock_reservation::{CreateStockReservationInput, StockReservation};
pub use unit_of_measure::UnitOfMeasure;

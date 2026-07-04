//! Sales domain — Sale aggregate and related value objects.

pub mod error;
pub mod payment_method;
pub mod sale;
pub mod sale_id;
pub mod sale_item;
pub mod sale_status;

pub use error::SaleError;
pub use payment_method::PaymentMethod;
pub use sale::{AddSaleItemInput, Sale, SaleCreateInput};
pub use sale_id::SaleId;
pub use sale_item::SaleItem;
pub use sale_status::SaleStatus;

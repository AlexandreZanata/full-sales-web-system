//! Commerces domain — Commerce aggregate and CNPJ value object.

pub mod cnpj;
pub mod commerce;
pub mod commerce_id;
pub mod error;

pub use cnpj::Cnpj;
pub use commerce::{Commerce, CreateCommerceInput};
pub use commerce_id::CommerceId;
pub use error::CommerceError;

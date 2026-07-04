//! Commerces domain — Commerce aggregate and CNPJ value object.

pub mod address_type;
pub mod brazilian_state;
pub mod cnpj;
pub mod commerce;
pub mod commerce_address;
pub mod commerce_address_id;
pub mod commerce_id;
pub mod error;
pub mod postal_code;

pub use address_type::AddressType;
pub use brazilian_state::BrazilianState;
pub use cnpj::Cnpj;
pub use commerce::{Commerce, CreateCommerceInput};
pub use commerce_address::{
    CommerceAddress, CreateCommerceAddressInput, ensure_address_allowed_for_commerce,
    validate_order_delivery_address,
};
pub use commerce_address_id::CommerceAddressId;
pub use commerce_id::CommerceId;
pub use error::CommerceError;
pub use postal_code::PostalCode;

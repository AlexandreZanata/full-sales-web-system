//! Sales domain — Sale aggregate and related value objects.

pub mod audit_port;
pub mod declared_payment;
pub mod declared_payment_method;
pub mod error;
pub mod payment_method;
pub mod sale;
pub mod sale_id;
pub mod sale_item;
pub mod sale_status;

pub use audit_port::{
    InMemoryPaymentDeclarationAuditPort, PaymentDeclarationAuditEntry, PaymentDeclarationAuditPort,
};
pub use declared_payment::DeclaredPayment;
pub use declared_payment_method::DeclaredPaymentMethod;
pub use error::SaleError;
pub use payment_method::PaymentMethod;
pub use sale::{
    AddSaleItemInput, DeclarePaymentInput, Sale, SaleCreateInput, SaleFromDeliveryInput,
};
pub use sale_id::SaleId;
pub use sale_item::SaleItem;
pub use sale_status::SaleStatus;

//! Shared domain value objects and errors used across bounded contexts.

pub mod error;
pub mod money;
pub mod tenant_id;

pub use error::DomainError;
pub use money::{Currency, Money};
pub use tenant_id::TenantId;

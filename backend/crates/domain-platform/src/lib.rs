//! Platform SaaS domain — tenant lifecycle (ADR-015).

pub mod error;
pub mod tenant;
pub mod tenant_status;

pub use error::PlatformError;
pub use tenant::Tenant;
pub use tenant_status::{TenantStatus, can_transition};

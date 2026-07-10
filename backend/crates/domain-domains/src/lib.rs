mod domain_status;
mod error;
mod hostname;
mod tenant_domain;

pub use domain_status::{DomainStatus, can_transition};
pub use error::DomainError;
pub use hostname::{is_reserved_hostname, normalize_hostname, txt_record_name};
pub use tenant_domain::TenantDomain;

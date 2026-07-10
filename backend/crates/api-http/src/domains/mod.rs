mod dns;
mod host;
mod support;
mod verification;

pub mod settings;
pub mod platform;

pub use dns::{EmptyDnsTxtResolver, MockDnsTxtResolver};
pub use support::HostTenant;
pub use host::{PublicTenantId, host_tenant_middleware};
pub use settings::{
    create_domain, delete_domain, get_domain_verify, list_domains, set_primary_domain,
};
pub use platform::{
    force_verify_platform_domain, list_platform_domains, patch_platform_domain,
    run_domain_verification_job_handler,
};
pub use verification::{force_verify_domain, run_domain_verification_job};

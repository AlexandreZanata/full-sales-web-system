use chrono::{Duration, Utc};
use domain_platform::{PlatformError, Tenant, TenantStatus};
use domain_shared::TenantId;
use uuid::Uuid;

use crate::AppError;

pub const PLAN_STARTER: Uuid = uuid::uuid!("01900002-0001-7000-8000-000000000001");
pub const RETENTION_DAYS: i32 = 90;
pub const TRIAL_DAYS: i64 = 14;

pub struct ProvisionTenantInput {
    pub legal_name: String,
    pub display_name: String,
    pub admin_email: String,
    pub plan_id: Uuid,
    pub trial: bool,
    pub seed_demo_catalog: bool,
}

pub struct ProvisionTenantResult {
    pub tenant_id: TenantId,
    pub admin_user_id: Uuid,
    pub status: TenantStatus,
    pub trial_ends_at: Option<chrono::DateTime<Utc>>,
}

pub fn ensure_tenant_active(status: TenantStatus) -> Result<(), AppError> {
    status
        .allows_mutations()
        .then_some(())
        .ok_or(AppError::TenantSuspended)
}

pub fn build_provision_tenant(
    tenant_id: TenantId,
    input: &ProvisionTenantInput,
) -> Result<(Tenant, Option<chrono::DateTime<Utc>>), AppError> {
    let mut tenant = Tenant::new_provisioning(tenant_id, input.legal_name.clone(), input.display_name.clone())
        .map_err(map_platform_error)?;
    let trial_ends = if input.trial {
        let ends = Utc::now() + Duration::days(TRIAL_DAYS);
        tenant
            .activate_trial(input.plan_id, ends)
            .map_err(map_platform_error)?;
        Some(ends)
    } else {
        tenant
            .activate_paid(input.plan_id)
            .map_err(map_platform_error)?;
        None
    };
    Ok((tenant, trial_ends))
}

pub fn apply_suspend(tenant: &mut Tenant, reason: &str) -> Result<(), AppError> {
    tenant
        .suspend(reason, Utc::now())
        .map_err(map_platform_error)
}

pub fn apply_reactivate(tenant: &mut Tenant) -> Result<(), AppError> {
    tenant.reactivate().map_err(map_platform_error)
}

pub fn apply_offboard(tenant: &mut Tenant) -> Result<(), AppError> {
    tenant
        .begin_offboarding(Utc::now())
        .map_err(map_platform_error)
}

pub fn map_platform_error(err: PlatformError) -> AppError {
    match err {
        PlatformError::TenantMutationsBlocked(_) => AppError::TenantSuspended,
        other => AppError::Platform(other),
    }
}

use chrono::Utc;
use uuid::Uuid;

use application::AppError;

use super::support::row_to_tenant;

pub async fn run_dunning_job(pool: &infra_postgres::PgPool) -> Result<Vec<Uuid>, AppError> {
    let candidates = infra_postgres::shared::find_dunning_candidates(pool)
        .await
        .map_err(|_| AppError::Forbidden)?;
    let mut processed = Vec::new();
    for tenant_id in candidates {
        let row = infra_postgres::shared::find_tenant_lifecycle(pool, tenant_id)
            .await
            .map_err(|_| AppError::Forbidden)?
            .ok_or(AppError::Forbidden)?;
        let mut tenant = row_to_tenant(&row);
        tenant
            .suspend("grace_period_expired", Utc::now())
            .map_err(application::tenants::map_platform_error)?;
        infra_postgres::shared::update_tenant_lifecycle(
            pool,
            tenant_id,
            tenant.status,
            tenant.plan_id,
            tenant.trial_ends_at,
            tenant.suspended_at,
            tenant.suspended_reason.as_deref(),
            tenant.offboarding_scheduled_at,
            None,
            None,
        )
        .await
        .map_err(|_| AppError::Forbidden)?;
        processed.push(tenant_id.as_uuid());
    }
    Ok(processed)
}

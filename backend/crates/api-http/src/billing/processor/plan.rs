use domain_shared::TenantId;
use uuid::Uuid;

use application::AppError;

pub async fn change_tenant_plan(
    pool: &infra_postgres::PgPool,
    tenant_id: TenantId,
    plan_id: Uuid,
) -> Result<(), AppError> {
    if infra_postgres::billing::find_plan(pool, plan_id)
        .await
        .map_err(|_| AppError::Forbidden)?
        .is_none()
    {
        return Err(AppError::Forbidden);
    }
    if let Some(sub) = infra_postgres::billing::find_subscription_by_tenant(pool, tenant_id)
        .await
        .map_err(|_| AppError::Forbidden)?
    {
        infra_postgres::billing::update_subscription_plan(pool, sub.id, plan_id, None)
            .await
            .map_err(|_| AppError::Forbidden)?;
    }
    let row = infra_postgres::shared::find_tenant_lifecycle(pool, tenant_id)
        .await
        .map_err(|_| AppError::Forbidden)?
        .ok_or(AppError::Forbidden)?;
    infra_postgres::shared::update_tenant_lifecycle(
        pool,
        tenant_id,
        row.status,
        Some(plan_id),
        row.trial_ends_at,
        row.suspended_at,
        row.suspended_reason.as_deref(),
        row.offboarding_scheduled_at,
        None,
        None,
    )
    .await
    .map_err(|_| AppError::Forbidden)?;
    Ok(())
}

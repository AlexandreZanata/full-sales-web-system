use domain_fraud::{FraudEvent, FraudEventType, FraudSeverity};
use domain_shared::TenantId;
use uuid::Uuid;

use crate::error::ApiError;
use crate::platform::tenants::row_to_tenant;
use crate::state::AppState;

use super::coordinator::record_event;

pub async fn handle_chargeback(
    state: &AppState,
    tenant_id: TenantId,
    payment_id: &str,
    auto_suspend: bool,
) -> Result<(), ApiError> {
    let event = FraudEvent::new_open(
        Uuid::now_v7(),
        Some(tenant_id),
        None,
        FraudEventType::Chargeback,
        FraudSeverity::Critical,
        serde_json::json!({ "paymentId": payment_id }),
    );
    record_event(state, event).await?;
    if !auto_suspend {
        return Ok(());
    }
    let row = infra_postgres::shared::find_tenant_lifecycle(&state.admin_pool, tenant_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;
    let mut tenant = row_to_tenant(&row);
    tenant
        .suspend("Asaas chargeback", chrono::Utc::now())
        .map_err(|_| ApiError::internal())?;
    infra_postgres::shared::update_tenant_lifecycle(
        &state.admin_pool,
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
    .map_err(|_| ApiError::internal())?;
    Ok(())
}

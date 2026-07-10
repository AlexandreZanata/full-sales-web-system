use domain_shared::TenantId;

use crate::error::ApiError;
use crate::state::AppState;

pub async fn load_resolved_flags(
    state: &AppState,
    tenant_id: TenantId,
) -> Result<application::feature_flags::ResolvedFeatureFlags, ApiError> {
    let lifecycle = infra_postgres::shared::find_tenant_lifecycle(&state.admin_pool, tenant_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;
    let plan_limits = match lifecycle.plan_id {
        Some(plan_id) => {
            infra_postgres::billing::find_plan(&state.admin_pool, plan_id)
                .await
                .map_err(|_| ApiError::internal())?
                .map(|p| p.feature_limits)
                .unwrap_or_else(|| serde_json::json!({}))
        }
        None => serde_json::json!({}),
    };
    Ok(application::feature_flags::resolve_feature_flags(
        &plan_limits,
        &lifecycle.settings,
    ))
}

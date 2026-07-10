use axum::{Json, extract::{Path, State}};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ApiError;
use crate::platform::auth::PlatformAuthUser;
use crate::platform_audit::record_platform_audit_stub;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct PatchTenantFeaturesRequest {
    #[serde(rename = "onlinePayments")]
    pub online_payments: Option<bool>,
    #[serde(rename = "customDomain")]
    pub custom_domain: Option<bool>,
    #[serde(rename = "apiRateTier")]
    pub api_rate_tier: Option<String>,
}

#[derive(Serialize)]
pub struct TenantFeaturesResponse {
    #[serde(rename = "onlinePayments")]
    pub online_payments: bool,
    #[serde(rename = "customDomain")]
    pub custom_domain: bool,
    #[serde(rename = "apiRateTier")]
    pub api_rate_tier: String,
}

pub async fn patch_tenant_features(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<PatchTenantFeaturesRequest>,
) -> Result<Json<TenantFeaturesResponse>, ApiError> {
    let tenant_id = domain_shared::TenantId::from_uuid(id);
    let mut lifecycle = infra_postgres::shared::find_tenant_lifecycle(&state.admin_pool, tenant_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;
    if let Some(tier) = &body.api_rate_tier {
        if !matches!(tier.as_str(), "standard" | "pro" | "enterprise") {
            return Err(ApiError::bad_request("VALIDATION_ERROR", "Invalid apiRateTier"));
        }
    }
    application::feature_flags::merge_feature_flags(
        &mut lifecycle.settings,
        application::feature_flags::TenantFeatureFlags {
            online_payments: body.online_payments,
            custom_domain: body.custom_domain,
            api_rate_tier: body.api_rate_tier,
        },
    );
    let flags = lifecycle
        .settings
        .get("feature_flags")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    infra_postgres::shared::update_tenant_feature_flags(&state.admin_pool, tenant_id, flags)
        .await
        .map_err(|_| ApiError::internal())?;
    record_platform_audit_stub(&state, auth.user_id, "tenant.features.patch", Some(id)).await;
    let resolved = super::feature_support::load_resolved_flags(&state, tenant_id).await?;
    Ok(Json(TenantFeaturesResponse {
        online_payments: resolved.online_payments,
        custom_domain: resolved.custom_domain,
        api_rate_tier: resolved.api_rate_tier,
    }))
}

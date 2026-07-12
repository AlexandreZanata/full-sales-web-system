use domain_billing::{plan_allows_custom_domain, plan_allows_online_payments};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TenantFeatureFlags {
    #[serde(rename = "onlinePayments", skip_serializing_if = "Option::is_none")]
    pub online_payments: Option<bool>,
    #[serde(rename = "customDomain", skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<bool>,
    #[serde(rename = "apiRateTier", skip_serializing_if = "Option::is_none")]
    pub api_rate_tier: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ResolvedFeatureFlags {
    pub online_payments: bool,
    pub custom_domain: bool,
    pub api_rate_tier: String,
}

pub fn parse_feature_flags(settings: &Value) -> TenantFeatureFlags {
    settings
        .get("feature_flags")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default()
}

pub fn resolve_feature_flags(plan_limits: &Value, settings: &Value) -> ResolvedFeatureFlags {
    let overrides = parse_feature_flags(settings);
    ResolvedFeatureFlags {
        online_payments: overrides
            .online_payments
            .unwrap_or_else(|| plan_allows_online_payments(plan_limits)),
        custom_domain: overrides
            .custom_domain
            .unwrap_or_else(|| plan_allows_custom_domain(plan_limits)),
        api_rate_tier: overrides
            .api_rate_tier
            .unwrap_or_else(|| default_rate_tier(plan_limits)),
    }
}

pub fn merge_feature_flags(settings: &mut Value, patch: TenantFeatureFlags) {
    let mut current = parse_feature_flags(settings);
    if let Some(v) = patch.online_payments {
        current.online_payments = Some(v);
    }
    if let Some(v) = patch.custom_domain {
        current.custom_domain = Some(v);
    }
    if let Some(v) = patch.api_rate_tier {
        current.api_rate_tier = Some(v);
    }
    if !settings.is_object() {
        *settings = serde_json::json!({});
    }
    let Some(obj) = settings.as_object_mut() else {
        return;
    };
    let Ok(flags) = serde_json::to_value(current) else {
        return;
    };
    obj.insert("feature_flags".into(), flags);
}

fn default_rate_tier(plan_limits: &Value) -> String {
    if plan_limits
        .get("maxUsers")
        .and_then(Value::as_i64)
        .is_none()
    {
        "enterprise".into()
    } else if plan_allows_custom_domain(plan_limits) {
        "pro".into()
    } else {
        "standard".into()
    }
}

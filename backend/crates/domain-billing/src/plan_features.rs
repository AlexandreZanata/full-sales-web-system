use serde_json::Value;

/// Pro+ plans expose `customDomain: true` in `feature_limits` (ADR-017).
pub fn plan_allows_custom_domain(feature_limits: &Value) -> bool {
    feature_limits
        .get("customDomain")
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

/// Pro+ plans expose `onlinePayments: true` in `feature_limits` (ADR-018).
pub fn plan_allows_online_payments(feature_limits: &Value) -> bool {
    feature_limits
        .get("onlinePayments")
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

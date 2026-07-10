use domain_fraud::{FraudEvent, FraudEventType, FraudSeverity};
use domain_shared::TenantId;
use uuid::Uuid;

use crate::error::ApiError;
use crate::state::AppState;

use super::coordinator::record_event;

pub async fn check_duplicate_card(
    state: &AppState,
    tenant_id: TenantId,
    card_fingerprint: &str,
) -> Result<(), ApiError> {
    let hit = infra_postgres::fraud::find_duplicate_card_tenant(
        &state.admin_pool,
        card_fingerprint,
        tenant_id,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    if hit.is_some() {
        let event = FraudEvent::new_open(
            Uuid::now_v7(),
            Some(tenant_id),
            None,
            FraudEventType::DuplicateCard,
            FraudSeverity::High,
            serde_json::json!({ "cardFingerprint": card_fingerprint }),
        );
        record_event(state, event).await?;
    }
    Ok(())
}

pub async fn check_amount_anomaly(
    state: &AppState,
    tenant_id: TenantId,
    amount_minor: i64,
) -> Result<(), ApiError> {
    let avg = infra_postgres::fraud::average_order_amount_minor(&state.admin_pool, tenant_id)
        .await
        .map_err(|_| ApiError::internal())?;
    let Some(avg) = avg else {
        return Ok(());
    };
    if amount_minor as f64 > avg * 3.0 {
        let event = FraudEvent::new_open(
            Uuid::now_v7(),
            Some(tenant_id),
            None,
            FraudEventType::AmountAnomaly,
            FraudSeverity::Medium,
            serde_json::json!({ "amountMinor": amount_minor, "averageMinor": avg }),
        );
        record_event(state, event).await?;
    }
    Ok(())
}

/// ponytail: email delivery deferred — log-only stub for high-severity tenant alerts.
pub fn notify_high_severity_stub(tenant_id: TenantId, event_type: FraudEventType) {
    tracing::warn!(
        tenant_id = %tenant_id.as_uuid(),
        event_type = event_type.as_str(),
        "high-severity fraud alert (email stub)"
    );
}

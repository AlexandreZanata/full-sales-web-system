use application::fraud::FraudThresholds;
use domain_fraud::{FraudEvent, FraudEventType, FraudResolution, FraudSeverity};
use domain_shared::TenantId;
use infra_redis::VelocityCounter;
use uuid::Uuid;

use crate::error::ApiError;
use crate::state::AppState;

pub async fn load_thresholds(state: &AppState) -> Result<FraudThresholds, ApiError> {
    let json = infra_postgres::fraud::get_platform_thresholds(&state.admin_pool)
        .await
        .map_err(|_| ApiError::internal())?;
    Ok(FraudThresholds::from_json(&json))
}

pub async fn record_event(state: &AppState, event: FraudEvent) -> Result<(), ApiError> {
    infra_postgres::fraud::insert_fraud_event(
        &state.admin_pool,
        infra_postgres::fraud::NewFraudEvent {
            id: event.id,
            tenant_id: event.tenant_id,
            user_id: event.user_id,
            event_type: event.event_type,
            severity: event.severity,
            metadata: event.metadata,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;
    if let Some(tenant_id) = event.tenant_id {
        infra_postgres::fraud::add_tenant_fraud_score(
            &state.admin_pool,
            tenant_id,
            event.severity.score_delta(),
        )
        .await
        .map_err(|_| ApiError::internal())?;
    }
    Ok(())
}

pub async fn check_blocklist(
    state: &AppState,
    email: Option<&str>,
    cnpj: Option<&str>,
    ip: Option<&str>,
    card_fingerprint: Option<&str>,
    tenant_id: Option<TenantId>,
) -> Result<(), ApiError> {
    let hit = infra_postgres::fraud::find_active_blocklist_match(
        &state.admin_pool,
        email,
        cnpj,
        ip,
        card_fingerprint,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    if let Some(row) = hit {
        let event = FraudEvent::new_open(
            Uuid::now_v7(),
            tenant_id,
            None,
            FraudEventType::BlocklistHit,
            FraudSeverity::High,
            serde_json::json!({
                "blocklistId": row.id,
                "reason": row.reason,
            }),
        );
        record_event(state, event).await?;
        return Err(ApiError::fraud_blocked());
    }
    Ok(())
}

pub async fn ensure_checkout_allowed(
    state: &AppState,
    tenant_id: TenantId,
) -> Result<(), ApiError> {
    let thresholds = load_thresholds(state).await?;
    let score = infra_postgres::fraud::get_tenant_fraud_score(&state.admin_pool, tenant_id)
        .await
        .map_err(|_| ApiError::internal())?;
    if thresholds.checkout_blocked(score) {
        return Err(ApiError::fraud_blocked());
    }
    Ok(())
}

pub async fn check_payment_velocity(state: &AppState, tenant_id: TenantId) -> Result<(), ApiError> {
    let thresholds = load_thresholds(state).await?;
    let key = format!("fraud:velocity:payment:{}", tenant_id.as_uuid());
    let count = state
        .velocity_counter
        .increment(&key, thresholds.payment_velocity_window)
        .await
        .map_err(|_| ApiError::internal())?;
    if count > thresholds.payment_velocity_max {
        let event = FraudEvent::new_open(
            Uuid::now_v7(),
            Some(tenant_id),
            None,
            FraudEventType::PaymentVelocity,
            FraudSeverity::High,
            serde_json::json!({ "count": count, "max": thresholds.payment_velocity_max }),
        );
        record_event(state, event).await?;
        return Err(ApiError::fraud_blocked());
    }
    Ok(())
}

pub async fn on_login_failure(state: &AppState, ip: &str, email: &str) -> Result<(), ApiError> {
    let thresholds = load_thresholds(state).await?;
    let ip_key = format!("fraud:velocity:login:ip:{ip}");
    let email_key = format!("fraud:velocity:login:email:{email}");
    let ip_count = state
        .velocity_counter
        .increment(&ip_key, thresholds.login_failure_window)
        .await
        .map_err(|_| ApiError::internal())?;
    let email_count = state
        .velocity_counter
        .increment(&email_key, thresholds.login_failure_window)
        .await
        .map_err(|_| ApiError::internal())?;
    let breached =
        ip_count > thresholds.login_failure_max || email_count > thresholds.login_failure_max;
    if breached {
        let event = FraudEvent::new_open(
            Uuid::now_v7(),
            None,
            None,
            FraudEventType::LoginVelocity,
            FraudSeverity::Medium,
            serde_json::json!({ "ip": ip, "email": email, "ipCount": ip_count, "emailCount": email_count }),
        );
        record_event(state, event).await?;
        return Err(ApiError::fraud_blocked());
    }
    Ok(())
}

pub async fn on_provision_attempt(
    state: &AppState,
    platform_user_id: Uuid,
) -> Result<(), ApiError> {
    let thresholds = load_thresholds(state).await?;
    let key = format!("fraud:velocity:provision:{platform_user_id}");
    let count = state
        .velocity_counter
        .increment(&key, thresholds.provision_alert_window)
        .await
        .map_err(|_| ApiError::internal())?;
    if count > thresholds.provision_alert_max {
        let event = FraudEvent::new_open(
            Uuid::now_v7(),
            None,
            Some(platform_user_id),
            FraudEventType::ProvisionVelocity,
            FraudSeverity::Medium,
            serde_json::json!({ "count": count }),
        );
        record_event(state, event).await?;
    }
    Ok(())
}

pub async fn on_webhook_processing_failure(
    state: &AppState,
    tenant_id: Option<TenantId>,
) -> Result<(), ApiError> {
    let thresholds = load_thresholds(state).await?;
    let key = format!(
        "fraud:velocity:webhook:{}",
        tenant_id
            .map(|t| t.as_uuid().to_string())
            .unwrap_or_else(|| "platform".into())
    );
    let count = state
        .velocity_counter
        .increment(&key, thresholds.webhook_failure_burst_window)
        .await
        .map_err(|_| ApiError::internal())?;
    if count > thresholds.webhook_failure_burst_max {
        let event = FraudEvent::new_open(
            Uuid::now_v7(),
            tenant_id,
            None,
            FraudEventType::WebhookFailureBurst,
            FraudSeverity::High,
            serde_json::json!({ "count": count }),
        );
        record_event(state, event).await?;
    }
    Ok(())
}

pub fn restore_fraud_event(row: &infra_postgres::fraud::FraudEventRow) -> FraudEvent {
    FraudEvent {
        id: row.id,
        tenant_id: row.tenant_id,
        user_id: row.user_id,
        event_type: parse_event_type(&row.event_type),
        severity: parse_severity(&row.severity),
        status: parse_status(&row.status),
        resolution: row
            .resolution
            .as_deref()
            .and_then(|v| FraudResolution::parse(v).ok()),
        resolution_note: row.resolution_note.clone(),
        metadata: row.metadata.clone(),
        reviewed_by: row.reviewed_by,
        reviewed_at: row.reviewed_at,
        created_at: row.created_at,
    }
}

fn parse_event_type(value: &str) -> FraudEventType {
    match value {
        "LoginVelocity" => FraudEventType::LoginVelocity,
        "PaymentVelocity" => FraudEventType::PaymentVelocity,
        "ProvisionVelocity" => FraudEventType::ProvisionVelocity,
        "WebhookFailureBurst" => FraudEventType::WebhookFailureBurst,
        "DuplicateCard" => FraudEventType::DuplicateCard,
        "AmountAnomaly" => FraudEventType::AmountAnomaly,
        "Chargeback" => FraudEventType::Chargeback,
        _ => FraudEventType::BlocklistHit,
    }
}

fn parse_severity(value: &str) -> FraudSeverity {
    match value {
        "Low" => FraudSeverity::Low,
        "Medium" => FraudSeverity::Medium,
        "High" => FraudSeverity::High,
        _ => FraudSeverity::Critical,
    }
}

fn parse_status(value: &str) -> domain_fraud::FraudEventStatus {
    match value {
        "Reviewed" => domain_fraud::FraudEventStatus::Reviewed,
        "Blocked" => domain_fraud::FraudEventStatus::Blocked,
        _ => domain_fraud::FraudEventStatus::Open,
    }
}

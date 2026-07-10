use axum::{
    Json,
    extract::{Query, State},
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use domain_fraud::FraudSeverity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::{AuthUser, require_admin};
use crate::error::ApiError;
use crate::fraud::{notify_high_severity_stub, restore_fraud_event};
use crate::state::AppState;

const DEFAULT_LIMIT: i64 = 20;
const MAX_LIMIT: i64 = 50;

#[derive(Deserialize)]
pub struct FraudAlertsQuery {
    pub limit: Option<i64>,
}

#[derive(Serialize)]
pub struct FraudAlertResponse {
    pub id: Uuid,
    #[serde(rename = "eventType")]
    pub event_type: String,
    pub severity: String,
    pub status: String,
    pub metadata: serde_json::Value,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

pub async fn list_fraud_alerts(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<FraudAlertsQuery>,
) -> Result<Json<Vec<FraudAlertResponse>>, Response> {
    require_admin(&auth).map_err(IntoResponse::into_response)?;
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let rows = infra_postgres::fraud::list_fraud_events_tenant(
        &state.app_pool,
        auth.tenant_id,
        limit,
    )
    .await
    .map_err(|_| IntoResponse::into_response(ApiError::internal()))?;
    let items: Vec<FraudAlertResponse> = rows
        .into_iter()
        .map(|row| {
            let event = restore_fraud_event(&row);
            if matches!(event.severity, FraudSeverity::High | FraudSeverity::Critical) {
                notify_high_severity_stub(auth.tenant_id, event.event_type);
            }
            FraudAlertResponse {
                id: event.id,
                event_type: event.event_type.as_str().into(),
                severity: event.severity.as_str().into(),
                status: event.status.as_str().into(),
                metadata: event.metadata,
                created_at: event.created_at,
            }
        })
        .collect();
    Ok(Json(items))
}

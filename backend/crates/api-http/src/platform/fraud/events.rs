use axum::{
    Json,
    extract::{Path, Query, State},
};
use chrono::{DateTime, Utc};
use domain_fraud::FraudResolution;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ApiError;
use crate::fraud::restore_fraud_event;
use crate::list_query::build_cursor_page;
use crate::platform::auth::PlatformAuthUser;
use crate::state::AppState;

const DEFAULT_LIMIT: u32 = 20;
const MAX_LIMIT: u32 = 100;

#[derive(Deserialize)]
pub struct FraudEventsQuery {
    pub limit: Option<u32>,
    pub cursor: Option<Uuid>,
    #[serde(rename = "filter[status]")]
    pub filter_status: Option<String>,
    #[serde(rename = "filter[severity]")]
    pub filter_severity: Option<String>,
    #[serde(rename = "filter[tenant_id]")]
    pub filter_tenant_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct FraudEventResponse {
    pub id: Uuid,
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<Uuid>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    #[serde(rename = "eventType")]
    pub event_type: String,
    pub severity: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(rename = "resolutionNote", skip_serializing_if = "Option::is_none")]
    pub resolution_note: Option<String>,
    pub metadata: serde_json::Value,
    #[serde(rename = "reviewedBy", skip_serializing_if = "Option::is_none")]
    pub reviewed_by: Option<Uuid>,
    #[serde(rename = "reviewedAt", skip_serializing_if = "Option::is_none")]
    pub reviewed_at: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct ResolveFraudEventRequest {
    pub resolution: String,
    pub note: Option<String>,
}

pub async fn list_fraud_events(
    State(state): State<AppState>,
    _auth: PlatformAuthUser,
    Query(query): Query<FraudEventsQuery>,
) -> Result<Json<crate::list_query::CursorListResponse<FraudEventResponse>>, ApiError> {
    let limit = query
        .limit
        .unwrap_or(DEFAULT_LIMIT)
        .clamp(1, MAX_LIMIT);
    let filters = infra_postgres::fraud::FraudEventFilters {
        status: query.filter_status,
        severity: query.filter_severity,
        tenant_id: query.filter_tenant_id,
    };
    let rows = infra_postgres::fraud::list_fraud_events_platform(
        &state.admin_pool,
        &filters,
        query.cursor,
        i64::from(limit) + 1,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    let items: Vec<FraudEventResponse> = rows
        .into_iter()
        .map(|row| event_response(&restore_fraud_event(&row)))
        .collect();
    Ok(Json(build_cursor_page(items, limit, |item| item.id)))
}

pub async fn resolve_fraud_event(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<ResolveFraudEventRequest>,
) -> Result<Json<FraudEventResponse>, ApiError> {
    let resolution = FraudResolution::parse(&body.resolution)
        .map_err(|_| ApiError::bad_request("INVALID_INPUT", "Invalid fraud resolution"))?;
    let row = infra_postgres::fraud::find_fraud_event(&state.admin_pool, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;
    let event = restore_fraud_event(&row);
    let note = body.note.clone();
    let _ = event
        .resolve(auth.user_id, resolution, note.clone())
        .map_err(|_| ApiError::bad_request("INVALID_TRANSITION", "Fraud event already resolved"))?;
    let updated = infra_postgres::fraud::resolve_fraud_event(
        &state.admin_pool,
        id,
        auth.user_id,
        resolution,
        note.as_deref(),
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::not_found)?;
    Ok(Json(event_response(&restore_fraud_event(&updated))))
}

fn event_response(event: &domain_fraud::FraudEvent) -> FraudEventResponse {
    FraudEventResponse {
        id: event.id,
        tenant_id: event.tenant_id.map(|t| t.as_uuid()),
        user_id: event.user_id,
        event_type: event.event_type.as_str().into(),
        severity: event.severity.as_str().into(),
        status: event.status.as_str().into(),
        resolution: event.resolution.map(|r| r.as_str().into()),
        resolution_note: event.resolution_note.clone(),
        metadata: event.metadata.clone(),
        reviewed_by: event.reviewed_by,
        reviewed_at: event.reviewed_at,
        created_at: event.created_at,
    }
}

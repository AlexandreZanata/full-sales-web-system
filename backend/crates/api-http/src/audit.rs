use axum::{
    Json,
    extract::{Path, RawQuery, State},
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::auth::{AuthUser, require_admin};
use crate::error::ApiError;
use crate::list_query::{
    AUDIT_EVENTS_LIST_CONFIG, CursorListResponse, build_cursor_page, decode_query_pairs,
    filter_eq_string, filter_eq_uuid, filter_gte_datetime, filter_lte_datetime, parse_list_query,
};
use crate::platform::audit::map_range_error;
use crate::platform::audit::map_row;
use crate::state::AppState;

#[derive(Serialize)]
pub struct AuditEventResponse {
    pub id: Uuid,
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<Uuid>,
    #[serde(rename = "actorId")]
    pub actor_id: Uuid,
    #[serde(rename = "actorType")]
    pub actor_type: String,
    pub action: String,
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceId")]
    pub resource_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "correlationId", skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

pub async fn list_audit_events(
    State(state): State<AppState>,
    auth: AuthUser,
    RawQuery(query): RawQuery,
) -> Result<Json<CursorListResponse<AuditEventResponse>>, Response> {
    require_admin(&auth).map_err(IntoResponse::into_response)?;
    let parsed = parse_list_query(
        &decode_query_pairs(query.as_deref()),
        &AUDIT_EVENTS_LIST_CONFIG,
    )
    .map_err(IntoResponse::into_response)?;
    let from = filter_gte_datetime(&parsed.filters, "created_at");
    let to = filter_lte_datetime(&parsed.filters, "created_at");
    let (from, to) = application::validate_audit_date_range(from, to)
        .map_err(|err| IntoResponse::into_response(map_range_error(err)))?;
    let filters = infra_postgres::audit::AuditEventFilters {
        tenant_id: None,
        actor_id: filter_eq_uuid(&parsed.filters, "actor_id"),
        action: filter_eq_string(&parsed.filters, "action"),
        from: Some(from),
        to: Some(to),
    };

    let rows = infra_postgres::audit::list_audit_events_cursor(
        &state.app_pool,
        auth.tenant_id,
        &filters,
        parsed.pagination.cursor,
        parsed.pagination.fetch_size() as i64,
    )
    .await
    .map_err(|_| IntoResponse::into_response(ApiError::internal()))?;

    let items: Vec<AuditEventResponse> = rows.into_iter().map(map_row).collect();

    Ok(Json(build_cursor_page(
        items,
        parsed.pagination.limit,
        |event| event.id,
    )))
}

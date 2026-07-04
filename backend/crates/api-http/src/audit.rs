use axum::{
    Json,
    extract::{Query, State},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::{AuthUser, require_admin};
use crate::error::ApiError;
use crate::pagination::paginate_offset;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ListAuditEventsQuery {
    #[serde(default = "crate::pagination::default_page")]
    pub page: u32,
    #[serde(rename = "pageSize", default = "crate::pagination::default_page_size")]
    pub page_size: u32,
}

#[derive(Serialize)]
pub struct AuditEventResponse {
    pub id: Uuid,
    #[serde(rename = "actorId")]
    pub actor_id: Uuid,
    pub action: String,
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceId")]
    pub resource_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "correlationId", skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<Uuid>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct PaginatedAuditEventsResponse {
    pub items: Vec<AuditEventResponse>,
    pub page: u32,
    #[serde(rename = "pageSize")]
    pub page_size: u32,
    pub total: u64,
}

pub async fn list_audit_events(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<ListAuditEventsQuery>,
) -> Result<Json<PaginatedAuditEventsResponse>, ApiError> {
    require_admin(&auth)?;
    let (page, page_size, offset) = paginate_offset(query.page, query.page_size);

    let rows = infra_postgres::audit::list_audit_events(
        &state.app_pool,
        auth.tenant_id,
        page_size as i64,
        offset,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let total = infra_postgres::audit::count_audit_events(&state.app_pool, auth.tenant_id)
        .await
        .map_err(|_| ApiError::internal())? as u64;

    Ok(Json(PaginatedAuditEventsResponse {
        items: rows
            .into_iter()
            .map(|row| AuditEventResponse {
                id: row.id,
                actor_id: row.actor_id,
                action: row.action,
                resource_type: row.resource_type,
                resource_id: row.resource_id,
                metadata: row.metadata,
                correlation_id: row.correlation_id,
                created_at: row.created_at,
            })
            .collect(),
        page,
        page_size,
        total,
    }))
}

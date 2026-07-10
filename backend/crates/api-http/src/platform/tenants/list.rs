use axum::{Json, extract::Query, extract::State};
use serde::Deserialize;
use uuid::Uuid;

use crate::error::ApiError;
use crate::list_query::build_cursor_page;
use crate::platform::auth::PlatformAuthUser;
use crate::platform::tenants::types::TenantListItem;
use crate::state::AppState;

const DEFAULT_LIMIT: u32 = 20;
const MAX_LIMIT: u32 = 100;

#[derive(Deserialize)]
pub struct TenantsQuery {
    pub limit: Option<u32>,
    pub cursor: Option<Uuid>,
    #[serde(rename = "filter[status]")]
    pub filter_status: Option<String>,
    #[serde(rename = "filter[plan_id]")]
    pub filter_plan_id: Option<Uuid>,
}

pub async fn list_platform_tenants(
    State(state): State<AppState>,
    _auth: PlatformAuthUser,
    Query(query): Query<TenantsQuery>,
) -> Result<Json<crate::list_query::CursorListResponse<TenantListItem>>, ApiError> {
    let limit = query
        .limit
        .unwrap_or(DEFAULT_LIMIT)
        .clamp(1, MAX_LIMIT);

    let rows = infra_postgres::shared::list_tenants_platform(
        &state.app_pool,
        query.filter_status.as_deref(),
        query.filter_plan_id,
        query.cursor,
        i64::from(limit) + 1,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let items: Vec<TenantListItem> = rows
        .into_iter()
        .map(|row| TenantListItem {
            id: row.id.as_uuid(),
            legal_name: row.legal_name,
            display_name: row.display_name,
            status: row.status,
            plan_id: row.plan_id,
            created_at: row.created_at,
        })
        .collect();

    Ok(Json(build_cursor_page(items, limit, |item| item.id)))
}

use axum::{Json, extract::Query, extract::State};
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::error::ApiError;
use crate::list_query::build_cursor_page;
use crate::platform::auth::PlatformAuthUser;
use crate::state::AppState;

const DEFAULT_LIMIT: u32 = 20;
const MAX_LIMIT: u32 = 100;

#[derive(Deserialize)]
pub struct PlatformUsersQuery {
    pub limit: Option<u32>,
    pub cursor: Option<Uuid>,
    #[serde(rename = "filter[tenant_id]")]
    pub filter_tenant_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct PlatformUserItem {
    pub id: Uuid,
    #[serde(rename = "tenantId")]
    pub tenant_id: Uuid,
    pub email: String,
    pub name: String,
    pub role: String,
    pub active: bool,
}

pub async fn list_platform_users(
    State(state): State<AppState>,
    _auth: PlatformAuthUser,
    Query(query): Query<PlatformUsersQuery>,
) -> Result<Json<crate::list_query::CursorListResponse<PlatformUserItem>>, ApiError> {
    let limit = query
        .limit
        .unwrap_or(DEFAULT_LIMIT)
        .clamp(1, MAX_LIMIT);

    let rows = infra_postgres::identity::list_users_cross_tenant(
        &state.app_pool,
        query.filter_tenant_id,
        query.cursor,
        i64::from(limit) + 1,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let items: Vec<PlatformUserItem> = rows
        .into_iter()
        .map(|row| PlatformUserItem {
            id: row.id,
            tenant_id: row.tenant_id,
            email: row.email,
            name: row.name,
            role: row.role,
            active: row.active,
        })
        .collect();

    Ok(Json(build_cursor_page(items, limit, |item| item.id)))
}

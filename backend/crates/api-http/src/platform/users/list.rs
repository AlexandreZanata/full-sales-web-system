use axum::{Json, extract::Query, extract::State};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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
    #[serde(rename = "filter[role]")]
    pub filter_role: Option<String>,
    #[serde(rename = "filter[active]")]
    pub filter_active: Option<bool>,
    #[serde(rename = "filter[email][prefix]")]
    pub filter_email_prefix: Option<String>,
    pub sort: Option<String>,
}

#[derive(Serialize)]
pub struct TenantSummary {
    pub id: Uuid,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[derive(Serialize)]
pub struct PlatformUserItem {
    pub id: Uuid,
    #[serde(rename = "tenantId")]
    pub tenant_id: Uuid,
    pub tenant: TenantSummary,
    pub email: String,
    pub name: String,
    pub role: String,
    pub active: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "lastLoginAt", skip_serializing_if = "Option::is_none")]
    pub last_login_at: Option<DateTime<Utc>>,
}

pub async fn list_platform_users(
    State(state): State<AppState>,
    _auth: PlatformAuthUser,
    Query(query): Query<PlatformUsersQuery>,
) -> Result<Json<crate::list_query::CursorListResponse<PlatformUserItem>>, ApiError> {
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let sort = parse_sort(query.sort.as_deref());
    let rows = infra_postgres::identity::list_users_cross_tenant(
        &state.app_pool,
        infra_postgres::identity::CrossTenantUserFilters {
            tenant_id: query.filter_tenant_id,
            role: query.filter_role.as_deref(),
            active: query.filter_active,
            email_prefix: query.filter_email_prefix.as_deref(),
            sort,
            after_id: query.cursor,
            limit: i64::from(limit) + 1,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let items: Vec<PlatformUserItem> = rows.into_iter().map(to_item).collect();
    Ok(Json(build_cursor_page(items, limit, |item| item.id)))
}

fn parse_sort(raw: Option<&str>) -> infra_postgres::identity::UserSort {
    match raw.unwrap_or("createdAt") {
        "email" => infra_postgres::identity::UserSort::Email,
        "name" => infra_postgres::identity::UserSort::Name,
        _ => infra_postgres::identity::UserSort::CreatedAt,
    }
}

pub fn to_item(row: infra_postgres::identity::CrossTenantUserDetailRow) -> PlatformUserItem {
    PlatformUserItem {
        id: row.id,
        tenant_id: row.tenant_id,
        tenant: TenantSummary {
            id: row.tenant_id,
            display_name: row.tenant_display_name,
        },
        email: row.email,
        name: row.name,
        role: row.role,
        active: row.active,
        created_at: row.created_at,
        last_login_at: row.last_login_at,
    }
}

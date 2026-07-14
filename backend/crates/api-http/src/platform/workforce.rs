use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ApiError;
use crate::list_query::build_cursor_page;
use crate::platform::auth::PlatformAuthUser;
use crate::platform::users::PlatformUserItem;
use crate::state::AppState;

const DEFAULT_LIMIT: u32 = 20;
const MAX_LIMIT: u32 = 100;

#[derive(Deserialize)]
pub(crate) struct WorkforceQuery {
    pub limit: Option<u32>,
    pub cursor: Option<Uuid>,
}

#[derive(Serialize)]
pub struct TenantStatsResponse {
    pub users: i64,
    pub drivers: i64,
    pub sellers: i64,
    pub commerces: i64,
    pub orders: i64,
    #[serde(rename = "mrrMinor")]
    pub mrr_minor: i64,
    #[serde(rename = "mrrCurrency")]
    pub mrr_currency: String,
}

pub async fn list_tenant_workforce(
    State(state): State<AppState>,
    _auth: PlatformAuthUser,
    Path(id): Path<Uuid>,
    Query(query): Query<WorkforceQuery>,
) -> Result<Json<crate::list_query::CursorListResponse<PlatformUserItem>>, ApiError> {
    ensure_tenant(&state, id).await?;
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let rows = infra_postgres::identity::list_tenant_users(
        &state.app_pool,
        domain_shared::TenantId::from_uuid(id),
        query.cursor,
        i64::from(limit) + 1,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    let items: Vec<PlatformUserItem> = rows.into_iter().map(super::users::to_item).collect();
    Ok(Json(build_cursor_page(items, limit, |item| item.id)))
}

pub async fn get_tenant_stats(
    State(state): State<AppState>,
    _auth: PlatformAuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<TenantStatsResponse>, ApiError> {
    ensure_tenant(&state, id).await?;
    let stats = infra_postgres::shared::tenant_workforce_stats(
        &state.admin_pool,
        domain_shared::TenantId::from_uuid(id),
    )
    .await
    .map_err(|_| ApiError::internal())?;
    Ok(Json(TenantStatsResponse {
        users: stats.users,
        drivers: stats.drivers,
        sellers: stats.sellers,
        commerces: stats.commerces,
        orders: stats.orders,
        mrr_minor: stats.mrr_minor,
        mrr_currency: stats.mrr_currency,
    }))
}

async fn ensure_tenant(state: &AppState, id: Uuid) -> Result<(), ApiError> {
    let exists = infra_postgres::identity::tenant_exists(&state.admin_pool, id)
        .await
        .map_err(|_| ApiError::internal())?;
    if !exists {
        return Err(ApiError::not_found());
    }
    Ok(())
}

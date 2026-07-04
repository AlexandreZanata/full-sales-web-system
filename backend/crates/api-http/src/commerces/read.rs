use axum::{
    Json,
    extract::{Path, Query, State},
};
use domain_identity::Role;
use serde::Deserialize;

use crate::auth::{require_admin, require_roles, AuthUser};
use crate::commerces::{commerce_response_from_row, CommerceResponse};
use crate::error::ApiError;
use crate::pagination::paginate_offset;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ListCommercesQuery {
    #[serde(default = "crate::pagination::default_page")]
    pub page: u32,
    #[serde(rename = "pageSize", default = "crate::pagination::default_page_size")]
    pub page_size: u32,
    pub active: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct PaginatedCommercesResponse {
    pub items: Vec<CommerceResponse>,
    pub page: u32,
    #[serde(rename = "pageSize")]
    pub page_size: u32,
    pub total: u64,
}

pub async fn list_commerces(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<ListCommercesQuery>,
) -> Result<Json<PaginatedCommercesResponse>, ApiError> {
    require_roles(&auth, &[Role::Admin, Role::Driver, Role::Seller])?;
    let (page, page_size, offset) = paginate_offset(query.page, query.page_size);

    let rows = infra_postgres::commerces::list_commerces(
        &state.app_pool,
        auth.tenant_id,
        page_size as i64,
        offset,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let items: Vec<CommerceResponse> = rows
        .iter()
        .filter(|row| query.active.is_none_or(|active| row.active == active))
        .map(commerce_response_from_row)
        .collect();

    let total = infra_postgres::commerces::count_commerces(&state.app_pool, auth.tenant_id)
        .await
        .map_err(|_| ApiError::internal())? as u64;

    Ok(Json(PaginatedCommercesResponse {
        items,
        page,
        page_size,
        total,
    }))
}

pub async fn get_commerce(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<CommerceResponse>, ApiError> {
    require_roles(&auth, &[Role::Admin, Role::Driver, Role::Seller])?;
    let row = infra_postgres::commerces::find_commerce_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::commerce_not_found)?;
    Ok(Json(commerce_response_from_row(&row)))
}

pub async fn deactivate_commerce(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<CommerceResponse>, ApiError> {
    require_admin(&auth)?;
    let existing = infra_postgres::commerces::find_commerce_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::commerce_not_found)?;

    let _ = infra_postgres::commerces::deactivate_commerce(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?;

    Ok(Json(commerce_response_from_row(
        &infra_postgres::commerces::CommerceRow {
            active: false,
            ..existing
        },
    )))
}

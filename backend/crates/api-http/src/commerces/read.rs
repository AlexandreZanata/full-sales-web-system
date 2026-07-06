use axum::{
    Json,
    extract::{Path, RawQuery, State},
    response::{IntoResponse, Response},
};
use domain_identity::Role;

use crate::auth::{AuthUser, require_admin, require_roles};
use crate::commerces::{CommerceResponse, commerce_response_from_row};
use crate::error::ApiError;
use crate::list_query::{
    COMMERCES_LIST_CONFIG, CursorListResponse, build_cursor_page, decode_query_pairs,
    filter_eq_bool, parse_list_query,
};
use crate::state::AppState;

pub async fn list_commerces(
    State(state): State<AppState>,
    auth: AuthUser,
    RawQuery(query): RawQuery,
) -> Result<Json<CursorListResponse<CommerceResponse>>, Response> {
    require_roles(&auth, &[Role::Admin, Role::Driver, Role::Seller])
        .map_err(IntoResponse::into_response)?;
    let parsed = parse_list_query(
        &decode_query_pairs(query.as_deref()),
        &COMMERCES_LIST_CONFIG,
    )
    .map_err(IntoResponse::into_response)?;
    let active = filter_eq_bool(&parsed.filters, "active");
    let rows = infra_postgres::commerces::list_commerces_cursor(
        &state.app_pool,
        auth.tenant_id,
        active,
        parsed.pagination.cursor,
        parsed.pagination.fetch_size() as i64,
    )
    .await
    .map_err(|_| IntoResponse::into_response(ApiError::internal()))?;

    let items: Vec<CommerceResponse> = rows.iter().map(commerce_response_from_row).collect();
    Ok(Json(build_cursor_page(
        items,
        parsed.pagination.limit,
        |commerce| commerce.id,
    )))
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
    let existing =
        infra_postgres::commerces::find_commerce_by_id(&state.app_pool, auth.tenant_id, id)
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

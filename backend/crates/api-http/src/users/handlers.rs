use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use infra_crypto::PasswordHasher;
use uuid::Uuid;

use crate::auth::{AuthUser, require_admin};
use crate::error::ApiError;
use crate::pagination::{PaginationQuery, paginate_offset};
use crate::state::AppState;
use crate::users::types::{
    CreateUserRequest, PaginatedUsersResponse, UserResponse, map_user_app_error,
    user_response_from_detail, user_response_from_list,
};

pub async fn create_user(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), ApiError> {
    require_admin(&auth)?;

    let user = application::parse_register_user(
        &body.name,
        &body.email,
        &body.role,
        auth.tenant_id,
        body.commerce_id,
    )
    .map_err(map_user_app_error)?;

    let password_hash = PasswordHasher::hash(&body.password).map_err(|_| ApiError::internal())?;

    infra_postgres::identity::insert_user(
        &state.app_pool,
        auth.tenant_id,
        infra_postgres::identity::InsertUserParams {
            id: user.id().as_uuid(),
            email: user.email().as_str(),
            name: user.name().as_str(),
            role: user.role().as_str(),
            password_hash: &password_hash,
            commerce_id: user.commerce_id(),
            profile_file_id: user.profile_file_id(),
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;

    Ok((
        StatusCode::CREATED,
        Json(user_response_from_detail(
            &infra_postgres::identity::UserDetailRow {
                id: user.id().as_uuid(),
                email: user.email().as_str().to_owned(),
                name: user.name().as_str().to_owned(),
                role: user.role().as_str().to_owned(),
                active: user.is_active(),
                commerce_id: user.commerce_id(),
            },
        )),
    ))
}

pub async fn list_users(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<PaginatedUsersResponse>, ApiError> {
    require_admin(&auth)?;
    let (page, page_size, offset) = paginate_offset(query.page, query.page_size);

    let rows = infra_postgres::identity::list_users(
        &state.app_pool,
        auth.tenant_id,
        page_size as i64,
        offset,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let total = infra_postgres::identity::count_users(&state.app_pool, auth.tenant_id)
        .await
        .map_err(|_| ApiError::internal())? as u64;

    Ok(Json(PaginatedUsersResponse {
        items: rows.iter().map(user_response_from_list).collect(),
        page,
        page_size,
        total,
    }))
}

pub async fn get_user(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    require_admin(&auth)?;
    let row = infra_postgres::identity::find_user_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::user_not_found)?;
    Ok(Json(user_response_from_detail(&row)))
}

pub async fn deactivate_user(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    require_admin(&auth)?;
    let existing = infra_postgres::identity::find_user_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::user_not_found)?;

    let _ = infra_postgres::identity::deactivate_user_tenant(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?;

    Ok(Json(user_response_from_detail(
        &infra_postgres::identity::UserDetailRow {
            active: false,
            ..existing
        },
    )))
}

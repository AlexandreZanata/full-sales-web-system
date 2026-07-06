use axum::{
    Json,
    extract::{Path, RawQuery, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use domain_identity::Role;
use infra_crypto::PasswordHasher;
use uuid::Uuid;

use crate::auth::{AuthUser, require_admin};
use crate::error::ApiError;
use crate::list_query::{
    USERS_LIST_CONFIG, CursorListResponse, build_cursor_page, decode_query_pairs, filter_eq_bool,
    filter_eq_string, parse_list_query,
};
use crate::state::AppState;
use crate::users::types::{
    CreateUserRequest, UserResponse, map_user_app_error, user_response_from_detail,
    user_response_from_list,
};

fn parse_role_filter(
    filters: &[application::list_query::ListFilter],
) -> Result<Option<String>, Response> {
    match filter_eq_string(filters, "role") {
        None => Ok(None),
        Some(value) => Role::parse(&value)
            .map(|role| Some(role.as_str().to_owned()))
            .map_err(|_| {
                IntoResponse::into_response(crate::list_query::ListQueryApiError {
                    status: StatusCode::BAD_REQUEST,
                    code: "invalid_filter_field",
                    message: "invalid role filter value",
                    field: "role".into(),
                })
            }),
    }
}

pub async fn list_users(
    State(state): State<AppState>,
    auth: AuthUser,
    RawQuery(query): RawQuery,
) -> Result<Json<CursorListResponse<UserResponse>>, Response> {
    require_admin(&auth).map_err(IntoResponse::into_response)?;
    let parsed = parse_list_query(&decode_query_pairs(query.as_deref()), &USERS_LIST_CONFIG)
        .map_err(IntoResponse::into_response)?;
    let active = filter_eq_bool(&parsed.filters, "active");
    let role = parse_role_filter(&parsed.filters)?;
    let rows = infra_postgres::identity::list_users_cursor(
        &state.app_pool,
        auth.tenant_id,
        active,
        role.as_deref(),
        parsed.pagination.cursor,
        parsed.pagination.fetch_size() as i64,
    )
    .await
    .map_err(|_| IntoResponse::into_response(ApiError::internal()))?;

    Ok(Json(build_cursor_page(
        rows.iter().map(user_response_from_list).collect(),
        parsed.pagination.limit,
        |user| user.id,
    )))
}

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

use axum::{
    Json,
    extract::{Path, State},
};
use domain_identity::{Role, normalize_contact_phone, normalize_public_code, slug_from_name};
use uuid::Uuid;

use crate::auth::{AuthUser, require_admin, require_roles};
use crate::error::ApiError;
use crate::state::AppState;
use crate::users::types::{
    SellerProfileRequest, SellerProfileResponse, SellerShareResponse, load_user,
};

pub async fn get_seller_profile(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<SellerProfileResponse>, ApiError> {
    require_admin(&auth)?;
    ensure_seller(&state, auth.tenant_id, id).await?;
    let profile = infra_postgres::identity::find_seller_profile_by_user_id(
        &state.app_pool,
        auth.tenant_id,
        id,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    Ok(Json(seller_response(id, profile)))
}

pub async fn upsert_seller_profile(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<SellerProfileRequest>,
) -> Result<Json<SellerProfileResponse>, ApiError> {
    require_admin(&auth)?;
    let user = ensure_seller(&state, auth.tenant_id, id).await?;
    let existing = infra_postgres::identity::find_seller_profile_by_user_id(
        &state.app_pool,
        auth.tenant_id,
        id,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let insert = merge_seller_insert(id, &user.name, &body, existing.as_ref())?;
    infra_postgres::identity::upsert_seller_profile(&state.app_pool, auth.tenant_id, insert)
        .await
        .map_err(map_profile_db_error)?;

    let profile = infra_postgres::identity::find_seller_profile_by_user_id(
        &state.app_pool,
        auth.tenant_id,
        id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::internal)?;

    Ok(Json(seller_response_from_row(&profile)))
}

pub async fn get_my_seller_share(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<SellerShareResponse>, ApiError> {
    require_roles(&auth, &[Role::Seller])?;
    let profile = infra_postgres::identity::find_seller_profile_by_user_id(
        &state.app_pool,
        auth.tenant_id,
        auth.user_id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::not_found)?;

    let code = profile
        .public_code
        .clone()
        .ok_or_else(ApiError::not_found)?;
    let share_path = format!("/s/{code}");
    let share_url = absolute_catalog_share_url(&state.portal_public_origin, &share_path);
    Ok(Json(SellerShareResponse {
        public_code: code,
        share_path,
        share_url,
        contact_phone: profile.contact_phone,
        share_link_active: profile.share_link_active,
    }))
}

fn absolute_catalog_share_url(portal_origin: &str, share_path: &str) -> String {
    let origin = portal_origin.trim_end_matches('/');
    let path = if share_path.starts_with('/') {
        share_path.to_string()
    } else {
        format!("/{share_path}")
    };
    format!("{origin}{path}")
}

async fn ensure_seller(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
    id: Uuid,
) -> Result<infra_postgres::identity::UserDetailRow, ApiError> {
    let user = load_user(state, tenant_id, id).await?;
    if user.role != Role::Seller.as_str() {
        return Err(ApiError::bad_request(
            "INVALID_INPUT",
            "Seller profile applies only to Seller role",
        ));
    }
    Ok(user)
}

pub(crate) fn merge_seller_insert(
    user_id: Uuid,
    user_name: &str,
    body: &SellerProfileRequest,
    existing: Option<&infra_postgres::identity::SellerProfileRow>,
) -> Result<infra_postgres::identity::SellerProfileInsert, ApiError> {
    let public_code = match &body.public_code {
        Some(raw) if raw.trim().is_empty() => existing.and_then(|e| e.public_code.clone()),
        Some(raw) => Some(
            normalize_public_code(raw)
                .map_err(|_| ApiError::bad_request("VALIDATION_ERROR", "Invalid publicCode"))?,
        ),
        None => existing
            .and_then(|e| e.public_code.clone())
            .or_else(|| Some(slug_from_name(user_name))),
    };
    let contact_phone = match &body.contact_phone {
        Some(raw) => normalize_contact_phone(raw)
            .map_err(|_| ApiError::bad_request("VALIDATION_ERROR", "Invalid contactPhone"))?,
        None => existing.and_then(|e| e.contact_phone.clone()),
    };
    let share_link_active = body
        .share_link_active
        .unwrap_or_else(|| existing.map(|e| e.share_link_active).unwrap_or(true));

    Ok(infra_postgres::identity::SellerProfileInsert {
        user_id,
        operating_region: body.operating_region.clone(),
        monthly_target_amount: body.monthly_target_amount,
        public_code,
        contact_phone,
        share_link_active,
    })
}

pub(crate) fn seller_response(
    user_id: Uuid,
    profile: Option<infra_postgres::identity::SellerProfileRow>,
) -> SellerProfileResponse {
    match profile {
        Some(row) => seller_response_from_row(&row),
        None => SellerProfileResponse {
            user_id,
            operating_region: None,
            monthly_target_amount: None,
            public_code: None,
            contact_phone: None,
            share_link_active: true,
        },
    }
}

pub(crate) fn seller_response_from_row(
    profile: &infra_postgres::identity::SellerProfileRow,
) -> SellerProfileResponse {
    SellerProfileResponse {
        user_id: profile.user_id,
        operating_region: profile.operating_region.clone(),
        monthly_target_amount: profile.monthly_target_amount,
        public_code: profile.public_code.clone(),
        contact_phone: profile.contact_phone.clone(),
        share_link_active: profile.share_link_active,
    }
}

pub(crate) fn map_profile_db_error(err: infra_postgres::PostgresError) -> ApiError {
    let msg = err.to_string();
    if msg.contains("idx_seller_profiles_tenant_public_code") || msg.contains("duplicate key") {
        return ApiError::bad_request("CONFLICT", "publicCode already in use");
    }
    ApiError::internal()
}

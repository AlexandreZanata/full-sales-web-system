//! Seller self-service profile (Phase 19 — OD-19-3 B).

use axum::{Json, extract::State};
use domain_identity::Role;
use serde::Deserialize;

use crate::auth::{AuthUser, require_roles};
use crate::error::ApiError;
use crate::state::AppState;
use crate::users::seller_profile::{
    merge_seller_insert, seller_response, seller_response_from_row,
};
use crate::users::types::{SellerProfileRequest, SellerProfileResponse};

#[derive(Deserialize)]
pub struct SellerSelfProfileRequest {
    #[serde(rename = "operatingRegion")]
    pub operating_region: Option<String>,
    #[serde(rename = "contactPhone")]
    pub contact_phone: Option<String>,
    #[serde(rename = "shareLinkActive")]
    pub share_link_active: Option<bool>,
}

pub async fn get_my_seller_profile(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<SellerProfileResponse>, ApiError> {
    require_roles(&auth, &[Role::Seller])?;
    let profile = infra_postgres::identity::find_seller_profile_by_user_id(
        &state.app_pool,
        auth.tenant_id,
        auth.user_id,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    Ok(Json(seller_response(auth.user_id, profile)))
}

pub async fn patch_my_seller_profile(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<SellerSelfProfileRequest>,
) -> Result<Json<SellerProfileResponse>, ApiError> {
    require_roles(&auth, &[Role::Seller])?;
    let user = infra_postgres::identity::find_user_by_id(
        &state.app_pool,
        auth.tenant_id,
        auth.user_id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::user_not_found)?;

    let existing = infra_postgres::identity::find_seller_profile_by_user_id(
        &state.app_pool,
        auth.tenant_id,
        auth.user_id,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let admin_body = SellerProfileRequest {
        operating_region: body.operating_region.or_else(|| {
            existing.as_ref().and_then(|e| e.operating_region.clone())
        }),
        monthly_target_amount: existing.as_ref().and_then(|e| e.monthly_target_amount),
        public_code: None,
        contact_phone: body.contact_phone,
        share_link_active: body.share_link_active,
    };

    let insert = merge_seller_insert(auth.user_id, &user.name, &admin_body, existing.as_ref())?;
    infra_postgres::identity::upsert_seller_profile(&state.app_pool, auth.tenant_id, insert)
        .await
        .map_err(crate::users::seller_profile::map_profile_db_error)?;

    let profile = infra_postgres::identity::find_seller_profile_by_user_id(
        &state.app_pool,
        auth.tenant_id,
        auth.user_id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::internal)?;

    Ok(Json(seller_response_from_row(&profile)))
}

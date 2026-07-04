use axum::{
    Json,
    extract::{Path, State},
};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::{require_admin, AuthUser};
use crate::commerces::addresses_support::{
    address_response_from_row, ensure_commerce, AddressResponse,
};
use crate::commerces::CommerceResponse;
use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize, Default)]
pub struct UpdateAddressRequest {
    pub street: Option<String>,
    pub number: Option<String>,
    pub district: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    #[serde(rename = "isPrimary")]
    pub is_primary: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdateLogoRequest {
    #[serde(rename = "fileId")]
    pub file_id: Uuid,
}

pub async fn update_address(
    State(state): State<AppState>,
    auth: AuthUser,
    Path((commerce_id, address_id)): Path<(Uuid, Uuid)>,
    Json(body): Json<UpdateAddressRequest>,
) -> Result<Json<AddressResponse>, ApiError> {
    require_admin(&auth)?;
    ensure_commerce(&state, auth.tenant_id, commerce_id).await?;
    let existing = infra_postgres::commerces::addresses::find_address_by_id(
        &state.app_pool,
        auth.tenant_id,
        address_id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::invalid_delivery_address)?;

    if existing.commerce_id != commerce_id {
        return Err(ApiError::invalid_delivery_address());
    }

    let updated = infra_postgres::commerces::addresses::update_address(
        &state.app_pool,
        auth.tenant_id,
        address_id,
        &infra_postgres::commerces::addresses::AddressUpdate {
            street: body.street,
            number: body.number,
            district: body.district,
            city: body.city,
            state: body.state,
            postal_code: body.postal_code,
            latitude: body.latitude,
            longitude: body.longitude,
            is_primary: body.is_primary,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;
    if !updated {
        return Err(ApiError::invalid_delivery_address());
    }

    let row = infra_postgres::commerces::addresses::find_address_by_id(
        &state.app_pool,
        auth.tenant_id,
        address_id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::internal)?;
    Ok(Json(address_response_from_row(&row)))
}

pub async fn update_logo(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(commerce_id): Path<Uuid>,
    Json(body): Json<UpdateLogoRequest>,
) -> Result<Json<CommerceResponse>, ApiError> {
    require_admin(&auth)?;
    ensure_commerce(&state, auth.tenant_id, commerce_id).await?;
    infra_postgres::commerces::addresses::update_commerce_logo(
        &state.app_pool,
        auth.tenant_id,
        commerce_id,
        Some(body.file_id),
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let row = infra_postgres::commerces::find_commerce_by_id(&state.app_pool, auth.tenant_id, commerce_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::commerce_not_found)?;
    Ok(Json(super::commerce_response_from_row(&row)))
}

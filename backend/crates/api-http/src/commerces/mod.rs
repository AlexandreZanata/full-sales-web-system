use axum::{Json, extract::State};
use domain_commerces::CommerceError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::{AuthUser, require_admin};
use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateCommerceRequest {
    pub cnpj: String,
    #[serde(rename = "legalName")]
    pub legal_name: String,
    #[serde(rename = "tradeName")]
    pub trade_name: Option<String>,
    pub address: serde_json::Value,
    pub contact: serde_json::Value,
}

#[derive(Serialize)]
pub struct CommerceResponse {
    pub id: Uuid,
    pub cnpj: String,
    #[serde(rename = "legalName")]
    pub legal_name: String,
    #[serde(rename = "tradeName")]
    pub trade_name: String,
    pub active: bool,
    #[serde(rename = "logoFileId", skip_serializing_if = "Option::is_none")]
    pub logo_file_id: Option<Uuid>,
}

pub async fn create_commerce(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<CreateCommerceRequest>,
) -> Result<(http::StatusCode, Json<CommerceResponse>), ApiError> {
    require_admin(&auth)?;

    let commerce = application::parse_create_commerce(
        &body.cnpj,
        &body.legal_name,
        body.trade_name.as_deref(),
        auth.tenant_id,
    )
    .map_err(map_commerce_error)?;

    let mut address = body.address;
    if let Some(obj) = address.as_object_mut() {
        obj.insert("contact".into(), body.contact);
    }

    infra_postgres::commerces::insert_commerce(
        &state.app_pool,
        auth.tenant_id,
        commerce.id().as_uuid(),
        commerce.cnpj().as_str(),
        commerce.legal_name(),
        commerce.trade_name(),
        address,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    Ok((
        http::StatusCode::CREATED,
        Json(commerce_response_from_row(
            &infra_postgres::commerces::CommerceRow {
                id: commerce.id().as_uuid(),
                cnpj: commerce.cnpj().as_str().to_owned(),
                legal_name: commerce.legal_name().to_owned(),
                trade_name: commerce.trade_name().to_owned(),
                active: commerce.is_active(),
                logo_file_id: None,
                registration_status: "Active".into(),
                submitted_by_user_id: None,
                reviewed_by_user_id: None,
                rejection_reason: None,
                lookup_snapshot: None,
                registration_mode: None,
            },
        )),
    ))
}

pub(crate) fn commerce_response_from_row(
    row: &infra_postgres::commerces::CommerceRow,
) -> CommerceResponse {
    CommerceResponse {
        id: row.id,
        cnpj: row.cnpj.clone(),
        legal_name: row.legal_name.clone(),
        trade_name: row.trade_name.clone(),
        active: row.active,
        logo_file_id: row.logo_file_id,
    }
}

fn map_commerce_error(err: application::AppError) -> ApiError {
    match err {
        application::AppError::Commerce(CommerceError::InvalidCnpj) => ApiError::invalid_cnpj(),
        application::AppError::Forbidden => ApiError::forbidden(),
        _ => ApiError::bad_request("INVALID_INPUT", "Invalid request"),
    }
}

mod addresses_handlers;
mod addresses_manage;
mod addresses_support;
mod cnpj_lookup_handler;
mod read;
mod registrations;

pub use addresses_handlers::{create_address, list_addresses};
pub use addresses_manage::{update_address, update_logo};
pub use cnpj_lookup_handler::lookup_cnpj;
pub use read::{activate_commerce, deactivate_commerce, get_commerce, list_commerces};
pub use registrations::{
    approve_registration, get_registration, list_registrations, patch_registration,
    reject_registration, submit_registration,
};

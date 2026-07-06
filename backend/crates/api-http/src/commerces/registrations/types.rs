use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct RegistrationResponse {
    pub id: Uuid,
    pub cnpj: String,
    #[serde(rename = "legalName")]
    pub legal_name: String,
    #[serde(rename = "tradeName")]
    pub trade_name: String,
    pub active: bool,
    #[serde(rename = "registrationStatus")]
    pub registration_status: String,
    #[serde(rename = "submittedByUserId", skip_serializing_if = "Option::is_none")]
    pub submitted_by_user_id: Option<Uuid>,
    #[serde(rename = "reviewedByUserId", skip_serializing_if = "Option::is_none")]
    pub reviewed_by_user_id: Option<Uuid>,
    #[serde(rename = "rejectionReason", skip_serializing_if = "Option::is_none")]
    pub rejection_reason: Option<String>,
    #[serde(rename = "registrationMode", skip_serializing_if = "Option::is_none")]
    pub registration_mode: Option<String>,
    #[serde(rename = "lookupSnapshot", skip_serializing_if = "Option::is_none")]
    pub lookup_snapshot: Option<serde_json::Value>,
    #[serde(rename = "logoFileId", skip_serializing_if = "Option::is_none")]
    pub logo_file_id: Option<Uuid>,
}

pub(crate) fn registration_response_from_row(
    row: &infra_postgres::commerces::CommerceRow,
) -> RegistrationResponse {
    RegistrationResponse {
        id: row.id,
        cnpj: row.cnpj.clone(),
        legal_name: row.legal_name.clone(),
        trade_name: row.trade_name.clone(),
        active: row.active,
        registration_status: row.registration_status.clone(),
        submitted_by_user_id: row.submitted_by_user_id,
        reviewed_by_user_id: row.reviewed_by_user_id,
        rejection_reason: row.rejection_reason.clone(),
        registration_mode: row.registration_mode.clone(),
        lookup_snapshot: row.lookup_snapshot.clone(),
        logo_file_id: row.logo_file_id,
    }
}

#[derive(Deserialize)]
pub struct DeliveryAddressRequest {
    pub street: String,
    pub number: String,
    pub district: Option<String>,
    pub city: String,
    pub state: String,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
    #[serde(rename = "isPrimary", default)]
    pub is_primary: bool,
}

#[derive(Deserialize)]
pub struct ContactRequest {
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Deserialize)]
pub struct SubmitRegistrationRequest {
    pub cnpj: String,
    #[serde(rename = "legalName")]
    pub legal_name: String,
    #[serde(rename = "tradeName")]
    pub trade_name: Option<String>,
    pub contact: ContactRequest,
    #[serde(rename = "deliveryAddress")]
    pub delivery_address: DeliveryAddressRequest,
    #[serde(rename = "registrationMode")]
    pub registration_mode: String,
    #[serde(rename = "lookupSnapshot")]
    pub lookup_snapshot: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct PatchRegistrationRequest {
    #[serde(rename = "legalName")]
    pub legal_name: Option<String>,
    #[serde(rename = "tradeName")]
    pub trade_name: Option<String>,
    pub contact: Option<ContactRequest>,
}

#[derive(Deserialize)]
pub struct RejectRegistrationRequest {
    pub reason: String,
}

pub(crate) fn map_registration_error(err: application::AppError) -> crate::error::ApiError {
    use application::AppError;
    use domain_commerces::CommerceError;
    match err {
        AppError::Commerce(CommerceError::InvalidCnpj) => crate::error::ApiError::invalid_cnpj(),
        AppError::Commerce(CommerceError::InvalidRegistrationMode) => {
            crate::error::ApiError::bad_request("INVALID_REGISTRATION_MODE", "Invalid mode")
        }
        AppError::Commerce(CommerceError::InvalidRegistrationTransition) => {
            crate::error::ApiError::invalid_registration_transition()
        }
        AppError::Commerce(CommerceError::RegistrationNotEditable) => {
            crate::error::ApiError::registration_not_editable()
        }
        AppError::Commerce(CommerceError::RejectionReasonRequired) => {
            crate::error::ApiError::rejection_reason_required()
        }
        AppError::Forbidden => crate::error::ApiError::forbidden(),
        _ => crate::error::ApiError::bad_request("INVALID_INPUT", "Invalid request"),
    }
}

pub(crate) fn map_commerce_domain_error(
    err: domain_commerces::CommerceError,
) -> crate::error::ApiError {
    map_registration_error(application::AppError::Commerce(err))
}

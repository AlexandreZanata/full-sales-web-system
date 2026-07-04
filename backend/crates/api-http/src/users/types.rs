use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ApiError;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    #[serde(rename = "commerceId")]
    pub commerce_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: String,
    pub active: bool,
    #[serde(rename = "commerceId", skip_serializing_if = "Option::is_none")]
    pub commerce_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct PaginatedUsersResponse {
    pub items: Vec<UserResponse>,
    pub page: u32,
    #[serde(rename = "pageSize")]
    pub page_size: u32,
    pub total: u64,
}

#[derive(Deserialize)]
pub struct DriverProfileRequest {
    #[serde(rename = "cnhNumber")]
    pub cnh_number: String,
    #[serde(rename = "cnhCategory")]
    pub cnh_category: String,
    #[serde(rename = "cnhPhotoFileId")]
    pub cnh_photo_file_id: Option<Uuid>,
    #[serde(rename = "vehiclePlate")]
    pub vehicle_plate: String,
    #[serde(rename = "vehicleModel")]
    pub vehicle_model: String,
    #[serde(rename = "vehicleCapacityKg")]
    pub vehicle_capacity_kg: Option<f64>,
}

#[derive(Serialize)]
pub struct DriverProfileResponse {
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    #[serde(rename = "cnhNumber")]
    pub cnh_number: String,
    #[serde(rename = "cnhCategory")]
    pub cnh_category: String,
    #[serde(rename = "cnhPhotoFileId")]
    pub cnh_photo_file_id: Option<Uuid>,
    #[serde(rename = "vehiclePlate")]
    pub vehicle_plate: String,
    #[serde(rename = "vehicleModel")]
    pub vehicle_model: String,
}

#[derive(Deserialize)]
pub struct SellerProfileRequest {
    #[serde(rename = "operatingRegion")]
    pub operating_region: Option<String>,
    #[serde(rename = "monthlyTargetAmount")]
    pub monthly_target_amount: Option<i64>,
}

#[derive(Serialize)]
pub struct SellerProfileResponse {
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    #[serde(rename = "operatingRegion")]
    pub operating_region: Option<String>,
    #[serde(rename = "monthlyTargetAmount")]
    pub monthly_target_amount: Option<i64>,
}

pub(crate) fn user_response_from_list(row: &infra_postgres::identity::UserListRow) -> UserResponse {
    UserResponse {
        id: row.id,
        name: row.name.clone(),
        email: row.email.clone(),
        role: row.role.clone(),
        active: row.active,
        commerce_id: None,
    }
}

pub(crate) fn user_response_from_detail(
    row: &infra_postgres::identity::UserDetailRow,
) -> UserResponse {
    UserResponse {
        id: row.id,
        name: row.name.clone(),
        email: row.email.clone(),
        role: row.role.clone(),
        active: row.active,
        commerce_id: row.commerce_id,
    }
}

pub(crate) fn map_user_app_error(err: application::AppError) -> ApiError {
    match err {
        application::AppError::Identity(domain_identity::IdentityError::CommerceRequired) => {
            ApiError::bad_request(
                "COMMERCE_REQUIRED",
                "commerceId is required for CommerceContact",
            )
        }
        application::AppError::Identity(domain_identity::IdentityError::InvalidCommerceScope) => {
            ApiError::bad_request(
                "INVALID_COMMERCE_SCOPE",
                "commerceId must be null for this role",
            )
        }
        application::AppError::Identity(domain_identity::IdentityError::InvalidRole) => {
            ApiError::bad_request("INVALID_ROLE", "Invalid role")
        }
        application::AppError::Identity(domain_identity::IdentityError::InvalidEmail) => {
            ApiError::bad_request("INVALID_EMAIL", "Invalid email address")
        }
        application::AppError::Identity(domain_identity::IdentityError::InvalidFullName) => {
            ApiError::bad_request("INVALID_NAME", "Name must contain at least two parts")
        }
        _ => ApiError::bad_request("INVALID_INPUT", "Invalid user request"),
    }
}

pub(crate) async fn load_user(
    state: &crate::state::AppState,
    tenant_id: domain_shared::TenantId,
    id: Uuid,
) -> Result<infra_postgres::identity::UserDetailRow, ApiError> {
    infra_postgres::identity::find_user_by_id(&state.app_pool, tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::user_not_found)
}

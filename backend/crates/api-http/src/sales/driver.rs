use domain_identity::Role;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::deliveries::support::ensure_active_driver;
use crate::error::ApiError;
use crate::state::AppState;

pub async fn resolve_sale_driver_id(
    state: &AppState,
    auth: &AuthUser,
    body_driver_id: Option<Uuid>,
) -> Result<Uuid, ApiError> {
    match auth.role {
        Role::Admin => {
            let driver_id = body_driver_id
                .ok_or_else(|| ApiError::bad_request("VALIDATION_ERROR", "driverId is required"))?;
            ensure_active_driver(state, auth.tenant_id, driver_id).await?;
            Ok(driver_id)
        }
        Role::Driver | Role::Seller => {
            if body_driver_id.is_some_and(|id| id != auth.user_id) {
                return Err(ApiError::bad_request(
                    "VALIDATION_ERROR",
                    "driverId must not be set for this role",
                ));
            }
            Ok(auth.user_id)
        }
        Role::CommerceContact => Err(ApiError::forbidden()),
    }
}

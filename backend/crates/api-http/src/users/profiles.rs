use axum::{
    Json,
    extract::{Path, State},
};
use domain_identity::Role;
use uuid::Uuid;

use crate::auth::{AuthUser, require_admin};
use crate::error::ApiError;
use crate::state::AppState;
use crate::users::types::{DriverProfileRequest, DriverProfileResponse, load_user};

pub async fn upsert_driver_profile(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<DriverProfileRequest>,
) -> Result<Json<DriverProfileResponse>, ApiError> {
    require_admin(&auth)?;
    let user = load_user(&state, auth.tenant_id, id).await?;
    if user.role != Role::Driver.as_str() {
        return Err(ApiError::bad_request(
            "INVALID_INPUT",
            "Driver profile applies only to Driver role",
        ));
    }

    infra_postgres::identity::upsert_driver_profile(
        &state.app_pool,
        auth.tenant_id,
        infra_postgres::identity::DriverProfileInsert {
            user_id: id,
            cnh_number: body.cnh_number,
            cnh_category: body.cnh_category,
            cnh_photo_file_id: body.cnh_photo_file_id,
            vehicle_plate: body.vehicle_plate,
            vehicle_model: body.vehicle_model,
            vehicle_capacity_kg: body.vehicle_capacity_kg,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let profile = infra_postgres::identity::find_driver_profile_by_user_id(
        &state.app_pool,
        auth.tenant_id,
        id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::internal)?;

    Ok(Json(DriverProfileResponse {
        user_id: profile.user_id,
        cnh_number: profile.cnh_number,
        cnh_category: profile.cnh_category,
        cnh_photo_file_id: profile.cnh_photo_file_id,
        vehicle_plate: profile.vehicle_plate,
        vehicle_model: profile.vehicle_model,
    }))
}

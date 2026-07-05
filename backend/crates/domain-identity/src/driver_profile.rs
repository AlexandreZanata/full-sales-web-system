use uuid::Uuid;

use crate::error::IdentityError;
use crate::user_id::UserId;

pub struct DriverProfileInput {
    pub user_id: UserId,
    pub cnh_number: String,
    pub cnh_category: String,
    pub cnh_photo_file_id: Option<Uuid>,
    pub vehicle_plate: String,
    pub vehicle_model: String,
    pub vehicle_capacity_kg: Option<f64>,
}

/// Driver extension profile — 1:1 with User where role = Driver.
#[derive(Debug, Clone)]
pub struct DriverProfile {
    user_id: UserId,
    cnh_number: String,
    cnh_category: String,
    cnh_photo_file_id: Option<Uuid>,
    vehicle_plate: String,
    vehicle_model: String,
    vehicle_capacity_kg: Option<f64>,
}

impl DriverProfile {
    pub fn create(input: DriverProfileInput) -> Result<Self, IdentityError> {
        let cnh_number = input.cnh_number.trim();
        let cnh_category = input.cnh_category.trim();
        let vehicle_plate = input.vehicle_plate.trim();
        let vehicle_model = input.vehicle_model.trim();

        if cnh_number.is_empty()
            || cnh_category.is_empty()
            || vehicle_plate.is_empty()
            || vehicle_model.is_empty()
        {
            return Err(IdentityError::InvalidProfileField);
        }

        Ok(Self {
            user_id: input.user_id,
            cnh_number: cnh_number.to_owned(),
            cnh_category: cnh_category.to_owned(),
            cnh_photo_file_id: input.cnh_photo_file_id,
            vehicle_plate: vehicle_plate.to_owned(),
            vehicle_model: vehicle_model.to_owned(),
            vehicle_capacity_kg: input.vehicle_capacity_kg,
        })
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn cnh_photo_file_id(&self) -> Option<Uuid> {
        self.cnh_photo_file_id
    }

    pub fn cnh_number(&self) -> &str {
        &self.cnh_number
    }

    pub fn cnh_category(&self) -> &str {
        &self.cnh_category
    }

    pub fn vehicle_plate(&self) -> &str {
        &self.vehicle_plate
    }

    pub fn vehicle_model(&self) -> &str {
        &self.vehicle_model
    }

    pub fn vehicle_capacity_kg(&self) -> Option<f64> {
        self.vehicle_capacity_kg
    }
}

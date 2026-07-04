use domain_commerces::CommerceAddress;
use uuid::Uuid;

use crate::error::ApiError;
use crate::state::AppState;

pub struct AddressResponse {
    pub id: Uuid,
    pub commerce_id: Uuid,
    pub address_type: String,
    pub street: String,
    pub number: String,
    pub district: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub is_primary: bool,
}

impl serde::Serialize for AddressResponse {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("AddressResponse", 12)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("commerceId", &self.commerce_id)?;
        state.serialize_field("addressType", &self.address_type)?;
        state.serialize_field("street", &self.street)?;
        state.serialize_field("number", &self.number)?;
        state.serialize_field("district", &self.district)?;
        state.serialize_field("city", &self.city)?;
        state.serialize_field("state", &self.state)?;
        state.serialize_field("postalCode", &self.postal_code)?;
        state.serialize_field("latitude", &self.latitude)?;
        state.serialize_field("longitude", &self.longitude)?;
        state.serialize_field("isPrimary", &self.is_primary)?;
        state.end()
    }
}

pub async fn ensure_commerce(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
    commerce_id: Uuid,
) -> Result<(), ApiError> {
    let _ = load_commerce(state, tenant_id, commerce_id).await?;
    Ok(())
}

pub async fn load_commerce(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
    commerce_id: Uuid,
) -> Result<domain_commerces::Commerce, ApiError> {
    let row =
        infra_postgres::commerces::find_commerce_by_id(&state.app_pool, tenant_id, commerce_id)
            .await
            .map_err(|_| ApiError::internal())?
            .ok_or_else(ApiError::commerce_not_found)?;
    application::restore_commerce(
        row.id,
        &row.cnpj,
        &row.legal_name,
        &row.trade_name,
        tenant_id,
        row.active,
    )
    .map_err(|_| ApiError::commerce_not_found())
}

pub async fn load_existing_addresses(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
    commerce_id: Uuid,
) -> Result<Vec<CommerceAddress>, ApiError> {
    let commerce = load_commerce(state, tenant_id, commerce_id).await?;
    let rows = infra_postgres::commerces::addresses::list_addresses_by_commerce(
        &state.app_pool,
        tenant_id,
        commerce_id,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    rows.iter()
        .map(|row| {
            application::restore_commerce_address(
                &commerce,
                application::AddressRowInput {
                    id: row.id,
                    address_type: &row.address_type,
                    street: &row.street,
                    number: &row.number,
                    district: row.district.as_deref(),
                    city: &row.city,
                    state: &row.state,
                    postal_code: &row.postal_code,
                    latitude: row.latitude,
                    longitude: row.longitude,
                    is_primary: row.is_primary,
                },
            )
            .map_err(|_| ApiError::internal())
        })
        .collect()
}

pub fn address_response_from_row(
    row: &infra_postgres::commerces::addresses::AddressRow,
) -> AddressResponse {
    AddressResponse {
        id: row.id,
        commerce_id: row.commerce_id,
        address_type: row.address_type.clone(),
        street: row.street.clone(),
        number: row.number.clone(),
        district: row.district.clone(),
        city: row.city.clone(),
        state: row.state.clone(),
        postal_code: row.postal_code.clone(),
        latitude: row.latitude,
        longitude: row.longitude,
        is_primary: row.is_primary,
    }
}

pub fn map_address_error(err: domain_commerces::CommerceError) -> ApiError {
    match err {
        domain_commerces::CommerceError::DuplicatePrimaryAddress => ApiError::bad_request(
            "DUPLICATE_PRIMARY_ADDRESS",
            "A primary address already exists for this type",
        ),
        domain_commerces::CommerceError::InvalidAddressField => {
            ApiError::bad_request("INVALID_ADDRESS", "Invalid address field")
        }
        domain_commerces::CommerceError::InactiveCommerce => ApiError::inactive_commerce(),
        _ => ApiError::bad_request("INVALID_ADDRESS", "Invalid address request"),
    }
}

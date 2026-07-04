use domain_shared::TenantId;

use crate::address_type::AddressType;
use crate::brazilian_state::BrazilianState;
use crate::commerce::Commerce;
use crate::commerce_address_id::CommerceAddressId;
use crate::commerce_id::CommerceId;
use crate::error::CommerceError;
use crate::postal_code::PostalCode;

pub struct CreateCommerceAddressInput {
    pub id: CommerceAddressId,
    pub tenant_id: TenantId,
    pub commerce_id: CommerceId,
    pub address_type: AddressType,
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

/// Normalized billing or delivery address for a Commerce.
#[derive(Debug, Clone)]
pub struct CommerceAddress {
    id: CommerceAddressId,
    tenant_id: TenantId,
    commerce_id: CommerceId,
    address_type: AddressType,
    street: String,
    number: String,
    district: Option<String>,
    city: String,
    state: BrazilianState,
    postal_code: PostalCode,
    latitude: Option<f64>,
    longitude: Option<f64>,
    is_primary: bool,
}

impl CommerceAddress {
    pub fn create(
        commerce: &Commerce,
        input: CreateCommerceAddressInput,
        existing: &[CommerceAddress],
    ) -> Result<Self, CommerceError> {
        ensure_address_allowed_for_commerce(commerce, input.address_type)?;
        if input.commerce_id != commerce.id() {
            return Err(CommerceError::AddressCommerceMismatch);
        }
        if input.tenant_id != commerce.tenant_id() {
            return Err(CommerceError::AddressTenantMismatch);
        }

        let street = input.street.trim();
        let number = input.number.trim();
        let city = input.city.trim();
        if street.is_empty() || number.is_empty() || city.is_empty() {
            return Err(CommerceError::InvalidAddressField);
        }

        let state = BrazilianState::parse(&input.state)?;
        let postal_code = PostalCode::parse(&input.postal_code)?;

        if input.is_primary && has_primary(existing, input.commerce_id, input.address_type) {
            return Err(CommerceError::DuplicatePrimaryAddress);
        }

        Ok(Self {
            id: input.id,
            tenant_id: input.tenant_id,
            commerce_id: input.commerce_id,
            address_type: input.address_type,
            street: street.to_owned(),
            number: number.to_owned(),
            district: input
                .district
                .map(|d| d.trim().to_owned())
                .filter(|d| !d.is_empty()),
            city: city.to_owned(),
            state,
            postal_code,
            latitude: input.latitude,
            longitude: input.longitude,
            is_primary: input.is_primary,
        })
    }

    pub fn id(&self) -> CommerceAddressId {
        self.id
    }

    pub fn tenant_id(&self) -> TenantId {
        self.tenant_id
    }

    pub fn commerce_id(&self) -> CommerceId {
        self.commerce_id
    }

    pub fn address_type(&self) -> AddressType {
        self.address_type
    }

    pub fn street(&self) -> &str {
        &self.street
    }

    pub fn number(&self) -> &str {
        &self.number
    }

    pub fn district(&self) -> Option<&str> {
        self.district.as_deref()
    }

    pub fn city(&self) -> &str {
        &self.city
    }

    pub fn state(&self) -> &BrazilianState {
        &self.state
    }

    pub fn postal_code(&self) -> &PostalCode {
        &self.postal_code
    }

    pub fn latitude(&self) -> Option<f64> {
        self.latitude
    }

    pub fn longitude(&self) -> Option<f64> {
        self.longitude
    }

    pub fn is_primary(&self) -> bool {
        self.is_primary
    }
}

/// BR-CO-004 — inactive commerce cannot add delivery addresses for orders.
pub fn ensure_address_allowed_for_commerce(
    commerce: &Commerce,
    address_type: AddressType,
) -> Result<(), CommerceError> {
    if !commerce.is_active() && address_type == AddressType::Delivery {
        return Err(CommerceError::InactiveCommerceCannotAddDeliveryAddress);
    }
    Ok(())
}

/// BR-CO-005 — order creation requires a delivery address belonging to an active commerce.
pub fn validate_order_delivery_address(
    commerce: &Commerce,
    address: &CommerceAddress,
) -> Result<(), CommerceError> {
    if !commerce.is_active() {
        return Err(CommerceError::InactiveCommerce);
    }
    if address.commerce_id() != commerce.id() {
        return Err(CommerceError::AddressCommerceMismatch);
    }
    if address.address_type() != AddressType::Delivery {
        return Err(CommerceError::InvalidDeliveryAddress);
    }
    Ok(())
}

fn has_primary(
    existing: &[CommerceAddress],
    commerce_id: CommerceId,
    address_type: AddressType,
) -> bool {
    existing.iter().any(|addr| {
        addr.commerce_id() == commerce_id
            && addr.address_type() == address_type
            && addr.is_primary()
    })
}

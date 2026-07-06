//! Commerce registration orchestration helpers.

use domain_commerces::{
    AddressType, Cnpj, Commerce, CommerceAddress, CommerceAddressId, CommerceId,
    CreateCommerceAddressInput, CreateCommerceInput, SubmitCommerceRegistrationInput,
};
use domain_shared::TenantId;
use uuid::Uuid;

pub use crate::{ensure_admin_can_register_commerce, parse_create_commerce, register_commerce};

pub struct AddressRowInput<'a> {
    pub id: Uuid,
    pub address_type: &'a str,
    pub street: &'a str,
    pub number: &'a str,
    pub district: Option<&'a str>,
    pub city: &'a str,
    pub state: &'a str,
    pub postal_code: &'a str,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub is_primary: bool,
}

pub fn restore_commerce(
    id: Uuid,
    cnpj: &str,
    legal_name: &str,
    trade_name: &str,
    tenant_id: TenantId,
    active: bool,
) -> Result<Commerce, domain_commerces::CommerceError> {
    restore_commerce_with_status(
        id,
        cnpj,
        legal_name,
        trade_name,
        tenant_id,
        active,
        domain_commerces::RegistrationStatus::Active,
        None,
        None,
    )
}

#[allow(clippy::too_many_arguments)] // ponytail: single restore helper until second caller needs struct
pub fn restore_commerce_with_status(
    id: Uuid,
    cnpj: &str,
    legal_name: &str,
    trade_name: &str,
    tenant_id: TenantId,
    active: bool,
    registration_status: domain_commerces::RegistrationStatus,
    submitted_by_user_id: Option<Uuid>,
    registration_mode: Option<domain_commerces::RegistrationMode>,
) -> Result<Commerce, domain_commerces::CommerceError> {
    let mut commerce = if registration_status == domain_commerces::RegistrationStatus::PendingReview
    {
        Commerce::submit_registration(SubmitCommerceRegistrationInput {
            id: CommerceId::from_uuid(id),
            cnpj: Cnpj::parse(cnpj)?,
            legal_name: legal_name.to_owned(),
            trade_name: Some(trade_name.to_owned()),
            tenant_id,
            submitted_by_user_id: submitted_by_user_id.unwrap_or_else(Uuid::nil),
            registration_mode: registration_mode
                .unwrap_or(domain_commerces::RegistrationMode::Manual),
        })
    } else {
        Commerce::create(CreateCommerceInput {
            id: CommerceId::from_uuid(id),
            cnpj: Cnpj::parse(cnpj)?,
            legal_name: legal_name.to_owned(),
            trade_name: Some(trade_name.to_owned()),
            tenant_id,
        })
    };
    if !active {
        commerce = commerce.deactivate();
    }
    Ok(commerce)
}

pub fn restore_commerce_address(
    commerce: &Commerce,
    row: AddressRowInput<'_>,
) -> Result<CommerceAddress, domain_commerces::CommerceError> {
    let address_type: AddressType = row.address_type.parse()?;
    CommerceAddress::create(
        commerce,
        CreateCommerceAddressInput {
            id: CommerceAddressId::from_uuid(row.id),
            tenant_id: commerce.tenant_id(),
            commerce_id: commerce.id(),
            address_type,
            street: row.street.to_owned(),
            number: row.number.to_owned(),
            district: row.district.map(str::to_owned),
            city: row.city.to_owned(),
            state: row.state.to_owned(),
            postal_code: row.postal_code.to_owned(),
            latitude: row.latitude,
            longitude: row.longitude,
            is_primary: row.is_primary,
        },
        &[],
    )
}

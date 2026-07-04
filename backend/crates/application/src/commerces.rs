//! Commerce registration orchestration helpers.

use domain_commerces::{Cnpj, Commerce, CommerceId, CreateCommerceInput};
use domain_shared::TenantId;
use uuid::Uuid;

pub use crate::{ensure_admin_can_register_commerce, parse_create_commerce, register_commerce};

pub fn restore_commerce(
    id: Uuid,
    cnpj: &str,
    legal_name: &str,
    trade_name: &str,
    tenant_id: TenantId,
    active: bool,
) -> Result<Commerce, domain_commerces::CommerceError> {
    let mut commerce = Commerce::create(CreateCommerceInput {
        id: CommerceId::from_uuid(id),
        cnpj: Cnpj::parse(cnpj)?,
        legal_name: legal_name.to_owned(),
        trade_name: Some(trade_name.to_owned()),
        tenant_id,
    });
    if !active {
        commerce = commerce.deactivate();
    }
    Ok(commerce)
}

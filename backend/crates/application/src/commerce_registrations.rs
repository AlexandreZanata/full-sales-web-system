//! Commerce registration application orchestration.

use domain_commerces::{
    Cnpj, Commerce, CommerceId, RegistrationMode, SubmitCommerceRegistrationInput,
};
use domain_identity::Role;
use domain_shared::TenantId;
use uuid::Uuid;

use crate::AppError;

pub fn can_review_commerce(role: Role, user_can_review_flag: bool) -> bool {
    role.can_review_commerce_by_role() || user_can_review_flag
}

pub fn ensure_can_submit_commerce(role: Role) -> Result<(), AppError> {
    if role.can_submit_commerce() {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

pub fn ensure_can_review_commerce(role: Role, user_can_review_flag: bool) -> Result<(), AppError> {
    if can_review_commerce(role, user_can_review_flag) {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

pub fn parse_submit_commerce_registration(
    cnpj: &str,
    legal_name: &str,
    trade_name: Option<&str>,
    tenant_id: TenantId,
    submitted_by_user_id: Uuid,
    registration_mode: &str,
) -> Result<Commerce, AppError> {
    Ok(Commerce::submit_registration(
        SubmitCommerceRegistrationInput {
            id: CommerceId::generate(),
            cnpj: Cnpj::parse(cnpj)?,
            legal_name: legal_name.to_owned(),
            trade_name: trade_name.map(str::to_owned),
            tenant_id,
            submitted_by_user_id,
            registration_mode: RegistrationMode::parse(registration_mode)?,
        },
    ))
}

use std::time::Duration;

use domain_commerces::{Cnpj, Commerce, CommerceId, CreateCommerceInput};
use domain_identity::{Email, FullName, IdentityError, RegisterUserInput, Role, User, UserId};
use domain_shared::TenantId;
use thiserror::Error;
use uuid::Uuid;

pub mod audit;
pub mod auth;
pub mod billing;
pub mod categories;
pub mod commerce_registrations;
pub mod commerces;
pub mod deliveries;
pub mod domains;
pub mod feature_flags;
pub mod fraud;
pub mod list_query;
pub mod orders;
pub mod products;
pub mod sales;
pub mod tenants;
pub mod users;

pub use audit::{
    AuditRangeError, MAX_AUDIT_RANGE_DAYS, RecordAuditEvent, validate_audit_date_range,
};
pub use commerce_registrations::{
    can_review_commerce, ensure_can_review_commerce, ensure_can_submit_commerce,
    parse_submit_commerce_registration,
};
pub use commerces::{
    AddressRowInput, restore_commerce, restore_commerce_address, restore_commerce_with_status,
};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("invalid credentials")]
    InvalidCredentials,

    #[error(transparent)]
    Identity(#[from] IdentityError),

    #[error(transparent)]
    Commerce(#[from] domain_commerces::CommerceError),

    #[error("forbidden")]
    Forbidden,

    #[error("tenant suspended")]
    TenantSuspended,

    #[error(transparent)]
    Platform(#[from] domain_platform::PlatformError),

    #[error(transparent)]
    Billing(#[from] domain_billing::BillingError),
}

/// Authenticated principal returned after successful login.
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub tenant_id: TenantId,
    pub role: Role,
    pub commerce_id: Option<Uuid>,
}

pub const ACCESS_TOKEN_TTL: Duration = Duration::from_secs(15 * 60);
pub const REFRESH_TOKEN_TTL: Duration = Duration::from_secs(7 * 24 * 60 * 60);

pub fn register_user(input: RegisterUserInput) -> Result<User, AppError> {
    User::register(input).map_err(AppError::Identity)
}

pub fn register_commerce(input: CreateCommerceInput) -> Commerce {
    Commerce::create(input)
}

pub fn parse_register_user(
    name: &str,
    email: &str,
    role: &str,
    tenant_id: TenantId,
    commerce_id: Option<Uuid>,
) -> Result<User, AppError> {
    Ok(User::register(RegisterUserInput {
        id: UserId::generate(),
        name: FullName::parse(name)?,
        email: Email::parse(email)?,
        role: Role::parse(role)?,
        tenant_id,
        commerce_id,
        profile_file_id: None,
    })?)
}

pub fn parse_create_commerce(
    cnpj: &str,
    legal_name: &str,
    trade_name: Option<&str>,
    tenant_id: TenantId,
) -> Result<Commerce, AppError> {
    Ok(Commerce::create(CreateCommerceInput {
        id: CommerceId::generate(),
        cnpj: Cnpj::parse(cnpj)?,
        legal_name: legal_name.to_owned(),
        trade_name: trade_name.map(str::to_owned),
        tenant_id,
    }))
}

pub fn ensure_admin_can_register_commerce(role: Role) -> Result<(), AppError> {
    if role.can_register_commerce() {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

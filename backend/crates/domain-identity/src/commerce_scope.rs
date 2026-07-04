use uuid::Uuid;

use crate::error::IdentityError;
use crate::role::Role;

/// Validates commerce_id presence rules for user registration.
pub fn validate_commerce_scope(role: Role, commerce_id: Option<Uuid>) -> Result<(), IdentityError> {
    match (role, commerce_id) {
        (Role::CommerceContact, None) => Err(IdentityError::CommerceRequired),
        (Role::CommerceContact, Some(_)) => Ok(()),
        (_, Some(_)) => Err(IdentityError::InvalidCommerceScope),
        (_, None) => Ok(()),
    }
}

/// BR-IA-003 — CommerceContact may only access data for their own commerce.
pub fn ensure_same_commerce(
    viewer_role: Role,
    viewer_commerce_id: Option<Uuid>,
    target_commerce_id: Uuid,
) -> Result<(), IdentityError> {
    if viewer_role != Role::CommerceContact {
        return Ok(());
    }
    match viewer_commerce_id {
        Some(id) if id == target_commerce_id => Ok(()),
        _ => Err(IdentityError::Forbidden),
    }
}

use crate::email::Email;
use crate::error::IdentityError;
use crate::platform_user_id::PlatformUserId;

/// Platform operator — not a tenant `User` (ADR-013).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformUser {
    pub id: PlatformUserId,
    pub email: Email,
    pub active: bool,
    pub mfa_enrolled: bool,
}

impl PlatformUser {
    pub fn ensure_can_authenticate(&self) -> Result<(), IdentityError> {
        if self.active {
            Ok(())
        } else {
            Err(IdentityError::InactiveUser)
        }
    }
}

use domain_shared::TenantId;

use crate::error::IdentityError;
use crate::platform_user_id::PlatformUserId;
use crate::user_id::UserId;

/// Short-lived audited grant for PlatformAdmin to act inside a tenant (ADR-013).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImpersonationGrant {
    pub platform_user_id: PlatformUserId,
    pub target_tenant_id: TenantId,
    pub target_user_id: Option<UserId>,
    pub reason: String,
    pub expires_at_unix: u64,
    pub revoked: bool,
}

impl ImpersonationGrant {
    pub fn new(
        platform_user_id: PlatformUserId,
        target_tenant_id: TenantId,
        target_user_id: Option<UserId>,
        reason: impl Into<String>,
        expires_at_unix: u64,
    ) -> Result<Self, IdentityError> {
        let reason = reason.into();
        if reason.trim().len() < 3 {
            return Err(IdentityError::InvalidImpersonationReason);
        }
        Ok(Self {
            platform_user_id,
            target_tenant_id,
            target_user_id,
            reason,
            expires_at_unix,
            revoked: false,
        })
    }

    pub fn ensure_active(&self, now_unix: u64) -> Result<(), IdentityError> {
        if self.revoked {
            return Err(IdentityError::ImpersonationRevoked);
        }
        if now_unix >= self.expires_at_unix {
            return Err(IdentityError::ImpersonationExpired);
        }
        Ok(())
    }
}

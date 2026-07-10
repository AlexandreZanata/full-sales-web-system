//! PlatformAdmin capability checks — separate from tenant `Role` (ADR-013).

/// Static permission helpers for `PlatformAdmin` actors.
pub struct PlatformAdminPermissions;

impl PlatformAdminPermissions {
    pub fn can_manage_tenants() -> bool {
        true
    }

    pub fn can_impersonate() -> bool {
        true
    }

    pub fn can_view_cross_tenant() -> bool {
        true
    }
}

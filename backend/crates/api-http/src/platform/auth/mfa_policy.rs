//! ponytail: dev bypass — set PLATFORM_MFA_DISABLED=1 to skip TOTP until MFA UX is finalized.

pub fn platform_mfa_enabled() -> bool {
    !matches!(
        std::env::var("PLATFORM_MFA_DISABLED")
            .ok()
            .as_deref()
            .map(str::trim),
        Some("1" | "true" | "TRUE" | "yes" | "YES")
    )
}

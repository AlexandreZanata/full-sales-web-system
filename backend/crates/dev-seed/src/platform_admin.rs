use infra_postgres::PgPool;
use infra_postgres::identity::{
    InsertPlatformUserParams, find_platform_user_for_login, insert_platform_user,
};

use crate::error::{DevSeedError, DevSeedResult};
use crate::foundation::hash_password;
use crate::ids::{DEV_PASSWORD, platform_admin_id};

pub const PLATFORM_ADMIN_EMAIL: &str = "platform@test.com";
/// Base32 secret (≥16 decoded bytes) for deterministic dev/CI TOTP when MFA is enabled.
pub const DEV_MFA_SECRET: &str = "KVKFKRCPNZQUYMLXOVYDSQKJKZDTSRLD";

fn platform_mfa_disabled() -> bool {
    matches!(
        std::env::var("PLATFORM_MFA_DISABLED")
            .ok()
            .as_deref()
            .map(str::trim),
        Some("1" | "true" | "TRUE" | "yes" | "YES")
    )
}

pub async fn seed_platform_admin(admin_pool: &PgPool) -> DevSeedResult<()> {
    ensure_platform_admin(admin_pool).await
}

/// Idempotent — creates platform admin if missing; aligns MFA with env on dev.
pub async fn ensure_platform_admin(admin_pool: &PgPool) -> DevSeedResult<()> {
    if find_platform_user_for_login(admin_pool, PLATFORM_ADMIN_EMAIL)
        .await?
        .is_some()
    {
        if platform_mfa_disabled() {
            sqlx::query(
                "UPDATE identity.platform_users
                 SET mfa_enrolled = false, mfa_secret = NULL, updated_at = now()
                 WHERE email = $1",
            )
            .bind(PLATFORM_ADMIN_EMAIL)
            .execute(admin_pool)
            .await
            .map_err(|e| DevSeedError::Aborted(e.to_string()))?;
        }
        return Ok(());
    }

    let password_hash = hash_password(DEV_PASSWORD)?;
    let mfa_disabled = platform_mfa_disabled();
    insert_platform_user(
        admin_pool,
        InsertPlatformUserParams {
            id: platform_admin_id(),
            email: PLATFORM_ADMIN_EMAIL,
            name: "Platform Admin",
            password_hash: &password_hash,
            mfa_secret: if mfa_disabled {
                None
            } else {
                Some(DEV_MFA_SECRET)
            },
            mfa_enrolled: !mfa_disabled,
        },
    )
    .await?;
    Ok(())
}

pub fn dev_mfa_code() -> DevSeedResult<String> {
    let verifier = infra_crypto::TotpVerifier::from_base32_secret(DEV_MFA_SECRET)
        .map_err(|e| DevSeedError::Aborted(e.to_string()))?;
    Ok(verifier.current_code())
}

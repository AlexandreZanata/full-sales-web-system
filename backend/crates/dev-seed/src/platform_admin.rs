use infra_crypto::TotpVerifier;
use infra_postgres::PgPool;
use infra_postgres::identity::{
    InsertPlatformUserParams, find_platform_user_for_login, insert_platform_user,
};

use crate::error::{DevSeedError, DevSeedResult};
use crate::foundation::hash_password;
use crate::ids::{DEV_PASSWORD, platform_admin_id};

pub const PLATFORM_ADMIN_EMAIL: &str = "platform@test.com";
/// Base32 secret (≥16 decoded bytes) for deterministic dev/CI TOTP.
pub const DEV_MFA_SECRET: &str = "KVKFKRCPNZQUYMLXOVYDSQKJKZDTSRLD";

pub async fn seed_platform_admin(admin_pool: &PgPool) -> DevSeedResult<()> {
    if find_platform_user_for_login(admin_pool, PLATFORM_ADMIN_EMAIL)
        .await?
        .is_some()
    {
        return Ok(());
    }

    let password_hash = hash_password(DEV_PASSWORD)?;
    insert_platform_user(
        admin_pool,
        InsertPlatformUserParams {
            id: platform_admin_id(),
            email: PLATFORM_ADMIN_EMAIL,
            name: "Platform Admin",
            password_hash: &password_hash,
            mfa_secret: Some(DEV_MFA_SECRET),
            mfa_enrolled: true,
        },
    )
    .await?;
    Ok(())
}

pub fn dev_mfa_code() -> DevSeedResult<String> {
    let verifier = TotpVerifier::from_base32_secret(DEV_MFA_SECRET)
        .map_err(|e| DevSeedError::Aborted(e.to_string()))?;
    Ok(verifier.current_code())
}

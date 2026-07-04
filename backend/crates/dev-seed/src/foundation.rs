use domain_shared::TenantId;
use ed25519_dalek::SigningKey;
use infra_crypto::PasswordHasher;
use infra_postgres::PgPool;
use infra_postgres::PostgresError;
use infra_postgres::identity;
use infra_postgres::reports;
use infra_postgres::shared;

use crate::error::{DevSeedError, DevSeedResult};
use crate::ids::{DEV_PASSWORD, DEV_SIGNING_KEY_ID, DEV_TENANT_NAME, admin_user_id, tenant_id};

pub struct FoundationSeed {
    pub tenant_id: TenantId,
    pub signing_key: SigningKey,
}

pub async fn seed_foundation(
    admin_pool: &PgPool,
    app_pool: &PgPool,
) -> DevSeedResult<FoundationSeed> {
    let tenant = tenant_id();
    if shared::find_tenant_by_id(admin_pool, tenant)
        .await?
        .is_none()
    {
        shared::insert_tenant(admin_pool, tenant, DEV_TENANT_NAME).await?;
    }
    remove_conflicting_dev_emails(admin_pool, tenant).await?;

    let signing_key = signing_key_from_env();
    let public_key = signing_key.verifying_key().to_bytes();
    if reports::find_signing_key_by_public_key_id(admin_pool, DEV_SIGNING_KEY_ID)
        .await?
        .is_none()
    {
        reports::insert_signing_key(app_pool, tenant, DEV_SIGNING_KEY_ID, &public_key).await?;
    }

    Ok(FoundationSeed {
        tenant_id: tenant,
        signing_key,
    })
}

pub fn signing_key_from_env() -> SigningKey {
    if let Ok(hex_key) = std::env::var("REPORT_SIGNING_KEY_HEX") {
        if let Ok(bytes) = hex::decode(hex_key.trim()) {
            if let Ok(array) = <[u8; 32]>::try_from(bytes) {
                return SigningKey::from_bytes(&array);
            }
        }
    }
    SigningKey::from_bytes(&[7u8; 32])
}

pub fn hash_password(password: &str) -> DevSeedResult<String> {
    PasswordHasher::hash(password).map_err(|err| DevSeedError::Aborted(err.to_string()))
}

pub async fn is_already_seeded(admin_pool: &PgPool, app_pool: &PgPool) -> DevSeedResult<bool> {
    let tenant = tenant_id();
    let Some(record) = identity::find_user_for_login(admin_pool, "admin@test.com").await? else {
        return Ok(false);
    };
    if record.tenant_id != tenant.as_uuid() {
        return Ok(false);
    }
    let session = infra_postgres::SessionContext {
        tenant_id: tenant,
        role: "Admin".into(),
        user_id: admin_user_id(),
        commerce_id: None,
    };
    let order_count = infra_postgres::orders::count_orders(
        app_pool,
        &session,
        &infra_postgres::orders::OrderListFilters {
            status: None,
            commerce_id: None,
            from: None,
            to: None,
        },
    )
    .await?;
    Ok(order_count >= 6)
}

async fn remove_conflicting_dev_emails(admin_pool: &PgPool, tenant: TenantId) -> DevSeedResult<()> {
    const EMAILS: &[&str] = &[
        "admin@test.com",
        "driver-a@test.com",
        "driver-b@test.com",
        "seller@test.com",
        "portal@seed-store.com",
        "inactive-driver@test.com",
    ];
    for email in EMAILS {
        sqlx::query("DELETE FROM identity.users WHERE email = $1 AND tenant_id != $2")
            .bind(email)
            .bind(tenant.as_uuid())
            .execute(admin_pool)
            .await
            .map_err(PostgresError::from)?;
    }
    Ok(())
}

pub async fn ensure_user(
    app_pool: &PgPool,
    admin_pool: &PgPool,
    tenant: TenantId,
    id: uuid::Uuid,
    email: &str,
    name: &str,
    role: &str,
    active: bool,
    commerce_id: Option<uuid::Uuid>,
    profile_file_id: Option<uuid::Uuid>,
) -> DevSeedResult<()> {
    if identity::find_user_for_login(admin_pool, email)
        .await?
        .is_some()
    {
        return Ok(());
    }
    let hash = hash_password(DEV_PASSWORD)?;
    identity::insert_user(
        app_pool,
        tenant,
        identity::InsertUserParams {
            id,
            email,
            name,
            role,
            password_hash: &hash,
            commerce_id,
            profile_file_id,
        },
    )
    .await?;
    if !active {
        identity::deactivate_user(admin_pool, id).await?;
    }
    Ok(())
}

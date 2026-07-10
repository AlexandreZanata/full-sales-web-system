use chrono::{DateTime, Utc};
use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_bypass_rls;

pub struct InsertPlatformUserParams<'a> {
    pub id: Uuid,
    pub email: &'a str,
    pub name: &'a str,
    pub password_hash: &'a str,
    pub mfa_secret: Option<&'a str>,
    pub mfa_enrolled: bool,
}

#[derive(Debug, Clone)]
pub struct PlatformLoginRecord {
    pub id: Uuid,
    pub password_hash: String,
    pub active: bool,
    pub mfa_secret: Option<String>,
    pub mfa_enrolled: bool,
}

pub async fn insert_platform_user(
    pool: &PgPool,
    params: InsertPlatformUserParams<'_>,
) -> Result<(), PostgresError> {
    sqlx::query(
        "INSERT INTO identity.platform_users
         (id, email, name, password_hash, mfa_secret, mfa_enrolled, active)
         VALUES ($1, $2, $3, $4, $5, $6, true)",
    )
    .bind(params.id)
    .bind(params.email)
    .bind(params.name)
    .bind(params.password_hash)
    .bind(params.mfa_secret)
    .bind(params.mfa_enrolled)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find_platform_user_for_login(
    pool: &PgPool,
    email: &str,
) -> Result<Option<PlatformLoginRecord>, PostgresError> {
    let row = sqlx::query_as::<_, PlatformLoginRow>(
        "SELECT id, password_hash, active, mfa_secret, mfa_enrolled
         FROM identity.platform_users WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(PlatformLoginRecord::from))
}

pub async fn find_platform_user_by_id(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<PlatformLoginRecord>, PostgresError> {
    let row = sqlx::query_as::<_, PlatformLoginRow>(
        "SELECT id, password_hash, active, mfa_secret, mfa_enrolled
         FROM identity.platform_users WHERE id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(PlatformLoginRecord::from))
}

pub struct ImpersonationGrantInsert<'a> {
    pub id: Uuid,
    pub platform_user_id: Uuid,
    pub target_tenant_id: Uuid,
    pub target_user_id: Option<Uuid>,
    pub reason: &'a str,
    pub expires_at: DateTime<Utc>,
}

pub async fn insert_impersonation_grant(
    pool: &PgPool,
    grant: ImpersonationGrantInsert<'_>,
) -> Result<(), PostgresError> {
    sqlx::query(
        "INSERT INTO identity.impersonation_grants
         (id, platform_user_id, target_tenant_id, target_user_id, reason, expires_at)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(grant.id)
    .bind(grant.platform_user_id)
    .bind(grant.target_tenant_id)
    .bind(grant.target_user_id)
    .bind(grant.reason)
    .bind(grant.expires_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn revoke_impersonation_grant(
    pool: &PgPool,
    grant_id: Uuid,
) -> Result<bool, PostgresError> {
    let result = sqlx::query(
        "UPDATE identity.impersonation_grants
         SET revoked_at = now()
         WHERE id = $1 AND revoked_at IS NULL",
    )
    .bind(grant_id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn find_tenant_admin_user(
    pool: &PgPool,
    tenant_id: TenantId,
    user_id: Option<Uuid>,
) -> Result<Option<Uuid>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    let id = if let Some(uid) = user_id {
        sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM identity.users
             WHERE id = $1 AND tenant_id = $2 AND role = 'Admin' AND active = true",
        )
        .bind(uid)
        .bind(tenant_id.as_uuid())
        .fetch_optional(&mut *tx)
        .await?
    } else {
        sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM identity.users
             WHERE tenant_id = $1 AND role = 'Admin' AND active = true
             ORDER BY created_at ASC LIMIT 1",
        )
        .bind(tenant_id.as_uuid())
        .fetch_optional(&mut *tx)
        .await?
    };
    tx.commit().await?;
    Ok(id)
}

pub async fn tenant_exists(pool: &PgPool, tenant_id: Uuid) -> Result<bool, PostgresError> {
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM shared.tenants WHERE id = $1)",
    )
    .bind(tenant_id)
    .fetch_one(pool)
    .await?;
    Ok(exists)
}

#[derive(sqlx::FromRow)]
struct PlatformLoginRow {
    id: Uuid,
    password_hash: String,
    active: bool,
    mfa_secret: Option<String>,
    mfa_enrolled: bool,
}

impl From<PlatformLoginRow> for PlatformLoginRecord {
    fn from(row: PlatformLoginRow) -> Self {
        Self {
            id: row.id,
            password_hash: row.password_hash,
            active: row.active,
            mfa_secret: row.mfa_secret,
            mfa_enrolled: row.mfa_enrolled,
        }
    }
}

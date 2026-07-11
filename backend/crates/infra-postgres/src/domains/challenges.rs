use chrono::{DateTime, Utc};
use domain_shared::TenantId;
use sqlx::PgPool;
use sqlx::Row;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

pub struct ChallengeRow {
    pub id: Uuid,
    pub domain_id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

pub async fn insert_challenge(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
    domain_id: Uuid,
    token: &str,
    expires_at: DateTime<Utc>,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO domains.domain_verification_challenges (id, domain_id, token, expires_at)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(id)
    .bind(domain_id)
    .bind(token)
    .bind(expires_at)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn find_active_challenge(
    pool: &PgPool,
    domain_id: Uuid,
) -> Result<Option<ChallengeRow>, PostgresError> {
    let row = sqlx::query_as::<_, ChallengeRow>(
        "SELECT id, domain_id, token, expires_at, created_at
         FROM domains.domain_verification_challenges
         WHERE domain_id = $1 AND expires_at > now()
         ORDER BY created_at DESC
         LIMIT 1",
    )
    .bind(domain_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn expire_challenges(pool: &PgPool, domain_id: Uuid) -> Result<(), PostgresError> {
    sqlx::query("DELETE FROM domains.domain_verification_challenges WHERE domain_id = $1")
        .bind(domain_id)
        .execute(pool)
        .await?;
    Ok(())
}

impl sqlx::FromRow<'_, sqlx::postgres::PgRow> for ChallengeRow {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            domain_id: row.try_get("domain_id")?,
            token: row.try_get("token")?,
            expires_at: row.try_get("expires_at")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

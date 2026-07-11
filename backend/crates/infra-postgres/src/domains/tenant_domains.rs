use chrono::{DateTime, Utc};
use domain_shared::TenantId;
use sqlx::PgPool;
use sqlx::Row;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::{apply_bypass_rls, apply_tenant_context};

pub struct DomainRow {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub hostname: String,
    pub status: String,
    pub verification_token: String,
    pub verified_at: Option<DateTime<Utc>>,
    pub is_primary: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct NewDomainRow {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub hostname: String,
    pub status: String,
    pub verification_token: String,
}

pub async fn insert_tenant_domain(pool: &PgPool, row: NewDomainRow) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, row.tenant_id).await?;
    sqlx::query(
        "INSERT INTO domains.tenant_domains
         (id, tenant_id, hostname, status, verification_token)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(row.id)
    .bind(row.tenant_id.as_uuid())
    .bind(row.hostname)
    .bind(row.status)
    .bind(row.verification_token)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn update_tenant_domain(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
    status: &str,
    verified_at: Option<DateTime<Utc>>,
    is_primary: bool,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "UPDATE domains.tenant_domains
         SET status = $2, verified_at = $3, is_primary = $4, updated_at = now()
         WHERE id = $1",
    )
    .bind(id)
    .bind(status)
    .bind(verified_at)
    .bind(is_primary)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn update_tenant_domain_admin(
    pool: &PgPool,
    id: Uuid,
    status: &str,
    verified_at: Option<DateTime<Utc>>,
    is_primary: bool,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    sqlx::query(
        "UPDATE domains.tenant_domains
         SET status = $2, verified_at = $3, is_primary = $4, updated_at = now()
         WHERE id = $1",
    )
    .bind(id)
    .bind(status)
    .bind(verified_at)
    .bind(is_primary)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn find_domain_by_id(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
) -> Result<Option<DomainRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = fetch_optional(&mut tx, "WHERE id = $1", id).await?;
    tx.commit().await?;
    Ok(row)
}

pub async fn find_domain_by_id_admin(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<DomainRow>, PostgresError> {
    let row = sqlx::query_as::<_, DomainRow>(
        "SELECT id, tenant_id, hostname, status, verification_token, verified_at,
                is_primary, created_at, updated_at
         FROM domains.tenant_domains WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn find_domain_by_hostname(
    pool: &PgPool,
    hostname: &str,
) -> Result<Option<DomainRow>, PostgresError> {
    let row = sqlx::query_as::<_, DomainRow>(
        "SELECT id, tenant_id, hostname, status, verification_token, verified_at,
                is_primary, created_at, updated_at
         FROM domains.tenant_domains WHERE lower(hostname) = lower($1)",
    )
    .bind(hostname)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn find_tenant_by_active_hostname(
    pool: &PgPool,
    hostname: &str,
) -> Result<Option<TenantId>, PostgresError> {
    let tenant_id = sqlx::query_scalar::<_, Option<Uuid>>(
        "SELECT tenant_id FROM domains.tenant_domains
         WHERE lower(hostname) = lower($1) AND status = 'Active'",
    )
    .bind(hostname)
    .fetch_optional(pool)
    .await?
    .flatten()
    .map(TenantId::from_uuid);
    Ok(tenant_id)
}

pub async fn list_domains_tenant(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Vec<DomainRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, DomainRow>(
        "SELECT id, tenant_id, hostname, status, verification_token, verified_at,
                is_primary, created_at, updated_at
         FROM domains.tenant_domains
         WHERE tenant_id = $1
         ORDER BY created_at DESC",
    )
    .bind(tenant_id.as_uuid())
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows)
}

pub async fn list_domains_platform(
    pool: &PgPool,
    status: Option<&str>,
    tenant_id: Option<Uuid>,
    before_id: Option<Uuid>,
    limit: i64,
) -> Result<Vec<DomainRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    let rows = sqlx::query_as::<_, DomainRow>(
        "SELECT id, tenant_id, hostname, status, verification_token, verified_at,
                is_primary, created_at, updated_at
         FROM domains.tenant_domains
         WHERE ($1::uuid IS NULL OR id < $1)
           AND ($2::text IS NULL OR status = $2)
           AND ($3::uuid IS NULL OR tenant_id = $3)
         ORDER BY id DESC
         LIMIT $4",
    )
    .bind(before_id)
    .bind(status)
    .bind(tenant_id)
    .bind(limit)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows)
}

pub async fn list_verifying_domains(pool: &PgPool) -> Result<Vec<DomainRow>, PostgresError> {
    let rows = sqlx::query_as::<_, DomainRow>(
        "SELECT id, tenant_id, hostname, status, verification_token, verified_at,
                is_primary, created_at, updated_at
         FROM domains.tenant_domains
         WHERE status = 'Verifying'
         ORDER BY updated_at ASC",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn clear_primary_for_tenant(
    pool: &PgPool,
    tenant_id: TenantId,
    except_id: Option<Uuid>,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "UPDATE domains.tenant_domains
         SET is_primary = false, updated_at = now()
         WHERE tenant_id = $1 AND is_primary = true
           AND ($2::uuid IS NULL OR id <> $2)",
    )
    .bind(tenant_id.as_uuid())
    .bind(except_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn clear_primary_for_tenant_admin(
    pool: &PgPool,
    tenant_id: TenantId,
    except_id: Option<Uuid>,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    sqlx::query(
        "UPDATE domains.tenant_domains
         SET is_primary = false, updated_at = now()
         WHERE tenant_id = $1 AND is_primary = true
           AND ($2::uuid IS NULL OR id <> $2)",
    )
    .bind(tenant_id.as_uuid())
    .bind(except_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn delete_domain(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let result = sqlx::query("DELETE FROM domains.tenant_domains WHERE id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(result.rows_affected() > 0)
}

async fn fetch_optional(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    clause: &str,
    id: Uuid,
) -> Result<Option<DomainRow>, PostgresError> {
    let query = format!(
        "SELECT id, tenant_id, hostname, status, verification_token, verified_at,
                is_primary, created_at, updated_at
         FROM domains.tenant_domains {clause}"
    );
    let row = sqlx::query_as::<_, DomainRow>(&query)
        .bind(id)
        .fetch_optional(&mut **tx)
        .await?;
    Ok(row)
}

impl sqlx::FromRow<'_, sqlx::postgres::PgRow> for DomainRow {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            tenant_id: TenantId::from_uuid(row.try_get("tenant_id")?),
            hostname: row.try_get("hostname")?,
            status: row.try_get("status")?,
            verification_token: row.try_get("verification_token")?,
            verified_at: row.try_get("verified_at")?,
            is_primary: row.try_get("is_primary")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

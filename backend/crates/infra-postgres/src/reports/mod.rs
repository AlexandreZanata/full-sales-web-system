use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

pub struct NewReport<'a> {
    pub id: Uuid,
    pub report_type: &'a str,
    pub period_start: chrono::DateTime<chrono::Utc>,
    pub period_end: chrono::DateTime<chrono::Utc>,
    pub canonical_payload: &'a str,
    pub signature: &'a [u8],
    pub public_key_id: &'a str,
}

pub async fn insert_signing_key(
    pool: &PgPool,
    tenant_id: TenantId,
    public_key_id: &str,
    public_key: &[u8],
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO reports.signing_keys (public_key_id, tenant_id, public_key, valid_from)
         VALUES ($1, $2, $3, now())",
    )
    .bind(public_key_id)
    .bind(tenant_id.as_uuid())
    .bind(public_key)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn insert_report(
    pool: &PgPool,
    tenant_id: TenantId,
    report: NewReport<'_>,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO reports.reports
         (id, tenant_id, report_type, period_start, period_end, canonical_payload, signature, public_key_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(report.id)
    .bind(tenant_id.as_uuid())
    .bind(report.report_type)
    .bind(report.period_start)
    .bind(report.period_end)
    .bind(report.canonical_payload)
    .bind(report.signature)
    .bind(report.public_key_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn list_report_ids(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Vec<Uuid>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows =
        sqlx::query_scalar::<_, Uuid>("SELECT id FROM reports.reports ORDER BY generated_at")
            .fetch_all(&mut *tx)
            .await?;
    tx.commit().await?;
    Ok(rows)
}

pub struct SigningKeyRow {
    pub public_key_id: String,
    pub public_key: Vec<u8>,
}

pub async fn find_active_signing_key(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Option<SigningKeyRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, (String, Vec<u8>)>(
        "SELECT public_key_id, public_key
         FROM reports.signing_keys
         WHERE active = true
         ORDER BY valid_from DESC
         LIMIT 1",
    )
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(|(public_key_id, public_key)| SigningKeyRow {
        public_key_id,
        public_key,
    }))
}

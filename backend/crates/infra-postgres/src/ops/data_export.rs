use chrono::{DateTime, Utc};
use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_bypass_rls;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DataExportJobRow {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub requested_by: Uuid,
    pub actor_type: String,
    pub status: String,
    pub storage_bucket: Option<String>,
    pub storage_key: Option<String>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct NewDataExportJob {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub requested_by: Uuid,
    pub actor_type: &'static str,
}

pub async fn insert_export_job(pool: &PgPool, job: NewDataExportJob) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    sqlx::query(
        "INSERT INTO ops.data_export_jobs (id, tenant_id, requested_by, actor_type, status)
         VALUES ($1, $2, $3, $4, 'pending')",
    )
    .bind(job.id)
    .bind(job.tenant_id.as_uuid())
    .bind(job.requested_by)
    .bind(job.actor_type)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn find_export_job(
    pool: &PgPool,
    tenant_id: TenantId,
    job_id: Uuid,
) -> Result<Option<DataExportJobRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    let row = sqlx::query_as::<_, DataExportJobRow>(
        "SELECT id, tenant_id, requested_by, actor_type, status, storage_bucket, storage_key,
                error_message, created_at, completed_at
         FROM ops.data_export_jobs
         WHERE id = $1 AND tenant_id = $2",
    )
    .bind(job_id)
    .bind(tenant_id.as_uuid())
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row)
}

pub async fn mark_export_processing(pool: &PgPool, job_id: Uuid) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    sqlx::query(
        "UPDATE ops.data_export_jobs SET status = 'processing' WHERE id = $1 AND status = 'pending'",
    )
    .bind(job_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn mark_export_completed(
    pool: &PgPool,
    job_id: Uuid,
    bucket: &str,
    key: &str,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    sqlx::query(
        "UPDATE ops.data_export_jobs
         SET status = 'completed', storage_bucket = $2, storage_key = $3, completed_at = now()
         WHERE id = $1",
    )
    .bind(job_id)
    .bind(bucket)
    .bind(key)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn mark_export_failed(
    pool: &PgPool,
    job_id: Uuid,
    message: &str,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    sqlx::query(
        "UPDATE ops.data_export_jobs
         SET status = 'failed', error_message = $2, completed_at = now()
         WHERE id = $1",
    )
    .bind(job_id)
    .bind(message)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn tenant_has_legal_hold(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<bool, PostgresError> {
    let held = sqlx::query_scalar::<_, bool>(
        "SELECT COALESCE((settings->>'legalHold')::boolean, false)
         FROM shared.tenants WHERE id = $1",
    )
    .bind(tenant_id.as_uuid())
    .fetch_optional(pool)
    .await?
    .unwrap_or(false);
    Ok(held)
}

pub async fn fetch_export_users(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Vec<serde_json::Value>, PostgresError> {
    let rows = crate::identity::list_users(pool, tenant_id, 10_000, 0).await?;
    Ok(rows
        .into_iter()
        .map(|row| {
            serde_json::json!({
                "id": row.id,
                "email": row.email,
                "name": row.name,
                "role": row.role,
                "active": row.active,
            })
        })
        .collect())
}

pub async fn fetch_export_commerces(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Vec<serde_json::Value>, PostgresError> {
    let rows = crate::commerces::list_commerces(pool, tenant_id, None, 10_000, 0).await?;
    Ok(rows
        .into_iter()
        .map(|row| {
            serde_json::json!({
                "id": row.id,
                "cnpj": row.cnpj,
                "legalName": row.legal_name,
                "tradeName": row.trade_name,
                "active": row.active,
            })
        })
        .collect())
}

pub async fn fetch_export_sales(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Vec<serde_json::Value>, PostgresError> {
    let filters = crate::sales::SaleFilters {
        commerce_id: None,
        driver_id: None,
        status: None,
        from: None,
        to: None,
    };
    let rows = crate::sales::list_sales(pool, tenant_id, &filters, 10_000, 0).await?;
    Ok(rows
        .into_iter()
        .map(|row| {
            serde_json::json!({
                "id": row.id,
                "commerceId": row.commerce_id,
                "driverId": row.driver_id,
                "status": row.status,
                "totalAmount": row.total_amount,
                "totalCurrency": row.total_currency,
                "createdAt": row.created_at,
            })
        })
        .collect())
}

pub async fn fetch_export_orders(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Vec<serde_json::Value>, PostgresError> {
    let session = crate::rls::SessionContext {
        tenant_id,
        user_id: Uuid::nil(),
        role: "Admin".into(),
        commerce_id: None,
    };
    let filters = crate::orders::OrderListFilters {
        status: None,
        commerce_id: None,
        from: None,
        to: None,
    };
    let rows = crate::orders::list_orders(pool, &session, &filters, 10_000, 0).await?;
    Ok(rows
        .into_iter()
        .map(|row| {
            serde_json::json!({
                "id": row.id,
                "commerceId": row.commerce_id,
                "status": row.status,
                "createdAt": row.created_at,
            })
        })
        .collect())
}

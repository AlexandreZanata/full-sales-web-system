use chrono::{DateTime, Utc};
use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;

#[derive(Debug, Clone)]
pub struct PaymentEventInsert {
    pub asaas_event_id: String,
    pub event_type: String,
    pub tenant_id: Option<TenantId>,
    pub payload: serde_json::Value,
}

pub async fn insert_payment_event(
    pool: &PgPool,
    event: PaymentEventInsert,
) -> Result<bool, PostgresError> {
    let row = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO billing.payment_events
            (asaas_event_id, event_type, tenant_id, payload, status)
         VALUES ($1, $2, $3, $4, 'received')
         ON CONFLICT (asaas_event_id) DO NOTHING
         RETURNING id",
    )
    .bind(&event.asaas_event_id)
    .bind(&event.event_type)
    .bind(event.tenant_id.map(|t| t.as_uuid()))
    .bind(&event.payload)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}

pub async fn mark_payment_event_processed(
    pool: &PgPool,
    asaas_event_id: &str,
) -> Result<(), PostgresError> {
    sqlx::query(
        "UPDATE billing.payment_events
         SET status = 'processed', processed_at = now()
         WHERE asaas_event_id = $1",
    )
    .bind(asaas_event_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_tenant_asaas_customer(
    pool: &PgPool,
    tenant_id: TenantId,
    asaas_customer_id: &str,
) -> Result<bool, PostgresError> {
    let result = sqlx::query(
        "UPDATE shared.tenants SET asaas_customer_id = $2, updated_at = now() WHERE id = $1",
    )
    .bind(tenant_id.as_uuid())
    .bind(asaas_customer_id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn insert_provisioning_dead_letter(
    pool: &PgPool,
    tenant_id: TenantId,
    error_code: &str,
    error_message: &str,
    payload: serde_json::Value,
) -> Result<(), PostgresError> {
    sqlx::query(
        "INSERT INTO billing.provisioning_dead_letters
            (tenant_id, error_code, error_message, payload)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(tenant_id.as_uuid())
    .bind(error_code)
    .bind(error_message)
    .bind(payload)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find_asaas_customer_id(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Option<String>, PostgresError> {
    let id = sqlx::query_scalar::<_, Option<String>>(
        "SELECT asaas_customer_id FROM shared.tenants WHERE id = $1",
    )
    .bind(tenant_id.as_uuid())
    .fetch_optional(pool)
    .await?
    .flatten();
    Ok(id)
}

pub async fn count_provisioning_dead_letters(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<i64, PostgresError> {
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM billing.provisioning_dead_letters WHERE tenant_id = $1",
    )
    .bind(tenant_id.as_uuid())
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn count_payment_events(pool: &PgPool) -> Result<i64, PostgresError> {
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM billing.payment_events")
        .fetch_one(pool)
        .await?;
    Ok(count)
}

pub async fn latest_payment_event_at(
    pool: &PgPool,
) -> Result<Option<DateTime<Utc>>, PostgresError> {
    let ts = sqlx::query_scalar::<_, Option<DateTime<Utc>>>(
        "SELECT created_at FROM billing.payment_events ORDER BY created_at DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?
    .flatten();
    Ok(ts)
}

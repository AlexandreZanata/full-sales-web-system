use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;

pub async fn find_duplicate_card_tenant(
    pool: &PgPool,
    card_fingerprint: &str,
    exclude_tenant_id: TenantId,
) -> Result<Option<TenantId>, PostgresError> {
    let tenant_id = sqlx::query_scalar::<_, Option<Uuid>>(
        "SELECT tenant_id FROM fraud.fraud_events
         WHERE metadata->>'cardFingerprint' = $1
           AND tenant_id IS NOT NULL
           AND tenant_id != $2
         LIMIT 1",
    )
    .bind(card_fingerprint)
    .bind(exclude_tenant_id.as_uuid())
    .fetch_optional(pool)
    .await?
    .flatten()
    .map(TenantId::from_uuid);
    Ok(tenant_id)
}

pub async fn average_order_amount_minor(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Option<f64>, PostgresError> {
    let avg = sqlx::query_scalar::<_, Option<f64>>(
        "SELECT AVG(total_amount)::float8 FROM orders.orders
         WHERE tenant_id = $1 AND status NOT IN ('Draft', 'Cancelled')",
    )
    .bind(tenant_id.as_uuid())
    .fetch_one(pool)
    .await?;
    Ok(avg)
}

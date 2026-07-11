use domain_shared::TenantId;
use sqlx::PgPool;

use crate::PostgresError;
use crate::rls::{apply_bypass_rls, apply_tenant_context};

#[derive(Debug, Clone, Default)]
pub struct TenantWorkforceStats {
    pub users: i64,
    pub drivers: i64,
    pub sellers: i64,
    pub commerces: i64,
    pub orders: i64,
    pub mrr_minor: i64,
    pub mrr_currency: String,
}

pub async fn tenant_workforce_stats(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<TenantWorkforceStats, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    let users = count_role(&mut tx, tenant_id, None).await?;
    let drivers = count_role(&mut tx, tenant_id, Some("Driver")).await?;
    let sellers = count_role(&mut tx, tenant_id, Some("Seller")).await?;
    let commerces = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM commerces.commerces WHERE tenant_id = $1",
    )
    .bind(tenant_id.as_uuid())
    .fetch_one(&mut *tx)
    .await?;
    let orders =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM orders.orders WHERE tenant_id = $1")
            .bind(tenant_id.as_uuid())
            .fetch_one(&mut *tx)
            .await?;
    let mrr = sqlx::query_as::<_, MrrRecord>(
        "SELECT COALESCE(p.price_minor, 0) AS price_minor, COALESCE(p.price_currency, 'BRL') AS price_currency
         FROM billing.subscriptions s
         JOIN billing.plans p ON p.id = s.plan_id
         WHERE s.tenant_id = $1 AND s.status IN ('Active', 'PastDue')
         ORDER BY s.created_at DESC
         LIMIT 1",
    )
    .bind(tenant_id.as_uuid())
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(TenantWorkforceStats {
        users,
        drivers,
        sellers,
        commerces,
        orders,
        mrr_minor: mrr.as_ref().map(|m| m.price_minor).unwrap_or(0),
        mrr_currency: mrr
            .map(|m| m.price_currency)
            .unwrap_or_else(|| "BRL".into()),
    })
}

async fn count_role(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    tenant_id: TenantId,
    role: Option<&str>,
) -> Result<i64, PostgresError> {
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM identity.users
         WHERE tenant_id = $1 AND ($2::text IS NULL OR role = $2)",
    )
    .bind(tenant_id.as_uuid())
    .bind(role)
    .fetch_one(&mut **tx)
    .await?;
    Ok(count)
}

pub async fn update_tenant_feature_flags(
    pool: &PgPool,
    tenant_id: TenantId,
    flags: serde_json::Value,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    let result = sqlx::query(
        "UPDATE shared.tenants
         SET settings = jsonb_set(COALESCE(settings, '{}'::jsonb), '{feature_flags}', $2, true),
             updated_at = now()
         WHERE id = $1",
    )
    .bind(tenant_id.as_uuid())
    .bind(flags)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(result.rows_affected() == 1)
}

pub async fn admin_session_for_tenant(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query("SELECT set_config('app.role', 'Admin', true)")
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}

#[derive(sqlx::FromRow)]
struct MrrRecord {
    price_minor: i64,
    price_currency: String,
}

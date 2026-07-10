use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_bypass_rls;

pub async fn find_offboarding_candidates(
    pool: &PgPool,
    retention_days: i32,
) -> Result<Vec<TenantId>, PostgresError> {
    let rows = sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM shared.tenants
         WHERE status = 'Offboarding'
           AND offboarding_scheduled_at IS NOT NULL
           AND offboarding_scheduled_at <= now() - make_interval(days => $1)",
    )
    .bind(retention_days)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(TenantId::from_uuid).collect())
}

pub async fn anonymize_tenant_pii(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    sqlx::query(
        "UPDATE identity.users SET
            email = 'deleted+' || id::text || '@anonymized.local',
            name = 'Anonymized User',
            active = false
         WHERE tenant_id = $1",
    )
    .bind(tenant_id.as_uuid())
    .execute(&mut *tx)
    .await?;
    sqlx::query(
        "UPDATE shared.tenants SET
            status = 'Deleted',
            legal_name = 'Deleted Tenant',
            display_name = 'Deleted',
            name = 'Deleted Tenant',
            active = false,
            settings = settings || '{\"lgpd_export\":\"stub\"}'::jsonb,
            updated_at = now()
         WHERE id = $1",
    )
    .bind(tenant_id.as_uuid())
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

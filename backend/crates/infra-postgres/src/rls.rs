use domain_shared::TenantId;
use sqlx::{PgPool, Postgres, Transaction};

use crate::PostgresError;

/// Sets RLS tenant context on an open transaction.
pub async fn apply_tenant_context(
    tx: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
) -> Result<(), PostgresError> {
    sqlx::query("SELECT set_config('app.tenant_id', $1, true)")
        .bind(tenant_id.to_string())
        .execute(&mut **tx)
        .await?;
    Ok(())
}

/// Sets tenant context in a no-op transaction.
pub async fn set_tenant_context(pool: &PgPool, tenant_id: TenantId) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    tx.commit().await?;
    Ok(())
}

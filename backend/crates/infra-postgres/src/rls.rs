use domain_shared::TenantId;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::PostgresError;

/// Session variables for tenant + role-scoped RLS (RN8).
#[derive(Debug, Clone)]
pub struct SessionContext {
    pub tenant_id: TenantId,
    pub role: String,
    pub user_id: Uuid,
    pub commerce_id: Option<Uuid>,
}

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

/// Enables PlatformAdmin cross-tenant reads (ADR-016).
pub async fn apply_bypass_rls(tx: &mut Transaction<'_, Postgres>) -> Result<(), PostgresError> {
    sqlx::query("SELECT set_config('app.bypass_rls', 'true', true)")
        .execute(&mut **tx)
        .await?;
    Ok(())
}

/// Sets tenant context for impersonation without RLS bypass.
pub async fn apply_impersonation_context(
    tx: &mut Transaction<'_, Postgres>,
    tenant_id: TenantId,
) -> Result<(), PostgresError> {
    sqlx::query("SELECT set_config('app.bypass_rls', 'false', true)")
        .execute(&mut **tx)
        .await?;
    apply_tenant_context(tx, tenant_id).await
}

/// Sets tenant + role context for role-scoped RLS policies.
pub async fn apply_session_context(
    tx: &mut Transaction<'_, Postgres>,
    session: &SessionContext,
) -> Result<(), PostgresError> {
    apply_tenant_context(tx, session.tenant_id).await?;
    sqlx::query("SELECT set_config('app.role', $1, true)")
        .bind(&session.role)
        .execute(&mut **tx)
        .await?;
    sqlx::query("SELECT set_config('app.user_id', $1, true)")
        .bind(session.user_id.to_string())
        .execute(&mut **tx)
        .await?;
    let commerce = session
        .commerce_id
        .map(|id| id.to_string())
        .unwrap_or_default();
    sqlx::query("SELECT set_config('app.commerce_id', $1, true)")
        .bind(commerce)
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

/// Sets full session context in a no-op transaction.
pub async fn set_session_context(
    pool: &PgPool,
    session: &SessionContext,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_session_context(&mut tx, session).await?;
    tx.commit().await?;
    Ok(())
}

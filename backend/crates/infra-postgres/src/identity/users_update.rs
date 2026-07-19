//! Tenant-scoped user profile updates.

use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

pub async fn update_user_profile_tenant(
    pool: &PgPool,
    tenant_id: TenantId,
    user_id: Uuid,
    name: &str,
    email: &str,
    password_hash: Option<&str>,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let result = if let Some(hash) = password_hash {
        sqlx::query(
            "UPDATE identity.users
             SET name = $1, email = $2, password_hash = $3
             WHERE id = $4",
        )
        .bind(name)
        .bind(email)
        .bind(hash)
        .bind(user_id)
        .execute(&mut *tx)
        .await?
    } else {
        sqlx::query(
            "UPDATE identity.users
             SET name = $1, email = $2
             WHERE id = $3",
        )
        .bind(name)
        .bind(email)
        .bind(user_id)
        .execute(&mut *tx)
        .await?
    };
    tx.commit().await?;
    Ok(result.rows_affected() == 1)
}

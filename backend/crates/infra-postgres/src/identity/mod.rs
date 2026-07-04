use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

/// Row persisted in `identity.users`.
#[derive(Debug, Clone)]
pub struct UserRow {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub email: String,
}

/// Inserts a user under the current RLS tenant context.
pub async fn insert_user(
    pool: &PgPool,
    tenant_id: TenantId,
    id: Uuid,
    email: &str,
    name: &str,
    role: &str,
    password_hash: &str,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO identity.users (id, tenant_id, email, name, role, password_hash)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(id)
    .bind(tenant_id.as_uuid())
    .bind(email)
    .bind(name)
    .bind(role)
    .bind(password_hash)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

/// Lists user ids visible under the current tenant RLS context.
pub async fn list_user_ids(pool: &PgPool, tenant_id: TenantId) -> Result<Vec<Uuid>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_scalar::<_, Uuid>("SELECT id FROM identity.users ORDER BY email")
        .fetch_all(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(rows)
}

/// Finds a user by id within tenant RLS scope.
pub async fn find_user_by_id(
    pool: &PgPool,
    tenant_id: TenantId,
    user_id: Uuid,
) -> Result<Option<UserRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, UserRecord>(
        "SELECT id, tenant_id, email FROM identity.users WHERE id = $1",
    )
    .bind(user_id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(|r| UserRow {
        id: r.id,
        tenant_id: TenantId::from_uuid(r.tenant_id),
        email: r.email,
    }))
}

#[derive(sqlx::FromRow)]
struct UserRecord {
    id: Uuid,
    tenant_id: Uuid,
    email: String,
}

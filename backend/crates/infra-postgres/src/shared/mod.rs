use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;

/// Row persisted in `shared.tenants`.
#[derive(Debug, Clone)]
pub struct TenantRow {
    pub id: TenantId,
    pub name: String,
    pub active: bool,
}

/// Inserts a tenant (admin/migration path — no RLS on `shared.tenants`).
pub async fn insert_tenant(pool: &PgPool, id: TenantId, name: &str) -> Result<(), PostgresError> {
    sqlx::query("INSERT INTO shared.tenants (id, name) VALUES ($1, $2)")
        .bind(id.as_uuid())
        .bind(name)
        .execute(pool)
        .await?;
    Ok(())
}

/// Finds a tenant by id.
pub async fn find_tenant_by_id(
    pool: &PgPool,
    id: TenantId,
) -> Result<Option<TenantRow>, PostgresError> {
    let row = sqlx::query_as::<_, TenantRecord>(
        "SELECT id, name, active FROM shared.tenants WHERE id = $1",
    )
    .bind(id.as_uuid())
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| TenantRow {
        id: TenantId::from_uuid(r.id),
        name: r.name,
        active: r.active,
    }))
}

#[derive(sqlx::FromRow)]
struct TenantRecord {
    id: Uuid,
    name: String,
    active: bool,
}

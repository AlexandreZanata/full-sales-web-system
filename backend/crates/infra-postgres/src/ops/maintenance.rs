use chrono::{DateTime, Utc};
use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_bypass_rls;

#[derive(Debug, Clone)]
pub struct MaintenanceWindowRow {
    pub id: Uuid,
    pub tenant_id: Option<TenantId>,
    pub message: String,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
}

pub struct MaintenanceInsert {
    pub id: Uuid,
    pub tenant_id: Option<TenantId>,
    pub message: String,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
}

pub async fn insert_maintenance_window(
    pool: &PgPool,
    row: MaintenanceInsert,
) -> Result<(), PostgresError> {
    sqlx::query(
        "INSERT INTO ops.maintenance_windows (id, tenant_id, message, starts_at, ends_at)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(row.id)
    .bind(row.tenant_id.map(|t| t.as_uuid()))
    .bind(row.message)
    .bind(row.starts_at)
    .bind(row.ends_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find_active_global(
    pool: &PgPool,
) -> Result<Option<MaintenanceWindowRow>, PostgresError> {
    let row = sqlx::query_as::<_, MaintenanceRecord>(
        "SELECT id, tenant_id, message, starts_at, ends_at
         FROM ops.maintenance_windows
         WHERE tenant_id IS NULL AND starts_at <= now() AND ends_at > now()
         ORDER BY starts_at DESC
         LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.map(MaintenanceWindowRow::from))
}

pub async fn find_active_for_tenant(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Option<MaintenanceWindowRow>, PostgresError> {
    let row = sqlx::query_as::<_, MaintenanceRecord>(
        "SELECT id, tenant_id, message, starts_at, ends_at
         FROM ops.maintenance_windows
         WHERE tenant_id = $1 AND starts_at <= now() AND ends_at > now()
         ORDER BY starts_at DESC
         LIMIT 1",
    )
    .bind(tenant_id.as_uuid())
    .fetch_optional(pool)
    .await?;
    Ok(row.map(MaintenanceWindowRow::from))
}

#[derive(sqlx::FromRow)]
struct MaintenanceRecord {
    id: Uuid,
    tenant_id: Option<Uuid>,
    message: String,
    starts_at: DateTime<Utc>,
    ends_at: DateTime<Utc>,
}

impl From<MaintenanceRecord> for MaintenanceWindowRow {
    fn from(row: MaintenanceRecord) -> Self {
        Self {
            id: row.id,
            tenant_id: row.tenant_id.map(TenantId::from_uuid),
            message: row.message,
            starts_at: row.starts_at,
            ends_at: row.ends_at,
        }
    }
}

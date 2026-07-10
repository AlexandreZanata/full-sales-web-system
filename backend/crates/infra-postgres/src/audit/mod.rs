use chrono::{DateTime, Utc};
use domain_audit::ActorType;
use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::{apply_bypass_rls, apply_tenant_context};

#[derive(Debug, Clone)]
pub struct NewAuditEvent {
    pub id: Uuid,
    pub actor_id: Uuid,
    pub actor_type: ActorType,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Uuid,
    pub metadata: Option<serde_json::Value>,
    pub correlation_id: Option<Uuid>,
    pub ip: Option<String>,
}

pub async fn insert_audit_event(
    pool: &PgPool,
    tenant_id: TenantId,
    event: NewAuditEvent,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    insert_row(&mut tx, Some(tenant_id.as_uuid()), &event).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn insert_platform_audit_event(
    pool: &PgPool,
    tenant_id: Option<TenantId>,
    event: NewAuditEvent,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    insert_row(&mut tx, tenant_id.map(|t| t.as_uuid()), &event).await?;
    tx.commit().await?;
    Ok(())
}

async fn insert_row(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    tenant_id: Option<Uuid>,
    event: &NewAuditEvent,
) -> Result<(), PostgresError> {
    sqlx::query(
        "INSERT INTO audit.events
            (id, tenant_id, actor_id, actor_type, action, resource_type, resource_id,
             metadata, correlation_id, ip)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
    )
    .bind(event.id)
    .bind(tenant_id)
    .bind(event.actor_id)
    .bind(event.actor_type.as_str())
    .bind(&event.action)
    .bind(&event.resource_type)
    .bind(event.resource_id)
    .bind(&event.metadata)
    .bind(event.correlation_id)
    .bind(&event.ip)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AuditEventRow {
    pub id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub actor_id: Uuid,
    pub actor_type: String,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Uuid,
    pub metadata: Option<serde_json::Value>,
    pub correlation_id: Option<Uuid>,
    pub ip: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct AuditEventFilters {
    pub tenant_id: Option<Uuid>,
    pub actor_id: Option<Uuid>,
    pub action: Option<String>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

pub async fn list_audit_events_cursor(
    pool: &PgPool,
    tenant_id: TenantId,
    filters: &AuditEventFilters,
    before_id: Option<Uuid>,
    limit: i64,
) -> Result<Vec<AuditEventRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = fetch_rows(&mut tx, filters, before_id, limit).await?;
    tx.commit().await?;
    Ok(rows)
}

pub async fn list_audit_events_platform(
    pool: &PgPool,
    filters: &AuditEventFilters,
    before_id: Option<Uuid>,
    limit: i64,
) -> Result<Vec<AuditEventRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    let rows = fetch_rows(&mut tx, filters, before_id, limit).await?;
    tx.commit().await?;
    Ok(rows)
}

async fn fetch_rows(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    filters: &AuditEventFilters,
    before_id: Option<Uuid>,
    limit: i64,
) -> Result<Vec<AuditEventRow>, PostgresError> {
    Ok(
        sqlx::query_as::<_, AuditEventRow>(
            "SELECT id, tenant_id, actor_id, actor_type, action, resource_type, resource_id,
                metadata, correlation_id, ip, created_at
         FROM audit.events
         WHERE ($1::uuid IS NULL OR id < $1)
           AND ($2::uuid IS NULL OR tenant_id = $2)
           AND ($3::uuid IS NULL OR actor_id = $3)
           AND ($4::text IS NULL OR action = $4)
           AND ($5::timestamptz IS NULL OR created_at >= $5)
           AND ($6::timestamptz IS NULL OR created_at <= $6)
         ORDER BY id DESC
         LIMIT $7",
        )
        .bind(before_id)
        .bind(filters.tenant_id)
        .bind(filters.actor_id)
        .bind(filters.action.as_deref())
        .bind(filters.from)
        .bind(filters.to)
        .bind(limit)
        .fetch_all(&mut **tx)
        .await?,
    )
}

pub async fn count_audit_events(pool: &PgPool, tenant_id: TenantId) -> Result<i64, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM audit.events")
        .fetch_one(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(count)
}

pub async fn list_audit_event_ids(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Vec<Uuid>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows =
        sqlx::query_scalar::<_, Uuid>("SELECT id FROM audit.events ORDER BY created_at DESC")
            .fetch_all(&mut *tx)
            .await?;
    tx.commit().await?;
    Ok(rows)
}

pub async fn list_audit_events(
    pool: &PgPool,
    tenant_id: TenantId,
    limit: i64,
    offset: i64,
) -> Result<Vec<AuditEventRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, AuditEventRow>(
        "SELECT id, tenant_id, actor_id, actor_type, action, resource_type, resource_id,
                metadata, correlation_id, ip, created_at
         FROM audit.events
         ORDER BY created_at DESC
         LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows)
}

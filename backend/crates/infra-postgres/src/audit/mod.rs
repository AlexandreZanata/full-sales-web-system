use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

pub struct NewAuditEvent {
    pub id: Uuid,
    pub actor_id: Uuid,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Uuid,
    pub metadata: Option<serde_json::Value>,
    pub correlation_id: Option<Uuid>,
}

pub struct AuditEventRow {
    pub id: Uuid,
    pub actor_id: Uuid,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Uuid,
    pub metadata: Option<serde_json::Value>,
    pub correlation_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn insert_audit_event(
    pool: &PgPool,
    tenant_id: TenantId,
    event: NewAuditEvent,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO audit.events
         (id, tenant_id, actor_id, action, resource_type, resource_id, metadata, correlation_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(event.id)
    .bind(tenant_id.as_uuid())
    .bind(event.actor_id)
    .bind(&event.action)
    .bind(&event.resource_type)
    .bind(event.resource_id)
    .bind(event.metadata)
    .bind(event.correlation_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
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
    let rows = sqlx::query_as::<_, (
        Uuid,
        Uuid,
        String,
        String,
        Uuid,
        Option<serde_json::Value>,
        Option<Uuid>,
        chrono::DateTime<chrono::Utc>,
    )>(
        "SELECT id, actor_id, action, resource_type, resource_id, metadata, correlation_id, created_at
         FROM audit.events
         ORDER BY created_at DESC
         LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(
            |(
                id,
                actor_id,
                action,
                resource_type,
                resource_id,
                metadata,
                correlation_id,
                created_at,
            )| AuditEventRow {
                id,
                actor_id,
                action,
                resource_type,
                resource_id,
                metadata,
                correlation_id,
                created_at,
            },
        )
        .collect())
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

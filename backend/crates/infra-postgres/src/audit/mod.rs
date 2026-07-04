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

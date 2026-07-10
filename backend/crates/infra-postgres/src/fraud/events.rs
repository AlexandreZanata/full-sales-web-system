use domain_fraud::{FraudEventType, FraudResolution, FraudSeverity};
use domain_shared::TenantId;
use sqlx::Row;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::{apply_bypass_rls, apply_tenant_context};

pub struct FraudEventRow {
    pub id: Uuid,
    pub tenant_id: Option<TenantId>,
    pub user_id: Option<Uuid>,
    pub event_type: String,
    pub severity: String,
    pub status: String,
    pub resolution: Option<String>,
    pub resolution_note: Option<String>,
    pub metadata: serde_json::Value,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct NewFraudEvent {
    pub id: Uuid,
    pub tenant_id: Option<TenantId>,
    pub user_id: Option<Uuid>,
    pub event_type: FraudEventType,
    pub severity: FraudSeverity,
    pub metadata: serde_json::Value,
}

pub struct FraudEventFilters {
    pub status: Option<String>,
    pub severity: Option<String>,
    pub tenant_id: Option<Uuid>,
}

pub async fn insert_fraud_event(
    pool: &PgPool,
    event: NewFraudEvent,
) -> Result<(), PostgresError> {
    sqlx::query(
        "INSERT INTO fraud.fraud_events
         (id, tenant_id, user_id, event_type, severity, metadata)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(event.id)
    .bind(event.tenant_id.map(|t| t.as_uuid()))
    .bind(event.user_id)
    .bind(event.event_type.as_str())
    .bind(event.severity.as_str())
    .bind(event.metadata)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find_fraud_event(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<FraudEventRow>, PostgresError> {
    let row = fetch_one(pool, id).await?;
    Ok(row)
}

pub async fn resolve_fraud_event(
    pool: &PgPool,
    id: Uuid,
    reviewer_id: Uuid,
    resolution: FraudResolution,
    note: Option<&str>,
) -> Result<Option<FraudEventRow>, PostgresError> {
    let status = resolution.resulting_status().as_str();
    sqlx::query(
        "UPDATE fraud.fraud_events
         SET status = $2, resolution = $3, resolution_note = $4,
             reviewed_by = $5, reviewed_at = now()
         WHERE id = $1 AND status = 'Open'",
    )
    .bind(id)
    .bind(status)
    .bind(resolution.as_str())
    .bind(note)
    .bind(reviewer_id)
    .execute(pool)
    .await?;
    fetch_one(pool, id).await
}

pub async fn list_fraud_events_platform(
    pool: &PgPool,
    filters: &FraudEventFilters,
    before_id: Option<Uuid>,
    limit: i64,
) -> Result<Vec<FraudEventRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    let rows = query_rows(
        &mut tx,
        filters,
        before_id,
        limit,
    )
    .await?;
    tx.commit().await?;
    Ok(rows)
}

pub async fn list_fraud_events_tenant(
    pool: &PgPool,
    tenant_id: TenantId,
    limit: i64,
) -> Result<Vec<FraudEventRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = query_rows(
        &mut tx,
        &FraudEventFilters {
            status: None,
            severity: None,
            tenant_id: Some(tenant_id.as_uuid()),
        },
        None,
        limit,
    )
    .await?;
    tx.commit().await?;
    Ok(rows)
}

async fn fetch_one(pool: &PgPool, id: Uuid) -> Result<Option<FraudEventRow>, PostgresError> {
    let row = sqlx::query_as::<_, FraudEventRow>(
        "SELECT id, tenant_id, user_id, event_type, severity, status, resolution,
                resolution_note, metadata, reviewed_by, reviewed_at, created_at
         FROM fraud.fraud_events WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

async fn query_rows(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    filters: &FraudEventFilters,
    before_id: Option<Uuid>,
    limit: i64,
) -> Result<Vec<FraudEventRow>, PostgresError> {
    let rows = sqlx::query_as::<_, FraudEventRow>(
        "SELECT id, tenant_id, user_id, event_type, severity, status, resolution,
                resolution_note, metadata, reviewed_by, reviewed_at, created_at
         FROM fraud.fraud_events
         WHERE ($1::uuid IS NULL OR id < $1)
           AND ($2::text IS NULL OR status = $2)
           AND ($3::text IS NULL OR severity = $3)
           AND ($4::uuid IS NULL OR tenant_id = $4)
         ORDER BY id DESC
         LIMIT $5",
    )
    .bind(before_id)
    .bind(filters.status.as_deref())
    .bind(filters.severity.as_deref())
    .bind(filters.tenant_id)
    .bind(limit)
    .fetch_all(&mut **tx)
    .await?;
    Ok(rows)
}

// sqlx FromRow helper — tenant_id comes as Option<Uuid>
impl sqlx::FromRow<'_, sqlx::postgres::PgRow> for FraudEventRow {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            tenant_id: row.try_get::<Option<Uuid>, _>("tenant_id")?.map(TenantId::from_uuid),
            user_id: row.try_get("user_id")?,
            event_type: row.try_get("event_type")?,
            severity: row.try_get("severity")?,
            status: row.try_get("status")?,
            resolution: row.try_get("resolution")?,
            resolution_note: row.try_get("resolution_note")?,
            metadata: row.try_get("metadata")?,
            reviewed_by: row.try_get("reviewed_by")?,
            reviewed_at: row.try_get("reviewed_at")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

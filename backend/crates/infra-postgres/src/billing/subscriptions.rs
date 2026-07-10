use chrono::{DateTime, Utc};
use domain_billing::SubscriptionStatus;
use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;

#[derive(Debug, Clone)]
pub struct SubscriptionRow {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub plan_id: Uuid,
    pub asaas_subscription_id: Option<String>,
    pub status: SubscriptionStatus,
    pub current_period_end: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct SubscriptionInsert {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub plan_id: Uuid,
    pub asaas_subscription_id: Option<String>,
    pub status: SubscriptionStatus,
    pub current_period_end: Option<DateTime<Utc>>,
}

pub async fn insert_subscription(
    pool: &PgPool,
    sub: SubscriptionInsert,
) -> Result<(), PostgresError> {
    sqlx::query(
        "INSERT INTO billing.subscriptions
            (id, tenant_id, plan_id, asaas_subscription_id, status, current_period_end)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(sub.id)
    .bind(sub.tenant_id.as_uuid())
    .bind(sub.plan_id)
    .bind(&sub.asaas_subscription_id)
    .bind(sub.status.as_str())
    .bind(sub.current_period_end)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find_subscription_by_tenant(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Option<SubscriptionRow>, PostgresError> {
    let row = sqlx::query_as::<_, SubRecord>(
        "SELECT id, tenant_id, plan_id, asaas_subscription_id, status, current_period_end
         FROM billing.subscriptions
         WHERE tenant_id = $1
         ORDER BY created_at DESC
         LIMIT 1",
    )
    .bind(tenant_id.as_uuid())
    .fetch_optional(pool)
    .await?;
    Ok(row.map(SubscriptionRow::from))
}

pub async fn update_subscription_status(
    pool: &PgPool,
    id: Uuid,
    status: SubscriptionStatus,
    current_period_end: Option<DateTime<Utc>>,
) -> Result<bool, PostgresError> {
    let result = sqlx::query(
        "UPDATE billing.subscriptions
         SET status = $2, current_period_end = $3, updated_at = now()
         WHERE id = $1",
    )
    .bind(id)
    .bind(status.as_str())
    .bind(current_period_end)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn update_subscription_plan(
    pool: &PgPool,
    id: Uuid,
    plan_id: Uuid,
    asaas_subscription_id: Option<&str>,
) -> Result<bool, PostgresError> {
    let result = sqlx::query(
        "UPDATE billing.subscriptions
         SET plan_id = $2, asaas_subscription_id = COALESCE($3, asaas_subscription_id), updated_at = now()
         WHERE id = $1",
    )
    .bind(id)
    .bind(plan_id)
    .bind(asaas_subscription_id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() == 1)
}

#[derive(sqlx::FromRow)]
struct SubRecord {
    id: Uuid,
    tenant_id: Uuid,
    plan_id: Uuid,
    asaas_subscription_id: Option<String>,
    status: String,
    current_period_end: Option<DateTime<Utc>>,
}

impl From<SubRecord> for SubscriptionRow {
    fn from(row: SubRecord) -> Self {
        Self {
            id: row.id,
            tenant_id: TenantId::from_uuid(row.tenant_id),
            plan_id: row.plan_id,
            asaas_subscription_id: row.asaas_subscription_id,
            status: SubscriptionStatus::parse(&row.status).unwrap_or(SubscriptionStatus::Pending),
            current_period_end: row.current_period_end,
        }
    }
}

use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;

#[derive(Debug, Clone)]
pub struct PlanRow {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub price_minor: i64,
    pub price_currency: String,
    pub billing_interval: String,
    pub feature_limits: serde_json::Value,
}

pub async fn find_plan(pool: &PgPool, plan_id: Uuid) -> Result<Option<PlanRow>, PostgresError> {
    let row = sqlx::query_as::<_, PlanRecord>(
        "SELECT id, code, name, price_minor, price_currency, billing_interval, feature_limits
         FROM billing.plans WHERE id = $1 AND active = true",
    )
    .bind(plan_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(Into::into))
}

#[derive(sqlx::FromRow)]
struct PlanRecord {
    id: Uuid,
    code: String,
    name: String,
    price_minor: i64,
    price_currency: String,
    billing_interval: String,
    feature_limits: serde_json::Value,
}

impl From<PlanRecord> for PlanRow {
    fn from(row: PlanRecord) -> Self {
        Self {
            id: row.id,
            code: row.code,
            name: row.name,
            price_minor: row.price_minor,
            price_currency: row.price_currency,
            billing_interval: row.billing_interval,
            feature_limits: row.feature_limits,
        }
    }
}

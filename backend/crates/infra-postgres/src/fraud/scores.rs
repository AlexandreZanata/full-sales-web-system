use domain_shared::TenantId;
use sqlx::PgPool;

use crate::PostgresError;

pub async fn add_tenant_fraud_score(
    pool: &PgPool,
    tenant_id: TenantId,
    delta: i32,
) -> Result<i32, PostgresError> {
    let score = sqlx::query_scalar::<_, i32>(
        "INSERT INTO fraud.tenant_fraud_scores (tenant_id, score)
         VALUES ($1, $2)
         ON CONFLICT (tenant_id) DO UPDATE
         SET score = fraud.tenant_fraud_scores.score + EXCLUDED.score,
             updated_at = now()
         RETURNING score",
    )
    .bind(tenant_id.as_uuid())
    .bind(delta)
    .fetch_one(pool)
    .await?;
    Ok(score)
}

pub async fn get_tenant_fraud_score(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<i32, PostgresError> {
    let score = sqlx::query_scalar::<_, Option<i32>>(
        "SELECT score FROM fraud.tenant_fraud_scores WHERE tenant_id = $1",
    )
    .bind(tenant_id.as_uuid())
    .fetch_optional(pool)
    .await?
    .flatten()
    .unwrap_or(0);
    Ok(score)
}

pub async fn get_platform_thresholds(
    pool: &PgPool,
) -> Result<serde_json::Value, PostgresError> {
    let value = sqlx::query_scalar::<_, serde_json::Value>(
        "SELECT thresholds FROM fraud.platform_settings WHERE id = 1",
    )
    .fetch_one(pool)
    .await?;
    Ok(value)
}

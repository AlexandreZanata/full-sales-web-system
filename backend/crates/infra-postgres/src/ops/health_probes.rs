use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;

pub async fn ping_postgres(pool: &PgPool) -> Result<(), PostgresError> {
    sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(pool)
        .await?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct HealthProbeInsert {
    pub probe_name: &'static str,
    pub status: &'static str,
    pub latency_ms: Option<i32>,
    pub details: Value,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct HealthProbeRow {
    pub id: Uuid,
    pub probe_name: String,
    pub status: String,
    pub latency_ms: Option<i32>,
    pub checked_at: DateTime<Utc>,
    pub details: Value,
}

#[derive(Debug, Clone)]
pub struct OpsAlertInsert {
    pub probe_name: &'static str,
    pub message: String,
    pub details: Value,
}

pub async fn insert_probe_result(
    pool: &PgPool,
    probe: HealthProbeInsert,
) -> Result<(), PostgresError> {
    sqlx::query(
        "INSERT INTO ops.health_probe_results (probe_name, status, latency_ms, details)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(probe.probe_name)
    .bind(probe.status)
    .bind(probe.latency_ms)
    .bind(probe.details)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn latest_probe_results(pool: &PgPool) -> Result<Vec<HealthProbeRow>, PostgresError> {
    let rows = sqlx::query_as::<_, HealthProbeRow>(
        "SELECT DISTINCT ON (probe_name)
            id, probe_name, status, latency_ms, checked_at, details
         FROM ops.health_probe_results
         ORDER BY probe_name, checked_at DESC",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn probe_history(
    pool: &PgPool,
    probe_name: &str,
    since: DateTime<Utc>,
) -> Result<Vec<HealthProbeRow>, PostgresError> {
    let rows = sqlx::query_as::<_, HealthProbeRow>(
        "SELECT id, probe_name, status, latency_ms, checked_at, details
         FROM ops.health_probe_results
         WHERE probe_name = $1 AND checked_at >= $2
         ORDER BY checked_at ASC",
    )
    .bind(probe_name)
    .bind(since)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn uptime_pct_24h(pool: &PgPool, probe_name: &str) -> Result<f64, PostgresError> {
    let since = Utc::now() - chrono::Duration::hours(24);
    let row = sqlx::query_as::<_, (i64, i64)>(
        "SELECT
            COUNT(*) FILTER (WHERE status = 'up'),
            COUNT(*)
         FROM ops.health_probe_results
         WHERE probe_name = $1 AND checked_at >= $2",
    )
    .bind(probe_name)
    .bind(since)
    .fetch_one(pool)
    .await?;
    if row.1 == 0 {
        return Ok(100.0);
    }
    Ok((row.0 as f64 / row.1 as f64) * 100.0)
}

pub async fn delete_probe_results_older_than(
    pool: &PgPool,
    days: i32,
) -> Result<u64, PostgresError> {
    let result = sqlx::query(
        "DELETE FROM ops.health_probe_results
         WHERE checked_at < now() - make_interval(days => $1)",
    )
    .bind(days)
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

pub async fn insert_ops_alert(pool: &PgPool, alert: OpsAlertInsert) -> Result<Uuid, PostgresError> {
    let id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO ops.ops_alerts (probe_name, message, details)
         VALUES ($1, $2, $3)
         RETURNING id",
    )
    .bind(alert.probe_name)
    .bind(&alert.message)
    .bind(alert.details)
    .fetch_one(pool)
    .await?;
    Ok(id)
}

pub async fn mark_ops_alert_webhook_sent(pool: &PgPool, id: Uuid) -> Result<(), PostgresError> {
    sqlx::query("UPDATE ops.ops_alerts SET webhook_sent = true WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn count_unprocessed_payment_events(pool: &PgPool) -> Result<i64, PostgresError> {
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM billing.payment_events WHERE status = 'received'",
    )
    .fetch_one(pool)
    .await?;
    Ok(count)
}

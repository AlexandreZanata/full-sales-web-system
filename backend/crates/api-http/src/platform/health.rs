use std::collections::BTreeMap;

use axum::extract::{Query, State};
use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::health::{
    self, PROBE_ASAAS, PROBE_DNS, PROBE_MINIO, PROBE_POSTGRES, PROBE_REDIS, PROBE_WEBHOOK_QUEUE,
};
use crate::state::AppState;

#[derive(Serialize)]
pub struct ProbeMatrixEntry {
    pub status: String,
    #[serde(rename = "latencyMs", skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<i32>,
    #[serde(rename = "checkedAt", skip_serializing_if = "Option::is_none")]
    pub checked_at: Option<String>,
    #[serde(rename = "uptime24hPct")]
    pub uptime_24h_pct: f64,
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    pub details: serde_json::Value,
}

#[derive(Serialize)]
pub struct HealthMatrixResponse {
    pub probes: BTreeMap<String, ProbeMatrixEntry>,
}

pub async fn health_matrix(
    State(state): State<AppState>,
) -> Result<Json<HealthMatrixResponse>, ApiError> {
    let stored = infra_postgres::ops::latest_probe_results(&state.admin_pool)
        .await
        .map_err(|_| ApiError::internal())?;
    let mut probes = BTreeMap::new();
    if stored.is_empty() {
        let live = health::run_all_probes(&state).await;
        for probe in live {
            probes.insert(
                probe.name.to_owned(),
                ProbeMatrixEntry {
                    status: probe.status.as_str().to_owned(),
                    latency_ms: probe.latency_ms,
                    checked_at: Some(Utc::now().to_rfc3339()),
                    uptime_24h_pct: if probe.status.is_healthy() { 100.0 } else { 0.0 },
                    details: probe.details,
                },
            );
        }
    } else {
        for row in stored {
            let uptime = infra_postgres::ops::uptime_pct_24h(&state.admin_pool, &row.probe_name)
                .await
                .map_err(|_| ApiError::internal())?;
            probes.insert(
                row.probe_name.clone(),
                ProbeMatrixEntry {
                    status: row.status,
                    latency_ms: row.latency_ms,
                    checked_at: Some(row.checked_at.to_rfc3339()),
                    uptime_24h_pct: (uptime * 10.0).round() / 10.0,
                    details: row.details,
                },
            );
        }
    }
    Ok(Json(HealthMatrixResponse { probes }))
}

#[derive(Deserialize)]
pub struct HealthHistoryQuery {
    pub probe: String,
    pub since: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct HealthHistoryPoint {
    pub status: String,
    #[serde(rename = "latencyMs", skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<i32>,
    #[serde(rename = "checkedAt")]
    pub checked_at: String,
    pub details: serde_json::Value,
}

#[derive(Serialize)]
pub struct HealthHistoryResponse {
    pub probe: String,
    pub data: Vec<HealthHistoryPoint>,
}

pub async fn health_history(
    State(state): State<AppState>,
    Query(query): Query<HealthHistoryQuery>,
) -> Result<Json<HealthHistoryResponse>, ApiError> {
    if !is_valid_probe(&query.probe) {
        return Err(ApiError::bad_request("INVALID_PROBE", "invalid probe name"));
    }
    let rows = infra_postgres::ops::probe_history(
        &state.admin_pool,
        &query.probe,
        query.since,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    let data = rows
        .into_iter()
        .map(|row| HealthHistoryPoint {
            status: row.status,
            latency_ms: row.latency_ms,
            checked_at: row.checked_at.to_rfc3339(),
            details: row.details,
        })
        .collect();
    Ok(Json(HealthHistoryResponse {
        probe: query.probe,
        data,
    }))
}

fn is_valid_probe(name: &str) -> bool {
    matches!(
        name,
        PROBE_POSTGRES
            | PROBE_REDIS
            | PROBE_MINIO
            | PROBE_ASAAS
            | PROBE_DNS
            | PROBE_WEBHOOK_QUEUE
    )
}

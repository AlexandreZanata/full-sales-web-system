use std::time::Instant;

use serde_json::json;

use super::types::{
    PROBE_ASAAS, PROBE_DNS, PROBE_MINIO, PROBE_POSTGRES, PROBE_REDIS, PROBE_WEBHOOK_QUEUE,
    ProbeOutcome, ProbeStatus,
};
use crate::state::AppState;

const WEBHOOK_LAG_DEGRADED: i64 = 100;
const WEBHOOK_LAG_DOWN: i64 = 1_000;

pub async fn probe_postgres(state: &AppState) -> ProbeOutcome {
    let started = Instant::now();
    match infra_postgres::ops::ping_postgres(&state.admin_pool).await {
        Ok(_) => ProbeOutcome::up(PROBE_POSTGRES, started.elapsed().as_millis() as u64),
        Err(err) => ProbeOutcome::down(
            PROBE_POSTGRES,
            started.elapsed().as_millis() as u64,
            err.to_string(),
        ),
    }
}

pub async fn probe_redis(state: &AppState) -> ProbeOutcome {
    let Some(url) = state.health_config.redis_url.as_deref() else {
        return ProbeOutcome::skipped(PROBE_REDIS);
    };
    let started = Instant::now();
    match infra_redis::ping_redis(url).await {
        Ok(latency) => ProbeOutcome::up(PROBE_REDIS, latency),
        Err(_) => ProbeOutcome::down(
            PROBE_REDIS,
            started.elapsed().as_millis() as u64,
            "redis ping failed",
        ),
    }
}

pub async fn probe_minio(state: &AppState) -> ProbeOutcome {
    let Some(config) = state.health_config.storage_config.as_ref() else {
        return ProbeOutcome::skipped(PROBE_MINIO);
    };
    let started = Instant::now();
    match infra_storage::head_bucket(config).await {
        Ok(latency) => ProbeOutcome::up(PROBE_MINIO, latency),
        Err(err) => ProbeOutcome::down(
            PROBE_MINIO,
            started.elapsed().as_millis() as u64,
            err.to_string(),
        ),
    }
}

pub async fn probe_asaas(state: &AppState) -> ProbeOutcome {
    let started = Instant::now();
    match state.payment_gateway.ping().await {
        Ok(()) => ProbeOutcome::up(PROBE_ASAAS, started.elapsed().as_millis() as u64),
        Err(err) => ProbeOutcome::down(
            PROBE_ASAAS,
            started.elapsed().as_millis() as u64,
            err.to_string(),
        ),
    }
}

pub async fn probe_dns() -> ProbeOutcome {
    let started = Instant::now();
    match tokio::net::lookup_host(("example.com", 80)).await {
        Ok(_) => ProbeOutcome::up(PROBE_DNS, started.elapsed().as_millis() as u64),
        Err(err) => ProbeOutcome::down(
            PROBE_DNS,
            started.elapsed().as_millis() as u64,
            err.to_string(),
        ),
    }
}

pub async fn probe_webhook_queue(state: &AppState) -> ProbeOutcome {
    let started = Instant::now();
    match infra_postgres::ops::count_unprocessed_payment_events(&state.admin_pool).await {
        Ok(count) => webhook_queue_outcome(count, started.elapsed().as_millis() as u64),
        Err(err) => ProbeOutcome::down(
            PROBE_WEBHOOK_QUEUE,
            started.elapsed().as_millis() as u64,
            err.to_string(),
        ),
    }
}

fn webhook_queue_outcome(count: i64, latency: u64) -> ProbeOutcome {
    let latency_ms = Some(latency.min(i32::MAX as u64) as i32);
    let details = json!({ "unprocessed": count });
    let status = if count >= WEBHOOK_LAG_DOWN {
        ProbeStatus::Down
    } else if count >= WEBHOOK_LAG_DEGRADED {
        ProbeStatus::Degraded
    } else {
        ProbeStatus::Up
    };
    ProbeOutcome {
        name: PROBE_WEBHOOK_QUEUE,
        status,
        latency_ms,
        details,
    }
}

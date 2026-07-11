mod checks;
mod types;

pub use types::{
    ComponentStatus, PROBE_ASAAS, PROBE_DNS, PROBE_MINIO, PROBE_POSTGRES, PROBE_REDIS,
    PROBE_WEBHOOK_QUEUE, ProbeOutcome, ProbeStatus,
};

use crate::state::AppState;
use checks::{
    probe_asaas, probe_dns, probe_minio, probe_postgres, probe_redis, probe_webhook_queue,
};

pub async fn run_readiness_probes(state: &AppState) -> Vec<ProbeOutcome> {
    run_all_probes(state)
        .await
        .into_iter()
        .filter(|probe| probe.name != PROBE_DNS && probe.name != PROBE_WEBHOOK_QUEUE)
        .collect()
}

pub async fn run_all_probes(state: &AppState) -> Vec<ProbeOutcome> {
    vec![
        probe_postgres(state).await,
        probe_redis(state).await,
        probe_minio(state).await,
        probe_asaas(state).await,
        probe_dns().await,
        probe_webhook_queue(state).await,
    ]
}

pub fn readiness_is_ready(probes: &[ProbeOutcome]) -> bool {
    probes
        .iter()
        .all(|probe| !probe.status.is_critical_failure())
}

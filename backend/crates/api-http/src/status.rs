use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;

use crate::health::{self, ProbeOutcome, ProbeStatus, PROBE_ASAAS, PROBE_DNS, PROBE_MINIO, PROBE_POSTGRES, PROBE_REDIS, PROBE_WEBHOOK_QUEUE};
use crate::state::AppState;

#[derive(Serialize)]
pub struct PublicStatusResponse {
    pub status: &'static str,
    pub components: PublicComponents,
}

#[derive(Serialize)]
pub struct PublicComponents {
    pub api: PublicComponentStatus,
    pub portal: PublicComponentStatus,
    pub payments: PublicComponentStatus,
    pub storage: PublicComponentStatus,
}

#[derive(Serialize, Copy, Clone)]
pub struct PublicComponentStatus {
    pub status: &'static str,
}

pub async fn public_status(
    State(state): State<AppState>,
) -> (StatusCode, Json<PublicStatusResponse>) {
    let probes = health::run_all_probes(&state).await;
    let api = aggregate_api_status(&probes);
    let portal = api;
    let payments = component_from_probe(find_probe(&probes, PROBE_ASAAS));
    let storage = component_from_probe(find_probe(&probes, PROBE_MINIO));
    let overall = worst_status(&[api.status, portal.status, payments.status, storage.status]);
    let code = if overall == "down" {
        StatusCode::SERVICE_UNAVAILABLE
    } else {
        StatusCode::OK
    };
    (
        code,
        Json(PublicStatusResponse {
            status: overall,
            components: PublicComponents {
                api: PublicComponentStatus { status: api.status },
                portal: PublicComponentStatus { status: portal.status },
                payments: PublicComponentStatus {
                    status: payments.status,
                },
                storage: PublicComponentStatus {
                    status: storage.status,
                },
            },
        }),
    )
}

fn aggregate_api_status(probes: &[ProbeOutcome]) -> PublicComponentStatus {
    let statuses: Vec<&'static str> = [
        find_probe(probes, PROBE_POSTGRES),
        find_probe(probes, PROBE_REDIS),
        find_probe(probes, PROBE_DNS),
        find_probe(probes, PROBE_WEBHOOK_QUEUE),
    ]
    .into_iter()
    .filter_map(|probe| probe.map(public_probe_status))
    .collect();
    PublicComponentStatus {
        status: worst_status(&statuses),
    }
}

fn find_probe<'a>(probes: &'a [ProbeOutcome], name: &str) -> Option<&'a ProbeOutcome> {
    probes.iter().find(|probe| probe.name == name)
}

fn component_from_probe(probe: Option<&ProbeOutcome>) -> PublicComponentStatus {
    PublicComponentStatus {
        status: probe.map(public_probe_status).unwrap_or("up"),
    }
}

fn public_probe_status(probe: &ProbeOutcome) -> &'static str {
    match probe.status {
        ProbeStatus::Up | ProbeStatus::Skipped => "up",
        ProbeStatus::Degraded => "degraded",
        ProbeStatus::Down => "down",
    }
}

fn worst_status(statuses: &[&'static str]) -> &'static str {
    if statuses.is_empty() {
        return "operational";
    }
    if statuses.contains(&"down") {
        "down"
    } else if statuses.contains(&"degraded") {
        "degraded"
    } else {
        "operational"
    }
}

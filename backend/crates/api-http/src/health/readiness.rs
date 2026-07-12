use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde::Serialize;
use std::collections::BTreeMap;

use crate::health::probes::{self, ComponentStatus, ProbeOutcome};
use crate::state::AppState;

#[derive(Serialize)]
pub struct ReadinessResponse {
    pub status: &'static str,
    pub components: BTreeMap<&'static str, ComponentStatus>,
}

pub async fn readiness(State(state): State<AppState>) -> (StatusCode, Json<ReadinessResponse>) {
    let outcomes = probes::run_readiness_probes(&state).await;
    let ready = probes::readiness_is_ready(&outcomes);
    let status_code = if ready {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    let components = components_from_probes(&outcomes);
    (
        status_code,
        Json(ReadinessResponse {
            status: if ready { "ready" } else { "not_ready" },
            components,
        }),
    )
}

pub fn components_from_probes(
    outcomes: &[ProbeOutcome],
) -> BTreeMap<&'static str, ComponentStatus> {
    outcomes
        .iter()
        .map(|probe| (probe.name, ComponentStatus::from(probe)))
        .collect()
}

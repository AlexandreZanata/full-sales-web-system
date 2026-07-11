use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use std::time::Duration;

use reqwest::Client;
use serde_json::json;
use tracing::{error, info, warn};

use crate::health::alerting::AlertConfig;
use crate::health::probes::{self, ProbeOutcome, ProbeStatus};
use crate::state::AppState;

static FAILURE_STREAKS: LazyLock<Mutex<HashMap<&'static str, u32>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn probe_interval() -> Duration {
    std::env::var("HEALTH_PROBE_INTERVAL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .map(Duration::from_secs)
        .unwrap_or(Duration::from_secs(60))
}

pub fn retention_days() -> i32 {
    std::env::var("HEALTH_PROBE_RETENTION_DAYS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(30)
}

pub async fn run_health_worker(state: AppState) {
    info!("health probe worker started");
    let interval = probe_interval();
    let alert_config = AlertConfig::from_env();
    loop {
        if let Err(err) = run_probe_cycle(&state, &alert_config).await {
            error!(error = %err, "health probe cycle failed");
        }
        tokio::time::sleep(interval).await;
    }
}

pub async fn run_probe_cycle(state: &AppState, alert_config: &AlertConfig) -> Result<(), String> {
    let outcomes = probes::run_all_probes(state).await;
    store_probe_results(state, &outcomes).await?;
    handle_alerting(state, alert_config, &outcomes).await?;
    cleanup_old_results(state).await?;
    Ok(())
}

async fn store_probe_results(state: &AppState, outcomes: &[ProbeOutcome]) -> Result<(), String> {
    for probe in outcomes {
        if probe.status == ProbeStatus::Skipped {
            continue;
        }
        infra_postgres::ops::insert_probe_result(
            &state.admin_pool,
            infra_postgres::ops::HealthProbeInsert {
                probe_name: probe.name,
                status: probe.status.as_str(),
                latency_ms: probe.latency_ms,
                details: probe.details.clone(),
            },
        )
        .await
        .map_err(|err| err.to_string())?;
    }
    Ok(())
}

async fn handle_alerting(
    state: &AppState,
    config: &AlertConfig,
    outcomes: &[ProbeOutcome],
) -> Result<(), String> {
    for probe in outcomes {
        if probe.status == ProbeStatus::Skipped {
            continue;
        }
        let failing = matches!(probe.status, ProbeStatus::Down | ProbeStatus::Degraded);
        let streak = update_failure_streak(probe.name, failing);
        if failing && streak >= config.consecutive_threshold {
            let message = format!(
                "Probe {} is {} ({} consecutive failures)",
                probe.name,
                probe.status.as_str(),
                streak
            );
            let alert_id = infra_postgres::ops::insert_ops_alert(
                &state.admin_pool,
                infra_postgres::ops::OpsAlertInsert {
                    probe_name: probe.name,
                    message: message.clone(),
                    details: probe.details.clone(),
                },
            )
            .await
            .map_err(|err| err.to_string())?;
            if let Some(url) = config.webhook_url.as_deref() {
                if send_webhook(url, &message).await {
                    let _ = infra_postgres::ops::mark_ops_alert_webhook_sent(
                        &state.admin_pool,
                        alert_id,
                    )
                    .await;
                }
            }
            reset_failure_streak(probe.name);
        }
    }
    Ok(())
}

fn update_failure_streak(name: &'static str, failing: bool) -> u32 {
    let mut guard = FAILURE_STREAKS.lock().expect("failure streak lock");
    if failing {
        let entry = guard.entry(name).or_insert(0);
        *entry += 1;
        *entry
    } else {
        guard.remove(name);
        0
    }
}

fn reset_failure_streak(name: &'static str) {
    let mut guard = FAILURE_STREAKS.lock().expect("failure streak lock");
    guard.remove(name);
}

async fn cleanup_old_results(state: &AppState) -> Result<(), String> {
    let deleted =
        infra_postgres::ops::delete_probe_results_older_than(&state.admin_pool, retention_days())
            .await
            .map_err(|err| err.to_string())?;
    if deleted > 0 {
        info!(deleted, "pruned old health probe results");
    }
    Ok(())
}

async fn send_webhook(url: &str, message: &str) -> bool {
    let client = Client::new();
    let body = json!({ "text": message, "content": message });
    match client.post(url).json(&body).send().await {
        Ok(resp) if resp.status().is_success() => true,
        Ok(resp) => {
            warn!(status = %resp.status(), "ops alert webhook rejected");
            false
        }
        Err(err) => {
            warn!(error = %err, "ops alert webhook failed");
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::health::PROBE_POSTGRES;

    #[test]
    fn given_failures_when_threshold_reached_then_streak_resets_after_alert() {
        assert_eq!(update_failure_streak(PROBE_POSTGRES, true), 1);
        assert_eq!(update_failure_streak(PROBE_POSTGRES, true), 2);
        reset_failure_streak(PROBE_POSTGRES);
        assert_eq!(update_failure_streak(PROBE_POSTGRES, true), 1);
    }
}

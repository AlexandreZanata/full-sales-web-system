use serde::Serialize;
use serde_json::{Value, json};

pub const PROBE_POSTGRES: &str = "postgres";
pub const PROBE_REDIS: &str = "redis";
pub const PROBE_MINIO: &str = "minio";
pub const PROBE_ASAAS: &str = "asaas";
pub const PROBE_DNS: &str = "dns";
pub const PROBE_WEBHOOK_QUEUE: &str = "webhook_queue";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeStatus {
    Up,
    Down,
    Degraded,
    Skipped,
}

impl ProbeStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Up => "up",
            Self::Down => "down",
            Self::Degraded => "degraded",
            Self::Skipped => "skipped",
        }
    }

    pub fn is_healthy(self) -> bool {
        matches!(self, Self::Up | Self::Skipped)
    }

    pub fn is_critical_failure(self) -> bool {
        self == Self::Down
    }
}

#[derive(Debug, Clone)]
pub struct ProbeOutcome {
    pub name: &'static str,
    pub status: ProbeStatus,
    pub latency_ms: Option<i32>,
    pub details: Value,
}

impl ProbeOutcome {
    pub fn up(name: &'static str, latency: u64) -> Self {
        Self {
            name,
            status: ProbeStatus::Up,
            latency_ms: Some(latency.min(i32::MAX as u64) as i32),
            details: json!({}),
        }
    }

    pub fn down(name: &'static str, latency: u64, error: impl Into<String>) -> Self {
        Self {
            name,
            status: ProbeStatus::Down,
            latency_ms: Some(latency.min(i32::MAX as u64) as i32),
            details: json!({ "error": error.into() }),
        }
    }

    pub fn skipped(name: &'static str) -> Self {
        Self {
            name,
            status: ProbeStatus::Skipped,
            latency_ms: None,
            details: json!({ "skipped": true }),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ComponentStatus {
    pub status: &'static str,
    #[serde(rename = "latencyMs", skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<i32>,
    #[serde(skip_serializing_if = "Value::is_null")]
    pub details: Value,
}

impl From<&ProbeOutcome> for ComponentStatus {
    fn from(probe: &ProbeOutcome) -> Self {
        Self {
            status: probe.status.as_str(),
            latency_ms: probe.latency_ms,
            details: probe.details.clone(),
        }
    }
}

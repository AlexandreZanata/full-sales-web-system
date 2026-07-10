use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

#[derive(Debug, Default)]
pub struct AsaasMetrics {
    pub requests_total: AtomicU64,
    pub errors_total: AtomicU64,
    pub duration_ms_total: AtomicU64,
}

impl AsaasMetrics {
    pub fn record_success(&self, started: Instant) {
        self.requests_total.fetch_add(1, Ordering::Relaxed);
        let ms = started.elapsed().as_millis() as u64;
        self.duration_ms_total.fetch_add(ms, Ordering::Relaxed);
        tracing::debug!(
            target: "asaas_metrics",
            metric = "asaas_request_duration",
            duration_ms = ms,
        );
    }

    pub fn record_error(&self, started: Instant) {
        self.requests_total.fetch_add(1, Ordering::Relaxed);
        self.errors_total.fetch_add(1, Ordering::Relaxed);
        let ms = started.elapsed().as_millis() as u64;
        self.duration_ms_total.fetch_add(ms, Ordering::Relaxed);
        tracing::warn!(
            target: "asaas_metrics",
            metric = "asaas_errors_total",
            duration_ms = ms,
        );
    }
}

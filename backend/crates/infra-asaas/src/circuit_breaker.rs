use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct CircuitBreaker {
    threshold: u32,
    failures: Mutex<u32>,
    open_until: Mutex<Option<Instant>>,
    cooldown: Duration,
}

impl CircuitBreaker {
    pub fn new(threshold: u32) -> Self {
        Self {
            threshold,
            failures: Mutex::new(0),
            open_until: Mutex::new(None),
            cooldown: Duration::from_secs(30),
        }
    }

    pub fn allow(&self) -> bool {
        let mut open = self.open_until.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(until) = *open {
            if Instant::now() < until {
                return false;
            }
            *open = None;
            *self.failures.lock().unwrap_or_else(|e| e.into_inner()) = 0;
        }
        true
    }

    pub fn record_success(&self) {
        *self.failures.lock().unwrap_or_else(|e| e.into_inner()) = 0;
    }

    pub fn record_failure(&self) {
        let mut failures = self.failures.lock().unwrap_or_else(|e| e.into_inner());
        *failures += 1;
        if *failures >= self.threshold {
            let mut open = self.open_until.lock().unwrap_or_else(|e| e.into_inner());
            *open = Some(Instant::now() + self.cooldown);
            tracing::warn!(target: "asaas_circuit", "circuit breaker opened");
        }
    }
}

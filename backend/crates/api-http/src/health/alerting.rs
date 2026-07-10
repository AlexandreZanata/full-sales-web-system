pub struct AlertConfig {
    pub webhook_url: Option<String>,
    pub consecutive_threshold: u32,
}

impl AlertConfig {
    pub fn from_env() -> Self {
        Self {
            webhook_url: std::env::var("OPS_ALERT_WEBHOOK").ok(),
            consecutive_threshold: std::env::var("HEALTH_ALERT_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3),
        }
    }
}

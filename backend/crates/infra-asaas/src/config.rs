use std::time::Duration;

pub const DEFAULT_BASE_URL: &str = "https://api-sandbox.asaas.com/v3";
pub const DEFAULT_TIMEOUT_SECS: u64 = 15;
pub const DEFAULT_MAX_RETRIES: u32 = 3;
pub const DEFAULT_CIRCUIT_THRESHOLD: u32 = 5;
pub const USER_AGENT: &str = "FullSales/0.1.0";

#[derive(Debug, Clone)]
pub struct AsaasConfig {
    pub base_url: String,
    pub api_key: String,
    pub timeout: Duration,
    pub max_retries: u32,
    pub circuit_threshold: u32,
}

impl AsaasConfig {
    pub fn from_env() -> Result<Self, String> {
        let api_key = std::env::var("ASAAS_API_KEY")
            .map_err(|_| "ASAAS_API_KEY is required for Asaas client".to_string())?;
        if api_key.trim().is_empty() {
            return Err("ASAAS_API_KEY must not be empty".into());
        }
        let base_url = std::env::var("ASAAS_BASE_URL")
            .unwrap_or_else(|_| DEFAULT_BASE_URL.into());
        let timeout_secs = std::env::var("ASAAS_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_TIMEOUT_SECS);
        let max_retries = std::env::var("ASAAS_MAX_RETRIES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_MAX_RETRIES);
        let circuit_threshold = std::env::var("ASAAS_CIRCUIT_THRESHOLD")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_CIRCUIT_THRESHOLD);
        Ok(Self {
            base_url: base_url.trim_end_matches('/').to_owned(),
            api_key,
            timeout: Duration::from_secs(timeout_secs),
            max_retries,
            circuit_threshold,
        })
    }
}

use async_trait::async_trait;
use uuid::Uuid;

use super::{CnpjLookupError, CnpjLookupProvider, CnpjLookupResult};
use super::opencnpj_map::{PublicCnpjResponse, map_opencnpj_response};

pub(crate) const DEFAULT_BASE_URL: &str = "https://api.comerc.app.br";
const TIMEOUT_SECS: u64 = 8;
const RETRY_BACKOFF_MS: u64 = 500;

pub struct OpenCnpjLookup {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
}

#[derive(serde::Deserialize)]
struct ErrorResponse {
    error: Option<String>,
}

enum FetchFailure {
    NotFound,
    Unavailable,
    GatewayTimeout,
}

impl OpenCnpjLookup {
    pub fn from_env() -> Result<Self, String> {
        let api_key = std::env::var("CNPJ_LOOKUP_API_KEY").map_err(|_| {
            "CNPJ_LOOKUP_API_KEY is required when CNPJ_LOOKUP_PROVIDER=opencnpj".to_string()
        })?;
        let base_url =
            std::env::var("CNPJ_LOOKUP_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.into());
        Self::from_config(base_url, api_key)
    }

    pub fn from_config(base_url: String, api_key: String) -> Result<Self, String> {
        if api_key.trim().is_empty() {
            return Err(
                "CNPJ_LOOKUP_API_KEY is required when CNPJ_LOOKUP_PROVIDER=opencnpj".to_string(),
            );
        }
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(TIMEOUT_SECS))
            .build()
            .map_err(|err| format!("failed to build HTTP client: {err}"))?;
        Ok(Self {
            client,
            base_url,
            api_key,
        })
    }

    pub fn new(base_url: String, api_key: String, client: reqwest::Client) -> Self {
        Self {
            client,
            base_url,
            api_key,
        }
    }

    fn classify_failure(status: reqwest::StatusCode, error_code: Option<&str>) -> FetchFailure {
        if status == reqwest::StatusCode::GATEWAY_TIMEOUT {
            return FetchFailure::GatewayTimeout;
        }
        if status == reqwest::StatusCode::NOT_FOUND || error_code == Some("cnpj_not_found") {
            return FetchFailure::NotFound;
        }
        FetchFailure::Unavailable
    }

    async fn fetch_once(&self, cnpj: &str) -> Result<CnpjLookupResult, FetchFailure> {
        let url = format!(
            "{}/api/v1/cnpj/{}",
            self.base_url.trim_end_matches('/'),
            cnpj
        );
        let response = self
            .client
            .get(url)
            .header("X-API-Key", &self.api_key)
            .header("X-Request-ID", Uuid::now_v7().to_string())
            .send()
            .await
            .map_err(|_| FetchFailure::Unavailable)?;
        let status = response.status();
        if status.is_success() {
            let bytes = response
                .bytes()
                .await
                .map_err(|_| FetchFailure::Unavailable)?;
            let snapshot: serde_json::Value =
                serde_json::from_slice(&bytes).map_err(|_| FetchFailure::Unavailable)?;
            let body: PublicCnpjResponse =
                serde_json::from_slice(&bytes).map_err(|_| FetchFailure::Unavailable)?;
            return Ok(map_opencnpj_response(body, snapshot));
        }
        let error_code = response
            .json::<ErrorResponse>()
            .await
            .ok()
            .and_then(|body| body.error);
        Err(Self::classify_failure(status, error_code.as_deref()))
    }

    fn map_fetch_failure(failure: FetchFailure) -> CnpjLookupError {
        match failure {
            FetchFailure::NotFound => CnpjLookupError::NotFound,
            FetchFailure::Unavailable | FetchFailure::GatewayTimeout => {
                CnpjLookupError::Unavailable
            }
        }
    }
}

#[async_trait]
impl CnpjLookupProvider for OpenCnpjLookup {
    async fn lookup(&self, cnpj: &str) -> Result<CnpjLookupResult, CnpjLookupError> {
        match self.fetch_once(cnpj).await {
            Ok(result) => Ok(result),
            Err(FetchFailure::GatewayTimeout) => {
                tokio::time::sleep(std::time::Duration::from_millis(RETRY_BACKOFF_MS)).await;
                self.fetch_once(cnpj)
                    .await
                    .map_err(Self::map_fetch_failure)
            }
            Err(other) => Err(Self::map_fetch_failure(other)),
        }
    }
}

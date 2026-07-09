use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use super::{CnpjLookupAddress, CnpjLookupError, CnpjLookupProvider, CnpjLookupResult};

pub(crate) const DEFAULT_BASE_URL: &str = "https://api.comerc.app.br";
const TIMEOUT_SECS: u64 = 8;
const RETRY_BACKOFF_MS: u64 = 500;

pub struct OpenCnpjLookup {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
}

#[derive(serde::Deserialize)]
struct OpenCnpjEndereco {
    logradouro: Option<String>,
    numero: Option<String>,
    bairro: Option<String>,
    municipio: Option<String>,
    uf: Option<String>,
    cep: Option<String>,
}

#[derive(serde::Deserialize)]
struct PublicCnpjResponse {
    cnpj: String,
    razao_social: String,
    nome_fantasia: Option<String>,
    uf: Option<String>,
    municipio: Option<String>,
    endereco: Option<OpenCnpjEndereco>,
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

    fn map_response(body: PublicCnpjResponse) -> CnpjLookupResult {
        let legal_name = body.razao_social;
        let trade_name = body
            .nome_fantasia
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| legal_name.clone());
        let endereco = body.endereco;
        let city = endereco
            .as_ref()
            .and_then(|addr| addr.municipio.clone())
            .or(body.municipio)
            .unwrap_or_default();
        let state = endereco
            .as_ref()
            .and_then(|addr| addr.uf.clone())
            .or(body.uf)
            .unwrap_or_default();
        let postal_code = endereco
            .as_ref()
            .and_then(|addr| addr.cep.clone())
            .unwrap_or_default()
            .replace(['-', '.'], "");
        CnpjLookupResult {
            cnpj: body.cnpj,
            legal_name: legal_name.clone(),
            trade_name,
            address: CnpjLookupAddress {
                street: endereco
                    .as_ref()
                    .and_then(|addr| addr.logradouro.clone())
                    .unwrap_or_default(),
                number: endereco
                    .as_ref()
                    .and_then(|addr| addr.numero.clone())
                    .unwrap_or_else(|| "S/N".into()),
                district: endereco
                    .as_ref()
                    .and_then(|addr| addr.bairro.clone())
                    .unwrap_or_default(),
                city,
                state,
                postal_code,
            },
            provider: "opencnpj".into(),
            fetched_at: Utc::now(),
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
            let body: PublicCnpjResponse = response
                .json()
                .await
                .map_err(|_| FetchFailure::Unavailable)?;
            return Ok(Self::map_response(body));
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

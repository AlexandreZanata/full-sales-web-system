use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;

use super::{CnpjLookupAddress, CnpjLookupError, CnpjLookupProvider, CnpjLookupResult};

pub struct BrasilApiCnpjLookup {
    client: reqwest::Client,
    base_url: String,
}

impl BrasilApiCnpjLookup {
    pub fn from_env() -> Self {
        let base_url = std::env::var("CNPJ_LOOKUP_URL")
            .unwrap_or_else(|_| "https://brasilapi.com.br/api/cnpj/v1".into());
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(8))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
            base_url,
        }
    }
}

#[derive(serde::Deserialize)]
struct BrasilApiResponse {
    cnpj: String,
    razao_social: String,
    nome_fantasia: Option<String>,
    logradouro: Option<String>,
    numero: Option<String>,
    bairro: Option<String>,
    municipio: Option<String>,
    uf: Option<String>,
    cep: Option<String>,
}

#[async_trait]
impl CnpjLookupProvider for BrasilApiCnpjLookup {
    async fn lookup(&self, cnpj: &str) -> Result<CnpjLookupResult, CnpjLookupError> {
        let url = format!("{}/{}", self.base_url.trim_end_matches('/'), cnpj);
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|_| CnpjLookupError::Unavailable)?;
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(CnpjLookupError::NotFound);
        }
        if !response.status().is_success() {
            return Err(CnpjLookupError::Unavailable);
        }
        let body: BrasilApiResponse = response
            .json()
            .await
            .map_err(|_| CnpjLookupError::Unavailable)?;
        let legal_name = body.razao_social;
        Ok(CnpjLookupResult {
            cnpj: body.cnpj,
            legal_name: legal_name.clone(),
            trade_name: body
                .nome_fantasia
                .filter(|v| !v.trim().is_empty())
                .unwrap_or(legal_name),
            address: CnpjLookupAddress {
                street: body.logradouro.unwrap_or_default(),
                number: body.numero.unwrap_or_else(|| "S/N".into()),
                district: body.bairro.unwrap_or_default(),
                city: body.municipio.unwrap_or_default(),
                state: body.uf.unwrap_or_default(),
                postal_code: body.cep.unwrap_or_default().replace(['-', '.'], ""),
            },
            provider: "brasilapi".into(),
            fetched_at: Utc::now(),
        })
    }
}

pub fn build_cnpj_lookup_provider(
    provider: &str,
) -> Result<Arc<dyn CnpjLookupProvider>, String> {
    match provider {
        "mock" => Ok(Arc::new(super::MockCnpjLookup)),
        "opencnpj" => Ok(Arc::new(super::opencnpj::OpenCnpjLookup::from_env()?)),
        _ => Ok(Arc::new(BrasilApiCnpjLookup::from_env())),
    }
}

#[cfg(test)]
pub fn build_cnpj_lookup_provider_with_key(
    provider: &str,
    api_key: Option<&str>,
) -> Result<Arc<dyn CnpjLookupProvider>, String> {
    match provider {
        "mock" => Ok(Arc::new(super::MockCnpjLookup)),
        "opencnpj" => {
            let key = api_key.ok_or_else(|| {
                "CNPJ_LOOKUP_API_KEY is required when CNPJ_LOOKUP_PROVIDER=opencnpj".to_string()
            })?;
            Ok(Arc::new(super::opencnpj::OpenCnpjLookup::from_config(
                super::opencnpj::DEFAULT_BASE_URL.into(),
                key.to_string(),
            )?))
        }
        _ => Ok(Arc::new(BrasilApiCnpjLookup::from_env())),
    }
}

pub fn cnpj_lookup_from_env() -> Result<Arc<dyn CnpjLookupProvider>, String> {
    let provider = std::env::var("CNPJ_LOOKUP_PROVIDER").unwrap_or_else(|_| "brasilapi".into());
    build_cnpj_lookup_provider(&provider)
}

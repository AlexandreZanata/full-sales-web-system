use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;

pub mod brasil_api;
pub mod opencnpj;

use brasil_api::cnpj_lookup_from_env;

#[derive(Debug, Clone, Serialize)]
pub struct CnpjLookupAddress {
    pub street: String,
    pub number: String,
    pub district: String,
    pub city: String,
    pub state: String,
    #[serde(rename = "postalCode")]
    pub postal_code: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CnpjLookupResult {
    pub cnpj: String,
    #[serde(rename = "legalName")]
    pub legal_name: String,
    #[serde(rename = "tradeName")]
    pub trade_name: String,
    pub address: CnpjLookupAddress,
    pub provider: String,
    #[serde(rename = "fetchedAt")]
    pub fetched_at: DateTime<Utc>,
}

#[derive(Debug)]
pub enum CnpjLookupError {
    NotFound,
    Unavailable,
}

#[async_trait]
pub trait CnpjLookupProvider: Send + Sync {
    async fn lookup(&self, cnpj: &str) -> Result<CnpjLookupResult, CnpjLookupError>;
}

pub struct MockCnpjLookup;

#[async_trait]
impl CnpjLookupProvider for MockCnpjLookup {
    async fn lookup(&self, cnpj: &str) -> Result<CnpjLookupResult, CnpjLookupError> {
        if cnpj == "11222333000181" {
            Ok(CnpjLookupResult {
                cnpj: cnpj.to_owned(),
                legal_name: "Acme Comercio Ltda".into(),
                trade_name: "Acme Store".into(),
                address: CnpjLookupAddress {
                    street: "Rua Example".into(),
                    number: "100".into(),
                    district: "Centro".into(),
                    city: "São Paulo".into(),
                    state: "SP".into(),
                    postal_code: "01001000".into(),
                },
                provider: "mock".into(),
                fetched_at: Utc::now(),
            })
        } else {
            Err(CnpjLookupError::NotFound)
        }
    }
}

pub fn default_cnpj_lookup_provider() -> std::sync::Arc<dyn CnpjLookupProvider> {
    cnpj_lookup_from_env().unwrap_or_else(|err| {
        eprintln!("CNPJ lookup provider misconfiguration: {err}");
        std::process::exit(1);
    })
}

#[cfg(test)]
mod provider_env {
    use super::brasil_api::{build_cnpj_lookup_provider, build_cnpj_lookup_provider_with_key};
    use super::opencnpj::OpenCnpjLookup;

    #[test]
    fn given_opencnpj_without_api_key_when_build_then_err() {
        assert!(build_cnpj_lookup_provider_with_key("opencnpj", None).is_err());
    }

    #[test]
    fn given_opencnpj_with_api_key_when_build_then_ok() {
        assert!(build_cnpj_lookup_provider_with_key("opencnpj", Some("test-key")).is_ok());
    }

    #[test]
    fn given_blank_api_key_when_from_config_then_err() {
        assert!(OpenCnpjLookup::from_config("http://localhost".into(), "  ".into()).is_err());
    }

    #[test]
    fn given_mock_when_build_then_ok() {
        assert!(build_cnpj_lookup_provider("mock").is_ok());
    }

    #[test]
    fn given_brasilapi_when_build_then_ok() {
        assert!(build_cnpj_lookup_provider("brasilapi").is_ok());
    }
}

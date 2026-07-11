use std::sync::Arc;
use std::time::Instant;

use domain_billing::BillingError;
use reqwest::Client;
use serde::Deserialize;

use crate::circuit_breaker::CircuitBreaker;
use crate::config::USER_AGENT;
use crate::error_map::map_status;
use crate::metrics::AsaasMetrics;
use crate::sanitize::mask_api_key;

#[derive(Debug, Clone, Deserialize)]
pub struct MyAccountResponse {
    pub name: String,
    #[serde(default)]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatePaymentResponse {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BalanceResponse {
    pub balance: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FinancialTransaction {
    pub id: String,
    #[serde(rename = "type")]
    pub tx_type: String,
    pub value: f64,
    pub date: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FinancialTransactionsResponse {
    pub data: Vec<FinancialTransaction>,
    pub has_more: bool,
}

/// Tenant-scoped Asaas HTTP client using the tenant's own API key (ADR-018).
#[derive(Clone)]
pub struct TenantAsaasClient {
    client: Client,
    api_key: String,
    base_url: String,
    circuit: Arc<CircuitBreaker>,
    metrics: Arc<AsaasMetrics>,
    max_retries: u32,
}

impl TenantAsaasClient {
    pub fn new(api_key: String, base_url: Option<String>) -> Result<Self, String> {
        let base = base_url.unwrap_or_else(|| {
            std::env::var("ASAAS_BASE_URL")
                .unwrap_or_else(|_| crate::config::DEFAULT_BASE_URL.into())
        });
        let timeout_secs = std::env::var("ASAAS_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(crate::config::DEFAULT_TIMEOUT_SECS);
        let max_retries = std::env::var("ASAAS_MAX_RETRIES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(crate::config::DEFAULT_MAX_RETRIES);
        let circuit_threshold = std::env::var("ASAAS_CIRCUIT_THRESHOLD")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(crate::config::DEFAULT_CIRCUIT_THRESHOLD);
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(timeout_secs))
            .build()
            .map_err(|err| format!("failed to build tenant Asaas client: {err}"))?;
        tracing::info!(
            base_url = %base,
            api_key = %mask_api_key(&api_key),
            "tenant Asaas client configured"
        );
        Ok(Self {
            client,
            api_key,
            base_url: base,
            circuit: Arc::new(CircuitBreaker::new(circuit_threshold)),
            metrics: Arc::new(AsaasMetrics::default()),
            max_retries,
        })
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn authed(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        self.client
            .request(method, self.url(path))
            .header("access_token", &self.api_key)
            .header("User-Agent", USER_AGENT)
            .header("Content-Type", "application/json")
    }

    async fn send(
        &self,
        build: impl Fn() -> reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, BillingError> {
        if !self.circuit.allow() {
            return Err(BillingError::CircuitOpen);
        }
        let started = Instant::now();
        let mut attempt = 0u32;
        loop {
            let response = build()
                .send()
                .await
                .map_err(|_| BillingError::UpstreamUnavailable);
            match response {
                Ok(resp) if resp.status().is_success() => {
                    self.circuit.record_success();
                    self.metrics.record_success(started);
                    return Ok(resp);
                }
                Ok(resp) if resp.status() == reqwest::StatusCode::UNAUTHORIZED => {
                    self.circuit.record_failure();
                    self.metrics.record_error(started);
                    return Err(BillingError::InvalidCredentials);
                }
                Ok(resp)
                    if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS
                        || resp.status().is_server_error() =>
                {
                    attempt += 1;
                    if attempt > self.max_retries {
                        self.circuit.record_failure();
                        self.metrics.record_error(started);
                        let status = resp.status();
                        let body = resp.text().await.unwrap_or_default();
                        return Err(map_status(status, &body));
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(
                        1000 << (attempt - 1).min(2),
                    ))
                    .await;
                }
                Ok(resp) => {
                    self.circuit.record_failure();
                    self.metrics.record_error(started);
                    let status = resp.status();
                    let body = resp.text().await.unwrap_or_default();
                    return Err(map_status(status, &body));
                }
                Err(err) => {
                    attempt += 1;
                    if attempt > self.max_retries {
                        self.circuit.record_failure();
                        self.metrics.record_error(started);
                        return Err(err);
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(
                        1000 << (attempt - 1).min(2),
                    ))
                    .await;
                }
            }
        }
    }

    pub async fn my_account(&self) -> Result<MyAccountResponse, BillingError> {
        let response = self
            .send(|| self.authed(reqwest::Method::GET, "/myAccount"))
            .await?;
        response
            .json()
            .await
            .map_err(|_| BillingError::UpstreamUnavailable)
    }

    pub async fn create_payment(
        &self,
        customer_name: &str,
        billing_type: &str,
        value: f64,
        external_reference: &str,
    ) -> Result<CreatePaymentResponse, BillingError> {
        let body = serde_json::json!({
            "customer": customer_name,
            "billingType": billing_type,
            "value": value,
            "dueDate": chrono::Utc::now().format("%Y-%m-%d").to_string(),
            "externalReference": external_reference,
        });
        let response = self
            .send(|| self.authed(reqwest::Method::POST, "/payments").json(&body))
            .await?;
        response
            .json()
            .await
            .map_err(|_| BillingError::UpstreamUnavailable)
    }

    pub async fn get_balance(&self) -> Result<BalanceResponse, BillingError> {
        let response = self
            .send(|| self.authed(reqwest::Method::GET, "/finance/balance"))
            .await?;
        response
            .json()
            .await
            .map_err(|_| BillingError::UpstreamUnavailable)
    }

    pub async fn list_financial_transactions(
        &self,
        offset: u32,
        limit: u32,
    ) -> Result<FinancialTransactionsResponse, BillingError> {
        let path = format!("/financialTransactions?offset={offset}&limit={limit}");
        let response = self
            .send(|| self.authed(reqwest::Method::GET, &path))
            .await?;
        response
            .json()
            .await
            .map_err(|_| BillingError::UpstreamUnavailable)
    }
}

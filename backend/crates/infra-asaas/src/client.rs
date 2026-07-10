use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use domain_billing::BillingError;
use reqwest::Client;
use serde::Deserialize;
use uuid::Uuid;

use application::billing::{
    AttachPaymentMethodRequest, CancelSubscriptionRequest, CreateCustomerRequest,
    CreateSubscriptionRequest, CustomerResponse, PaymentGateway, SubscriptionResponse,
};

use crate::circuit_breaker::CircuitBreaker;
use crate::config::AsaasConfig;
use crate::error_map::map_status;
use crate::metrics::AsaasMetrics;
use crate::sanitize::mask_api_key;

#[derive(Clone)]
pub struct AsaasClient {
    client: Client,
    config: AsaasConfig,
    circuit: Arc<CircuitBreaker>,
    metrics: Arc<AsaasMetrics>,
}

impl AsaasClient {
    pub fn new(config: AsaasConfig) -> Result<Self, String> {
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(|err| format!("failed to build Asaas HTTP client: {err}"))?;
        tracing::info!(
            base_url = %config.base_url,
            api_key = %mask_api_key(&config.api_key),
            "Asaas client configured"
        );
        let threshold = config.circuit_threshold;
        Ok(Self {
            client,
            circuit: Arc::new(CircuitBreaker::new(threshold)),
            metrics: Arc::new(AsaasMetrics::default()),
            config,
        })
    }

    pub fn metrics(&self) -> Arc<AsaasMetrics> {
        self.metrics.clone()
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.config.base_url, path)
    }

    async fn send_with_retry(
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
                Ok(resp) if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS
                    || resp.status().is_server_error() =>
                {
                    attempt += 1;
                    if attempt > self.config.max_retries {
                        self.circuit.record_failure();
                        self.metrics.record_error(started);
                        let status = resp.status();
                        let body = resp.text().await.unwrap_or_default();
                        return Err(map_status(status, &body));
                    }
                    let backoff = Duration::from_millis(1000 << (attempt - 1).min(2));
                    tokio::time::sleep(backoff).await;
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
                    if attempt > self.config.max_retries {
                        self.circuit.record_failure();
                        self.metrics.record_error(started);
                        return Err(err);
                    }
                    let backoff = Duration::from_millis(1000 << (attempt - 1).min(2));
                    tokio::time::sleep(backoff).await;
                }
            }
        }
    }

    fn authed(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        self.client
            .request(method, self.url(path))
            .header("access_token", &self.config.api_key)
            .header("User-Agent", crate::config::USER_AGENT)
            .header("Content-Type", "application/json")
    }
}

#[derive(Deserialize)]
struct CustomerBody {
    id: String,
}

#[derive(Deserialize)]
struct SubscriptionBody {
    id: String,
}

#[async_trait]
impl PaymentGateway for AsaasClient {
    async fn create_customer(
        &self,
        req: CreateCustomerRequest,
    ) -> Result<CustomerResponse, BillingError> {
        let idempotency = Uuid::now_v7().to_string();
        let body = serde_json::json!({
            "name": req.name,
            "cpfCnpj": req.cpf_cnpj,
            "email": req.email,
            "externalReference": req.external_reference,
        });
        let response = self
            .send_with_retry(|| {
                self.authed(reqwest::Method::POST, "/customers")
                    .header("Idempotency-Key", &idempotency)
                    .json(&body)
            })
            .await?;
        let parsed: CustomerBody = response
            .json()
            .await
            .map_err(|_| BillingError::UpstreamUnavailable)?;
        Ok(CustomerResponse { id: parsed.id })
    }

    async fn get_customer(&self, id: &str) -> Result<CustomerResponse, BillingError> {
        let path = format!("/customers/{id}");
        let response = self
            .send_with_retry(|| self.authed(reqwest::Method::GET, &path))
            .await?;
        let parsed: CustomerBody = response
            .json()
            .await
            .map_err(|_| BillingError::UpstreamUnavailable)?;
        Ok(CustomerResponse { id: parsed.id })
    }

    async fn create_subscription(
        &self,
        req: CreateSubscriptionRequest,
    ) -> Result<SubscriptionResponse, BillingError> {
        let idempotency = Uuid::now_v7().to_string();
        let body = serde_json::json!({
            "customer": req.customer_id,
            "billingType": req.billing_type,
            "value": req.value,
            "cycle": req.cycle,
            "description": req.description,
            "externalReference": req.external_reference,
        });
        let response = self
            .send_with_retry(|| {
                self.authed(reqwest::Method::POST, "/subscriptions")
                    .header("Idempotency-Key", &idempotency)
                    .json(&body)
            })
            .await?;
        let parsed: SubscriptionBody = response
            .json()
            .await
            .map_err(|_| BillingError::UpstreamUnavailable)?;
        Ok(SubscriptionResponse { id: parsed.id })
    }

    async fn cancel_subscription(&self, req: CancelSubscriptionRequest) -> Result<(), BillingError> {
        let path = format!("/subscriptions/{}", req.subscription_id);
        self.send_with_retry(|| self.authed(reqwest::Method::DELETE, &path))
            .await?;
        Ok(())
    }

    async fn attach_payment_method(&self, req: AttachPaymentMethodRequest) -> Result<(), BillingError> {
        let path = format!("/customers/{}", req.customer_id);
        let body = serde_json::json!({ "creditCardToken": req.credit_card_token });
        self.send_with_retry(|| {
            self.authed(reqwest::Method::POST, &path)
                .json(&body)
        })
        .await?;
        Ok(())
    }

    async fn ping(&self) -> Result<(), BillingError> {
        self.send_with_retry(|| self.authed(reqwest::Method::GET, "/customers?limit=1"))
            .await?;
        Ok(())
    }
}

mod circuit_breaker;
mod client;
mod config;
mod error_map;
mod metrics;
mod mock;
mod sanitize;

pub use circuit_breaker::CircuitBreaker;
pub use client::AsaasClient;
pub use config::AsaasConfig;
pub use metrics::AsaasMetrics;
pub use mock::{FailingPaymentGateway, MockPaymentGateway};

use std::sync::Arc;

use application::billing::PaymentGateway;

/// Builds a real Asaas client from environment variables.
pub fn client_from_env() -> Result<Arc<dyn PaymentGateway>, String> {
    let config = AsaasConfig::from_env()?;
    Ok(Arc::new(AsaasClient::new(config)?))
}

/// Dev/test fallback when `ASAAS_API_KEY` is unset.
pub fn mock_gateway() -> Arc<dyn PaymentGateway> {
    Arc::new(MockPaymentGateway)
}

/// Production boot: real client when configured, otherwise mock (local dev only).
pub fn gateway_from_env() -> Arc<dyn PaymentGateway> {
    match client_from_env() {
        Ok(client) => client,
        Err(err) => {
            tracing::warn!(error = %err, "ASAAS_API_KEY not set — using mock payment gateway");
            mock_gateway()
        }
    }
}

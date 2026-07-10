use async_trait::async_trait;
use domain_billing::BillingError;
use uuid::Uuid;

use application::billing::{
    CancelSubscriptionRequest, CreateCustomerRequest, CreateSubscriptionRequest, CustomerResponse,
    PaymentGateway, SubscriptionResponse,
};

#[derive(Debug, Default)]
pub struct MockPaymentGateway;

#[async_trait]
impl PaymentGateway for MockPaymentGateway {
    async fn create_customer(
        &self,
        req: CreateCustomerRequest,
    ) -> Result<CustomerResponse, BillingError> {
        Ok(CustomerResponse {
            id: format!("cus_mock_{}", req.external_reference),
        })
    }

    async fn get_customer(&self, id: &str) -> Result<CustomerResponse, BillingError> {
        if id.starts_with("cus_") {
            Ok(CustomerResponse { id: id.to_owned() })
        } else {
            Err(BillingError::CustomerNotFound)
        }
    }

    async fn create_subscription(
        &self,
        req: CreateSubscriptionRequest,
    ) -> Result<SubscriptionResponse, BillingError> {
        Ok(SubscriptionResponse {
            id: format!("sub_mock_{}", req.external_reference),
        })
    }

    async fn cancel_subscription(&self, req: CancelSubscriptionRequest) -> Result<(), BillingError> {
        if req.subscription_id.starts_with("sub_") {
            Ok(())
        } else {
            Err(BillingError::InvalidRequest("subscription_not_found".into()))
        }
    }

    async fn ping(&self) -> Result<(), BillingError> {
        Ok(())
    }
}

/// Fails create_customer — for dead-letter contract tests.
#[derive(Debug)]
pub struct FailingPaymentGateway;

#[async_trait]
impl PaymentGateway for FailingPaymentGateway {
    async fn create_customer(
        &self,
        _req: CreateCustomerRequest,
    ) -> Result<CustomerResponse, BillingError> {
        Err(BillingError::UpstreamUnavailable)
    }

    async fn get_customer(&self, _id: &str) -> Result<CustomerResponse, BillingError> {
        Err(BillingError::UpstreamUnavailable)
    }

    async fn create_subscription(
        &self,
        _req: CreateSubscriptionRequest,
    ) -> Result<SubscriptionResponse, BillingError> {
        Err(BillingError::UpstreamUnavailable)
    }

    async fn cancel_subscription(&self, _req: CancelSubscriptionRequest) -> Result<(), BillingError> {
        Err(BillingError::UpstreamUnavailable)
    }

    async fn ping(&self) -> Result<(), BillingError> {
        Err(BillingError::UpstreamUnavailable)
    }
}

/// Returns deterministic customer id from tenant UUID string.
pub fn mock_customer_id(tenant_id: &str) -> String {
    format!("cus_mock_{tenant_id}")
}

pub fn idempotency_key() -> String {
    Uuid::now_v7().to_string()
}

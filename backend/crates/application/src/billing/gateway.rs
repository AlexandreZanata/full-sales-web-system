use async_trait::async_trait;
use domain_billing::BillingError;

#[derive(Debug, Clone)]
pub struct CreateCustomerRequest {
    pub name: String,
    pub cpf_cnpj: String,
    pub email: String,
    pub external_reference: String,
}

#[derive(Debug, Clone)]
pub struct CustomerResponse {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct CreateSubscriptionRequest {
    pub customer_id: String,
    pub billing_type: String,
    pub value: f64,
    pub cycle: String,
    pub description: String,
    pub external_reference: String,
}

#[derive(Debug, Clone)]
pub struct SubscriptionResponse {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct CancelSubscriptionRequest {
    pub subscription_id: String,
}

#[derive(Debug, Clone)]
pub struct AttachPaymentMethodRequest {
    pub customer_id: String,
    pub credit_card_token: String,
}

#[async_trait]
pub trait PaymentGateway: Send + Sync {
    async fn create_customer(
        &self,
        req: CreateCustomerRequest,
    ) -> Result<CustomerResponse, BillingError>;

    async fn get_customer(&self, id: &str) -> Result<CustomerResponse, BillingError>;

    async fn create_subscription(
        &self,
        req: CreateSubscriptionRequest,
    ) -> Result<SubscriptionResponse, BillingError>;

    async fn cancel_subscription(&self, req: CancelSubscriptionRequest) -> Result<(), BillingError>;

    async fn attach_payment_method(&self, req: AttachPaymentMethodRequest) -> Result<(), BillingError>;

    async fn ping(&self) -> Result<(), BillingError>;
}

pub fn map_billing_error(err: BillingError) -> String {
    match err {
        BillingError::InvalidCredentials => "invalid_credentials".into(),
        BillingError::CustomerNotFound => "customer_not_found".into(),
        BillingError::SubscriptionInactive => "subscription_inactive".into(),
        BillingError::RateLimited => "rate_limited".into(),
        BillingError::UpstreamUnavailable => "upstream_unavailable".into(),
        BillingError::CircuitOpen => "circuit_open".into(),
        BillingError::InvalidRequest(code) => code,
        BillingError::SubscriptionNotFound => "subscription_not_found".into(),
        BillingError::InvoiceNotFound => "invoice_not_found".into(),
        _ => "billing_error".into(),
    }
}

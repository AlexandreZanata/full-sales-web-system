use chrono::{DateTime, Duration, Utc};
use domain_billing::BillingInterval;
use domain_platform::{Tenant, TenantStatus};
use domain_shared::TenantId;
use uuid::Uuid;

use crate::billing::gateway::{CreateSubscriptionRequest, PaymentGateway};
use crate::AppError;

pub const GRACE_DAYS: i64 = 7;

pub struct ProvisionSubscriptionInput {
    pub tenant_id: TenantId,
    pub plan_id: Uuid,
    pub customer_id: String,
    pub plan_name: String,
    pub price_minor: i64,
    pub billing_interval: BillingInterval,
}

pub struct ProvisionSubscriptionResult {
    pub subscription_id: Uuid,
    pub asaas_subscription_id: String,
}

pub async fn provision_subscription<G: PaymentGateway + ?Sized>(
    gateway: &G,
    input: &ProvisionSubscriptionInput,
) -> Result<ProvisionSubscriptionResult, AppError> {
    let subscription_id = Uuid::now_v7();
    let value_major = input.price_minor as f64 / 100.0;

    let asaas = gateway
        .create_subscription(CreateSubscriptionRequest {
            customer_id: input.customer_id.clone(),
            billing_type: "UNDEFINED".into(),
            value: value_major,
            cycle: input.billing_interval.as_asaas_cycle().into(),
            description: format!("{} plan", input.plan_name),
            external_reference: input.tenant_id.as_uuid().to_string(),
        })
        .await
        .map_err(AppError::Billing)?;

    Ok(ProvisionSubscriptionResult {
        subscription_id,
        asaas_subscription_id: asaas.id,
    })
}

pub fn apply_payment_confirmed(tenant: &mut Tenant) -> Result<(), AppError> {
    tenant.restore_from_payment().map_err(crate::tenants::map_platform_error)
}

pub fn apply_payment_overdue(tenant: &mut Tenant) -> Result<(), AppError> {
    tenant.mark_past_due().map_err(crate::tenants::map_platform_error)
}

pub fn apply_subscription_deleted(tenant: &mut Tenant) -> Result<(), AppError> {
    tenant
        .suspend("subscription_deleted", Utc::now())
        .map_err(crate::tenants::map_platform_error)
}

pub fn grace_expired(past_due_at: DateTime<Utc>, grace_extended_until: Option<DateTime<Utc>>) -> bool {
    let deadline = grace_extended_until.unwrap_or_else(|| past_due_at + Duration::days(GRACE_DAYS));
    Utc::now() >= deadline
}

pub fn should_suspend_for_dunning(
    status: TenantStatus,
    past_due_at: Option<DateTime<Utc>>,
    grace_extended_until: Option<DateTime<Utc>>,
) -> bool {
    status == TenantStatus::PastDue
        && past_due_at.is_some_and(|at| grace_expired(at, grace_extended_until))
}

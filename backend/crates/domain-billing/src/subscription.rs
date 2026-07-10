use chrono::{DateTime, Utc};
use domain_shared::TenantId;
use uuid::Uuid;

use crate::error::BillingError;
use crate::subscription_status::{SubscriptionStatus, can_transition_subscription};

#[derive(Debug, Clone)]
pub struct Subscription {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub plan_id: Uuid,
    pub asaas_subscription_id: Option<String>,
    pub status: SubscriptionStatus,
    pub current_period_end: Option<DateTime<Utc>>,
}

impl Subscription {
    pub fn new_pending(id: Uuid, tenant_id: TenantId, plan_id: Uuid) -> Self {
        Self {
            id,
            tenant_id,
            plan_id,
            asaas_subscription_id: None,
            status: SubscriptionStatus::Pending,
            current_period_end: None,
        }
    }

    pub fn attach_asaas(&mut self, asaas_id: String) {
        self.asaas_subscription_id = Some(asaas_id);
    }

    pub fn activate(&mut self, period_end: DateTime<Utc>) -> Result<(), BillingError> {
        self.transition_to(SubscriptionStatus::Active)?;
        self.current_period_end = Some(period_end);
        Ok(())
    }

    pub fn mark_past_due(&mut self) -> Result<(), BillingError> {
        self.transition_to(SubscriptionStatus::PastDue)
    }

    pub fn restore_from_payment(&mut self, period_end: DateTime<Utc>) -> Result<(), BillingError> {
        self.transition_to(SubscriptionStatus::Active)?;
        self.current_period_end = Some(period_end);
        Ok(())
    }

    pub fn cancel(&mut self) -> Result<(), BillingError> {
        self.transition_to(SubscriptionStatus::Cancelled)
    }

    pub fn expire(&mut self) -> Result<(), BillingError> {
        self.transition_to(SubscriptionStatus::Expired)
    }

    fn transition_to(&mut self, target: SubscriptionStatus) -> Result<(), BillingError> {
        if !can_transition_subscription(self.status, target) {
            return Err(BillingError::InvalidSubscriptionTransition {
                from: self.status.as_str().to_owned(),
                to: target.as_str().to_owned(),
            });
        }
        self.status = target;
        Ok(())
    }
}

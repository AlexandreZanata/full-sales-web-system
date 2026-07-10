use chrono::{DateTime, Utc};
use domain_shared::TenantId;
use uuid::Uuid;

use crate::error::PlatformError;
use crate::tenant_status::{TenantStatus, can_transition};

#[derive(Debug, Clone)]
pub struct Tenant {
    pub id: TenantId,
    pub legal_name: String,
    pub display_name: String,
    pub status: TenantStatus,
    pub plan_id: Option<Uuid>,
    pub trial_ends_at: Option<DateTime<Utc>>,
    pub suspended_at: Option<DateTime<Utc>>,
    pub suspended_reason: Option<String>,
    pub offboarding_scheduled_at: Option<DateTime<Utc>>,
    pub settings: serde_json::Value,
}

impl Tenant {
    pub fn new_provisioning(
        id: TenantId,
        legal_name: String,
        display_name: String,
    ) -> Result<Self, PlatformError> {
        if legal_name.trim().len() < 2 {
            return Err(PlatformError::InvalidLegalName);
        }
        Ok(Self {
            id,
            legal_name: legal_name.trim().to_owned(),
            display_name: display_name.trim().to_owned(),
            status: TenantStatus::Provisioning,
            plan_id: None,
            trial_ends_at: None,
            suspended_at: None,
            suspended_reason: None,
            offboarding_scheduled_at: None,
            settings: serde_json::json!({}),
        })
    }

    pub fn activate_trial(
        &mut self,
        plan_id: Uuid,
        trial_ends_at: DateTime<Utc>,
    ) -> Result<(), PlatformError> {
        self.transition_to(TenantStatus::Trial)?;
        self.plan_id = Some(plan_id);
        self.trial_ends_at = Some(trial_ends_at);
        Ok(())
    }

    pub fn activate_paid(&mut self, plan_id: Uuid) -> Result<(), PlatformError> {
        self.transition_to(TenantStatus::Active)?;
        self.plan_id = Some(plan_id);
        self.trial_ends_at = None;
        Ok(())
    }

    pub fn suspend(&mut self, reason: &str, at: DateTime<Utc>) -> Result<(), PlatformError> {
        let trimmed = reason.trim();
        if trimmed.is_empty() {
            return Err(PlatformError::SuspendReasonRequired);
        }
        if trimmed.len() < 3 {
            return Err(PlatformError::SuspendReasonTooShort);
        }
        self.transition_to(TenantStatus::Suspended)?;
        self.suspended_at = Some(at);
        self.suspended_reason = Some(trimmed.to_owned());
        Ok(())
    }

    pub fn reactivate(&mut self) -> Result<(), PlatformError> {
        self.transition_to(TenantStatus::Active)?;
        self.suspended_at = None;
        self.suspended_reason = None;
        Ok(())
    }

    pub fn begin_offboarding(&mut self, at: DateTime<Utc>) -> Result<(), PlatformError> {
        self.transition_to(TenantStatus::Offboarding)?;
        self.offboarding_scheduled_at = Some(at);
        Ok(())
    }

    pub fn mark_deleted(&mut self) -> Result<(), PlatformError> {
        self.transition_to(TenantStatus::Deleted)
    }

    pub fn transition_to(&mut self, target: TenantStatus) -> Result<(), PlatformError> {
        if !can_transition(self.status, target) {
            return Err(PlatformError::InvalidTenantTransition {
                from: self.status.as_str().to_owned(),
                to: target.as_str().to_owned(),
            });
        }
        self.status = target;
        Ok(())
    }

    pub fn ensure_mutations_allowed(&self) -> Result<(), PlatformError> {
        if self.status.allows_mutations() {
            Ok(())
        } else {
            Err(PlatformError::TenantMutationsBlocked(
                self.status.as_str().to_owned(),
            ))
        }
    }
}

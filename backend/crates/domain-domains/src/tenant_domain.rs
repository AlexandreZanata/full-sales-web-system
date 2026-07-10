use chrono::{DateTime, Utc};
use domain_shared::TenantId;
use uuid::Uuid;

use crate::domain_status::{DomainStatus, can_transition};
use crate::error::DomainError;
use crate::hostname::normalize_hostname;

#[derive(Debug, Clone)]
pub struct TenantDomain {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub hostname: String,
    pub status: DomainStatus,
    pub verification_token: String,
    pub verified_at: Option<DateTime<Utc>>,
    pub is_primary: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TenantDomain {
    pub fn add(
        id: Uuid,
        tenant_id: TenantId,
        raw_hostname: &str,
        verification_token: String,
        now: DateTime<Utc>,
    ) -> Result<Self, DomainError> {
        let hostname = normalize_hostname(raw_hostname)?;
        if verification_token.trim().len() < 16 {
            return Err(DomainError::InvalidHostname);
        }
        Ok(Self {
            id,
            tenant_id,
            hostname,
            status: DomainStatus::Pending,
            verification_token,
            verified_at: None,
            is_primary: false,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn start_verifying(&mut self, now: DateTime<Utc>) -> Result<(), DomainError> {
        self.transition(DomainStatus::Verifying)?;
        self.updated_at = now;
        Ok(())
    }

    pub fn mark_verified(&mut self, now: DateTime<Utc>) -> Result<(), DomainError> {
        self.transition(DomainStatus::Verified)?;
        self.verified_at = Some(now);
        self.updated_at = now;
        Ok(())
    }

    pub fn activate(&mut self, now: DateTime<Utc>) -> Result<(), DomainError> {
        if self.status != DomainStatus::Verified {
            return Err(DomainError::NotVerified);
        }
        self.transition(DomainStatus::Active)?;
        self.updated_at = now;
        Ok(())
    }

    pub fn mark_failed(&mut self, now: DateTime<Utc>) -> Result<(), DomainError> {
        self.transition(DomainStatus::Failed)?;
        self.updated_at = now;
        Ok(())
    }

    pub fn detach(&mut self, now: DateTime<Utc>) -> Result<(), DomainError> {
        if self.status == DomainStatus::Detached {
            return Ok(());
        }
        self.transition(DomainStatus::Detached)?;
        self.is_primary = false;
        self.updated_at = now;
        Ok(())
    }

    pub fn retry_verification(&mut self, now: DateTime<Utc>) -> Result<(), DomainError> {
        self.transition(DomainStatus::Verifying)?;
        self.updated_at = now;
        Ok(())
    }

    pub fn set_primary(&mut self, now: DateTime<Utc>) -> Result<(), DomainError> {
        if !matches!(self.status, DomainStatus::Verified | DomainStatus::Active) {
            return Err(DomainError::CannotSetPrimary);
        }
        if self.status == DomainStatus::Verified {
            self.activate(now)?;
        }
        self.is_primary = true;
        self.updated_at = now;
        Ok(())
    }

    pub fn clear_primary(&mut self, now: DateTime<Utc>) {
        self.is_primary = false;
        self.updated_at = now;
    }

    fn transition(&mut self, to: DomainStatus) -> Result<(), DomainError> {
        if !can_transition(self.status, to) {
            return Err(DomainError::InvalidTransition {
                from: self.status.as_str().into(),
                to: to.as_str().into(),
            });
        }
        self.status = to;
        Ok(())
    }
}

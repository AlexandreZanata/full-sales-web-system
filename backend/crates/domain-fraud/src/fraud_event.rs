use chrono::{DateTime, Utc};
use domain_shared::TenantId;
use uuid::Uuid;

use crate::error::FraudError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FraudEventType {
    LoginVelocity,
    PaymentVelocity,
    ProvisionVelocity,
    WebhookFailureBurst,
    DuplicateCard,
    AmountAnomaly,
    Chargeback,
    BlocklistHit,
}

impl FraudEventType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LoginVelocity => "LoginVelocity",
            Self::PaymentVelocity => "PaymentVelocity",
            Self::ProvisionVelocity => "ProvisionVelocity",
            Self::WebhookFailureBurst => "WebhookFailureBurst",
            Self::DuplicateCard => "DuplicateCard",
            Self::AmountAnomaly => "AmountAnomaly",
            Self::Chargeback => "Chargeback",
            Self::BlocklistHit => "BlocklistHit",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FraudSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl FraudSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Critical => "Critical",
        }
    }

    pub fn score_delta(self) -> i32 {
        match self {
            Self::Low => 10,
            Self::Medium => 25,
            Self::High => 50,
            Self::Critical => 100,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FraudEventStatus {
    Open,
    Reviewed,
    Blocked,
}

impl FraudEventStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "Open",
            Self::Reviewed => "Reviewed",
            Self::Blocked => "Blocked",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FraudResolution {
    Blocked,
    Whitelisted,
    Dismissed,
}

impl FraudResolution {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "blocked",
            Self::Whitelisted => "whitelisted",
            Self::Dismissed => "dismissed",
        }
    }

    pub fn parse(value: &str) -> Result<Self, FraudError> {
        match value {
            "blocked" => Ok(Self::Blocked),
            "whitelisted" => Ok(Self::Whitelisted),
            "dismissed" => Ok(Self::Dismissed),
            _ => Err(FraudError::InvalidEventStatus),
        }
    }

    pub fn resulting_status(self) -> FraudEventStatus {
        match self {
            Self::Blocked => FraudEventStatus::Blocked,
            Self::Whitelisted | Self::Dismissed => FraudEventStatus::Reviewed,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FraudEvent {
    pub id: Uuid,
    pub tenant_id: Option<TenantId>,
    pub user_id: Option<Uuid>,
    pub event_type: FraudEventType,
    pub severity: FraudSeverity,
    pub status: FraudEventStatus,
    pub resolution: Option<FraudResolution>,
    pub resolution_note: Option<String>,
    pub metadata: serde_json::Value,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl FraudEvent {
    pub fn new_open(
        id: Uuid,
        tenant_id: Option<TenantId>,
        user_id: Option<Uuid>,
        event_type: FraudEventType,
        severity: FraudSeverity,
        metadata: serde_json::Value,
    ) -> Self {
        Self {
            id,
            tenant_id,
            user_id,
            event_type,
            severity,
            status: FraudEventStatus::Open,
            resolution: None,
            resolution_note: None,
            metadata,
            reviewed_by: None,
            reviewed_at: None,
            created_at: Utc::now(),
        }
    }

    pub fn resolve(
        mut self,
        reviewer_id: Uuid,
        resolution: FraudResolution,
        note: Option<String>,
    ) -> Result<Self, FraudError> {
        if self.status != FraudEventStatus::Open {
            return Err(FraudError::InvalidEventTransition {
                from: self.status.as_str().into(),
                to: resolution.resulting_status().as_str().into(),
            });
        }
        self.status = resolution.resulting_status();
        self.resolution = Some(resolution);
        self.resolution_note = note.filter(|n| !n.trim().is_empty());
        self.reviewed_by = Some(reviewer_id);
        self.reviewed_at = Some(Utc::now());
        Ok(self)
    }
}

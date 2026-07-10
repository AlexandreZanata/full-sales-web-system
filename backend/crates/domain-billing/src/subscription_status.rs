use serde::{Deserialize, Serialize};

use crate::error::BillingError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SubscriptionStatus {
    Pending,
    Active,
    PastDue,
    Cancelled,
    Expired,
}

impl SubscriptionStatus {
    pub fn parse(raw: &str) -> Result<Self, BillingError> {
        match raw {
            "Pending" => Ok(Self::Pending),
            "Active" => Ok(Self::Active),
            "PastDue" => Ok(Self::PastDue),
            "Cancelled" => Ok(Self::Cancelled),
            "Expired" => Ok(Self::Expired),
            _ => Err(BillingError::InvalidSubscriptionStatus),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "Pending",
            Self::Active => "Active",
            Self::PastDue => "PastDue",
            Self::Cancelled => "Cancelled",
            Self::Expired => "Expired",
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Expired)
    }
}

pub fn can_transition_subscription(from: SubscriptionStatus, to: SubscriptionStatus) -> bool {
    if from == to {
        return true;
    }
    use SubscriptionStatus::*;
    match (from, to) {
        (Pending, Active) => true,
        (Active, PastDue) | (Active, Cancelled) => true,
        (PastDue, Active) | (PastDue, Cancelled) => true,
        (Cancelled, Expired) => true,
        (Expired, _) => false,
        _ => false,
    }
}

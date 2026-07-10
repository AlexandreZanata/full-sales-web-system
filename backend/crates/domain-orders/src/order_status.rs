use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::OrderError;

/// Lifecycle state of an Order aggregate (GLOSSARY: Order).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OrderStatus {
    Draft,
    AwaitingPayment,
    Paid,
    PendingApproval,
    Approved,
    Rejected,
    Picking,
    InTransit,
    Delivered,
    PartiallyDelivered,
    Cancelled,
}

impl OrderStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "Draft",
            Self::AwaitingPayment => "AwaitingPayment",
            Self::Paid => "Paid",
            Self::PendingApproval => "PendingApproval",
            Self::Approved => "Approved",
            Self::Rejected => "Rejected",
            Self::Picking => "Picking",
            Self::InTransit => "InTransit",
            Self::Delivered => "Delivered",
            Self::PartiallyDelivered => "PartiallyDelivered",
            Self::Cancelled => "Cancelled",
        }
    }

    pub fn allows_item_changes(self) -> bool {
        self == Self::Draft
    }

    pub fn can_cancel(self) -> bool {
        matches!(
            self,
            Self::Draft | Self::PendingApproval | Self::Paid | Self::Approved | Self::Picking
        )
    }

    pub fn had_active_reservations(self) -> bool {
        matches!(self, Self::Approved | Self::Picking)
    }
}

impl FromStr for OrderStatus {
    type Err = OrderError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Draft" => Ok(Self::Draft),
            "AwaitingPayment" => Ok(Self::AwaitingPayment),
            "Paid" => Ok(Self::Paid),
            "PendingApproval" => Ok(Self::PendingApproval),
            "Approved" => Ok(Self::Approved),
            "Rejected" => Ok(Self::Rejected),
            "Picking" => Ok(Self::Picking),
            "InTransit" => Ok(Self::InTransit),
            "Delivered" => Ok(Self::Delivered),
            "PartiallyDelivered" => Ok(Self::PartiallyDelivered),
            "Cancelled" => Ok(Self::Cancelled),
            _ => Err(OrderError::InvalidOrderStatus),
        }
    }
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

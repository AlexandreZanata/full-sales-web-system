use serde::{Deserialize, Serialize};

use crate::error::BillingError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum InvoiceStatus {
    Pending,
    Open,
    Paid,
    Overdue,
    Cancelled,
    Refunded,
}

impl InvoiceStatus {
    pub fn parse(raw: &str) -> Result<Self, BillingError> {
        match raw {
            "Pending" => Ok(Self::Pending),
            "Open" => Ok(Self::Open),
            "Paid" => Ok(Self::Paid),
            "Overdue" => Ok(Self::Overdue),
            "Cancelled" => Ok(Self::Cancelled),
            "Refunded" => Ok(Self::Refunded),
            _ => Err(BillingError::InvalidInvoiceStatus),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "Pending",
            Self::Open => "Open",
            Self::Paid => "Paid",
            Self::Overdue => "Overdue",
            Self::Cancelled => "Cancelled",
            Self::Refunded => "Refunded",
        }
    }
}

pub fn can_transition_invoice(from: InvoiceStatus, to: InvoiceStatus) -> bool {
    if from == to {
        return true;
    }
    use InvoiceStatus::*;
    match (from, to) {
        (Pending, Open) | (Pending, Cancelled) => true,
        (Open, Paid) | (Open, Overdue) | (Open, Cancelled) => true,
        (Overdue, Paid) | (Overdue, Cancelled) => true,
        (Paid, Refunded) => true,
        (Cancelled, _) | (Refunded, _) => false,
        _ => false,
    }
}

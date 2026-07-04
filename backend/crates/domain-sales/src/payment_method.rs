use serde::{Deserialize, Serialize};

use crate::error::SaleError;

/// How a sale was paid — recorded only, no gateway capture (ADR-006).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PaymentMethod {
    Cash,
    Pix,
    Credit,
    Debit,
}

impl PaymentMethod {
    pub fn parse(value: &str) -> Result<Self, SaleError> {
        match value {
            "Cash" | "cash" => Ok(Self::Cash),
            "Pix" | "pix" => Ok(Self::Pix),
            "Credit" | "credit" => Ok(Self::Credit),
            "Debit" | "debit" => Ok(Self::Debit),
            _ => Err(SaleError::InvalidPaymentMethod),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cash => "Cash",
            Self::Pix => "Pix",
            Self::Credit => "Credit",
            Self::Debit => "Debit",
        }
    }
}

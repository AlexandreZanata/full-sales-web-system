use serde::{Deserialize, Serialize};

use crate::error::BillingError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum BillingInterval {
    Monthly,
}

impl BillingInterval {
    pub fn parse(raw: &str) -> Result<Self, BillingError> {
        match raw {
            "Monthly" => Ok(Self::Monthly),
            _ => Err(BillingError::InvalidBillingInterval),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Monthly => "Monthly",
        }
    }

    pub fn as_asaas_cycle(self) -> &'static str {
        match self {
            Self::Monthly => "MONTHLY",
        }
    }
}

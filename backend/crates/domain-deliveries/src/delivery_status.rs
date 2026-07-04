use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::DeliveryError;

/// Lifecycle state of a Delivery aggregate (GLOSSARY: Delivery).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DeliveryStatus {
    Waiting,
    InTransit,
    Delivered,
    Failed,
}

impl DeliveryStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Waiting => "Waiting",
            Self::InTransit => "InTransit",
            Self::Delivered => "Delivered",
            Self::Failed => "Failed",
        }
    }
}

impl FromStr for DeliveryStatus {
    type Err = DeliveryError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Waiting" => Ok(Self::Waiting),
            "InTransit" => Ok(Self::InTransit),
            "Delivered" => Ok(Self::Delivered),
            "Failed" => Ok(Self::Failed),
            _ => Err(DeliveryError::InvalidDeliveryStatus),
        }
    }
}

impl fmt::Display for DeliveryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

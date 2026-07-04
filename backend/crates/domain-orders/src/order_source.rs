use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::OrderError;

/// How the order was created — portal or seller visit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OrderSource {
    CommercePortal,
    SellerVisit,
}

impl OrderSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CommercePortal => "CommercePortal",
            Self::SellerVisit => "SellerVisit",
        }
    }
}

impl FromStr for OrderSource {
    type Err = OrderError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "CommercePortal" => Ok(Self::CommercePortal),
            "SellerVisit" => Ok(Self::SellerVisit),
            _ => Err(OrderError::InvalidOrderSource),
        }
    }
}

impl fmt::Display for OrderSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

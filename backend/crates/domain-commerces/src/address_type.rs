use std::fmt;
use std::str::FromStr;

use crate::error::CommerceError;

/// Billing or delivery address classification for a Commerce.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddressType {
    Billing,
    Delivery,
}

impl AddressType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Billing => "Billing",
            Self::Delivery => "Delivery",
        }
    }
}

impl FromStr for AddressType {
    type Err = CommerceError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Billing" => Ok(Self::Billing),
            "Delivery" => Ok(Self::Delivery),
            _ => Err(CommerceError::InvalidAddressType),
        }
    }
}

impl fmt::Display for AddressType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

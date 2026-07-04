use std::fmt;
use std::str::FromStr;

use crate::error::InventoryError;

/// Lifecycle state of a stock reservation (RN2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReservationStatus {
    Active,
    Released,
    Consumed,
}

impl ReservationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Released => "Released",
            Self::Consumed => "Consumed",
        }
    }
}

impl FromStr for ReservationStatus {
    type Err = InventoryError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Active" => Ok(Self::Active),
            "Released" => Ok(Self::Released),
            "Consumed" => Ok(Self::Consumed),
            _ => Err(InventoryError::InvalidReservationStatus),
        }
    }
}

impl fmt::Display for ReservationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

use std::fmt;
use std::str::FromStr;

use crate::error::InventoryError;

/// Product unit of measure for catalog display and order lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum UnitOfMeasure {
    #[default]
    Unit,
    Kg,
    Box,
    Liter,
}

impl UnitOfMeasure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unit => "Unit",
            Self::Kg => "Kg",
            Self::Box => "Box",
            Self::Liter => "Liter",
        }
    }
}

impl FromStr for UnitOfMeasure {
    type Err = InventoryError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Unit" => Ok(Self::Unit),
            "Kg" => Ok(Self::Kg),
            "Box" => Ok(Self::Box),
            "Liter" => Ok(Self::Liter),
            _ => Err(InventoryError::InvalidUnitOfMeasure),
        }
    }
}

impl fmt::Display for UnitOfMeasure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

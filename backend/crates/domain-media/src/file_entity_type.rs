use std::fmt;
use std::str::FromStr;

use crate::error::MediaError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileEntityType {
    Product,
    User,
    Commerce,
    Delivery,
    Tenant,
}

impl FileEntityType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Product => "Product",
            Self::User => "User",
            Self::Commerce => "Commerce",
            Self::Delivery => "Delivery",
            Self::Tenant => "Tenant",
        }
    }
}

impl fmt::Display for FileEntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for FileEntityType {
    type Err = MediaError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Product" => Ok(Self::Product),
            "User" => Ok(Self::User),
            "Commerce" => Ok(Self::Commerce),
            "Delivery" => Ok(Self::Delivery),
            "Tenant" => Ok(Self::Tenant),
            _ => Err(MediaError::InvalidEntityType),
        }
    }
}

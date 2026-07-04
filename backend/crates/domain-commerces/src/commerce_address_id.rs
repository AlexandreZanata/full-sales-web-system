use std::fmt;
use std::str::FromStr;

use uuid::Uuid;

use crate::error::CommerceError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommerceAddressId(Uuid);

impl CommerceAddressId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl FromStr for CommerceAddressId {
    type Err = CommerceError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(value)
            .map(Self)
            .map_err(|_| CommerceError::InvalidCommerceAddressId)
    }
}

impl fmt::Display for CommerceAddressId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

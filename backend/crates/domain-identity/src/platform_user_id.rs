use std::str::FromStr;

use uuid::Uuid;

use crate::error::IdentityError;

/// Platform operator identifier — distinct from tenant `UserId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlatformUserId(Uuid);

impl PlatformUserId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl FromStr for PlatformUserId {
    type Err = IdentityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s)
            .map(Self)
            .map_err(|_| IdentityError::InvalidPlatformUserId)
    }
}

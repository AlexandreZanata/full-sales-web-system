use serde::{Deserialize, Serialize};

use crate::error::PlatformError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TenantStatus {
    Provisioning,
    Trial,
    Active,
    PastDue,
    Suspended,
    Offboarding,
    Deleted,
}

impl TenantStatus {
    pub fn parse(raw: &str) -> Result<Self, PlatformError> {
        match raw {
            "Provisioning" => Ok(Self::Provisioning),
            "Trial" => Ok(Self::Trial),
            "Active" => Ok(Self::Active),
            "PastDue" => Ok(Self::PastDue),
            "Suspended" => Ok(Self::Suspended),
            "Offboarding" => Ok(Self::Offboarding),
            "Deleted" => Ok(Self::Deleted),
            _ => Err(PlatformError::InvalidTenantStatus),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Provisioning => "Provisioning",
            Self::Trial => "Trial",
            Self::Active => "Active",
            Self::PastDue => "PastDue",
            Self::Suspended => "Suspended",
            Self::Offboarding => "Offboarding",
            Self::Deleted => "Deleted",
        }
    }

    pub fn allows_mutations(self) -> bool {
        matches!(self, Self::Trial | Self::Active | Self::PastDue)
    }

    pub fn allows_read(self) -> bool {
        !matches!(self, Self::Deleted)
    }
}

pub fn can_transition(from: TenantStatus, to: TenantStatus) -> bool {
    use TenantStatus::*;
    if from == to {
        return true;
    }
    match (from, to) {
        (Provisioning, Trial) | (Provisioning, Active) => true,
        (Trial, Active) | (Trial, Suspended) => true,
        (Active, PastDue) | (Active, Suspended) | (Active, Offboarding) => true,
        (PastDue, Active) | (PastDue, Suspended) | (PastDue, Offboarding) => true,
        (Suspended, Active) | (Suspended, Offboarding) => true,
        (Offboarding, Deleted) => true,
        (Deleted, _) => false,
        _ => false,
    }
}

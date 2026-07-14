use crate::error::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainStatus {
    Pending,
    Verifying,
    Verified,
    Active,
    Failed,
    Detached,
}

impl DomainStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "Pending",
            Self::Verifying => "Verifying",
            Self::Verified => "Verified",
            Self::Active => "Active",
            Self::Failed => "Failed",
            Self::Detached => "Detached",
        }
    }

    pub fn parse(value: &str) -> Result<Self, DomainError> {
        match value {
            "Pending" => Ok(Self::Pending),
            "Verifying" => Ok(Self::Verifying),
            "Verified" => Ok(Self::Verified),
            "Active" => Ok(Self::Active),
            "Failed" => Ok(Self::Failed),
            "Detached" => Ok(Self::Detached),
            _ => Err(DomainError::InvalidTransition {
                from: value.into(),
                to: "unknown".into(),
            }),
        }
    }

    pub fn allows_routing(self) -> bool {
        matches!(self, Self::Active)
    }
}

pub fn can_transition(from: DomainStatus, to: DomainStatus) -> bool {
    matches!(
        (from, to),
        (DomainStatus::Pending, DomainStatus::Verifying)
            | (DomainStatus::Verifying, DomainStatus::Verified)
            | (DomainStatus::Verifying, DomainStatus::Failed)
            | (DomainStatus::Verifying, DomainStatus::Detached)
            | (DomainStatus::Verified, DomainStatus::Active)
            | (DomainStatus::Active, DomainStatus::Detached)
            | (DomainStatus::Failed, DomainStatus::Verifying)
            | (DomainStatus::Pending, DomainStatus::Detached)
            | (DomainStatus::Verified, DomainStatus::Detached)
            | (DomainStatus::Failed, DomainStatus::Detached)
    )
}

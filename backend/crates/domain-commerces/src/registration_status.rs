use crate::error::CommerceError;

/// Commerce registration lifecycle (Phase 69).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistrationStatus {
    Active,
    PendingReview,
    Rejected,
}

impl RegistrationStatus {
    pub fn parse(value: &str) -> Result<Self, CommerceError> {
        match value {
            "Active" => Ok(Self::Active),
            "PendingReview" => Ok(Self::PendingReview),
            "Rejected" => Ok(Self::Rejected),
            _ => Err(CommerceError::InvalidRegistrationStatus),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::PendingReview => "PendingReview",
            Self::Rejected => "Rejected",
        }
    }
}

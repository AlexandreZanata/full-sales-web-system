use crate::error::CommerceError;

/// How the seller submitted the registration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistrationMode {
    CnpjLookup,
    Manual,
}

impl RegistrationMode {
    pub fn parse(value: &str) -> Result<Self, CommerceError> {
        match value {
            "cnpj_lookup" => Ok(Self::CnpjLookup),
            "manual" => Ok(Self::Manual),
            _ => Err(CommerceError::InvalidRegistrationMode),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CnpjLookup => "cnpj_lookup",
            Self::Manual => "manual",
        }
    }
}

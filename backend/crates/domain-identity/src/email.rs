use std::fmt;

use crate::error::IdentityError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn parse(value: &str) -> Result<Self, IdentityError> {
        let trimmed = value.trim().to_lowercase();
        if !trimmed.contains('@') || trimmed.starts_with('@') || trimmed.ends_with('@') {
            return Err(IdentityError::InvalidEmail);
        }
        Ok(Self(trimmed))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

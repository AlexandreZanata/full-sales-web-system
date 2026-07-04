use std::fmt;

use crate::error::IdentityError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FullName(String);

impl FullName {
    pub fn parse(value: &str) -> Result<Self, IdentityError> {
        let trimmed = value.trim();
        let parts: Vec<&str> = trimmed
            .split_whitespace()
            .filter(|p| !p.is_empty())
            .collect();
        if parts.len() < 2 || parts.iter().any(|p| p.len() < 2) {
            return Err(IdentityError::InvalidFullName);
        }
        Ok(Self(trimmed.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FullName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

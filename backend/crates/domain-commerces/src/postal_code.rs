use std::fmt;

use crate::error::CommerceError;

/// Brazilian postal code (CEP) — eight digits, normalized without punctuation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PostalCode(String);

impl PostalCode {
    pub fn parse(raw: &str) -> Result<Self, CommerceError> {
        let digits: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
        if digits.len() != 8 {
            return Err(CommerceError::InvalidPostalCode);
        }
        Ok(Self(digits))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PostalCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_valid_cep_when_parse_then_normalized() {
        let cep = PostalCode::parse("01310-100").expect("valid cep");
        assert_eq!(cep.as_str(), "01310100");
    }

    #[test]
    fn given_short_cep_when_parse_then_invalid() {
        assert_eq!(
            PostalCode::parse("01310"),
            Err(CommerceError::InvalidPostalCode)
        );
    }
}

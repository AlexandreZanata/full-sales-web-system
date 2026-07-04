use std::fmt;

use crate::error::CommerceError;

const VALID_STATES: [&str; 27] = [
    "AC", "AL", "AP", "AM", "BA", "CE", "DF", "ES", "GO", "MA", "MT", "MS", "MG", "PA", "PB", "PR",
    "PE", "PI", "RJ", "RN", "RS", "RO", "RR", "SC", "SP", "SE", "TO",
];

/// Brazilian federative unit (UF) — two-letter code.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BrazilianState(String);

impl BrazilianState {
    pub fn parse(raw: &str) -> Result<Self, CommerceError> {
        let code = raw.trim().to_ascii_uppercase();
        if code.len() != 2 || !VALID_STATES.contains(&code.as_str()) {
            return Err(CommerceError::InvalidState);
        }
        Ok(Self(code))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for BrazilianState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_valid_uf_when_parse_then_uppercase() {
        let state = BrazilianState::parse("sp").expect("valid state");
        assert_eq!(state.as_str(), "SP");
    }

    #[test]
    fn given_invalid_uf_when_parse_then_rejected() {
        assert_eq!(
            BrazilianState::parse("XX"),
            Err(CommerceError::InvalidState)
        );
    }
}

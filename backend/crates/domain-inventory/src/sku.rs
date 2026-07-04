use crate::error::InventoryError;

/// Stock-keeping unit identifier for a Product (GLOSSARY: Sku).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sku(String);

impl Sku {
    pub fn parse(value: &str) -> Result<Self, InventoryError> {
        let trimmed = value.trim();
        if trimmed.is_empty()
            || !trimmed
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
        {
            return Err(InventoryError::InvalidSku);
        }
        Ok(Self(trimmed.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

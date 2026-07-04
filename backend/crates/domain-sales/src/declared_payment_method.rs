use serde::{Deserialize, Serialize};

use crate::error::SaleError;

/// Seller-declared payment method — unverified off-platform assertion (RN-PAG1–RN-PAG4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DeclaredPaymentMethod {
    Cash,
    Pix,
    Card,
    Boleto,
    Other,
    NotDeclared,
}

impl DeclaredPaymentMethod {
    pub fn parse(value: &str) -> Result<Self, SaleError> {
        match value {
            "Cash" | "cash" => Ok(Self::Cash),
            "Pix" | "pix" => Ok(Self::Pix),
            "Card" | "card" => Ok(Self::Card),
            "Boleto" | "boleto" => Ok(Self::Boleto),
            "Other" | "other" => Ok(Self::Other),
            "NotDeclared" | "not_declared" => Ok(Self::NotDeclared),
            _ => Err(SaleError::InvalidDeclaredPaymentMethod),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cash => "Cash",
            Self::Pix => "Pix",
            Self::Card => "Card",
            Self::Boleto => "Boleto",
            Self::Other => "Other",
            Self::NotDeclared => "NotDeclared",
        }
    }
}

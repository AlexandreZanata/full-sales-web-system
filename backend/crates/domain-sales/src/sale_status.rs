use std::fmt;

use serde::{Deserialize, Serialize};

/// Lifecycle state of a Sale aggregate (GLOSSARY: SaleStatus).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SaleStatus {
    Pending,
    Confirmed,
    Cancelled,
}

impl SaleStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "Pending",
            Self::Confirmed => "Confirmed",
            Self::Cancelled => "Cancelled",
        }
    }
}

impl fmt::Display for SaleStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

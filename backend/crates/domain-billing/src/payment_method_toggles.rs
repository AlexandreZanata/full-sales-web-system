use serde::{Deserialize, Serialize};

/// Tenant-configurable online payment methods (PIX, credit, boleto).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentMethodToggles {
    pub pix: bool,
    pub credit: bool,
    pub boleto: bool,
}

impl PaymentMethodToggles {
    pub fn all_enabled() -> Self {
        Self {
            pix: true,
            credit: true,
            boleto: true,
        }
    }

    pub fn any_enabled(self) -> bool {
        self.pix || self.credit || self.boleto
    }
}

impl Default for PaymentMethodToggles {
    fn default() -> Self {
        Self {
            pix: true,
            credit: true,
            boleto: false,
        }
    }
}

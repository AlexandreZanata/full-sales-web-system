use chrono::{DateTime, Utc};

use domain_identity::UserId;

use crate::declared_payment_method::DeclaredPaymentMethod;

/// Off-platform payment assertion on a Sale (RN-PAG1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclaredPayment {
    method: DeclaredPaymentMethod,
    received: bool,
    declared_at: Option<DateTime<Utc>>,
    declared_by: Option<UserId>,
    notes: Option<String>,
}

impl DeclaredPayment {
    pub fn not_declared() -> Self {
        Self {
            method: DeclaredPaymentMethod::NotDeclared,
            received: false,
            declared_at: None,
            declared_by: None,
            notes: None,
        }
    }

    pub fn method(&self) -> DeclaredPaymentMethod {
        self.method
    }

    pub fn received(&self) -> bool {
        self.received
    }

    pub fn declared_at(&self) -> Option<DateTime<Utc>> {
        self.declared_at
    }

    pub fn declared_by(&self) -> Option<UserId> {
        self.declared_by
    }

    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    pub(crate) fn apply(
        method: DeclaredPaymentMethod,
        received: bool,
        declared_at: DateTime<Utc>,
        declared_by: UserId,
        notes: Option<String>,
    ) -> Self {
        Self {
            method,
            received,
            declared_at: Some(declared_at),
            declared_by: Some(declared_by),
            notes: notes.filter(|n| !n.trim().is_empty()),
        }
    }
}

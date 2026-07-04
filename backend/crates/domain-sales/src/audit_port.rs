use domain_identity::UserId;

use crate::declared_payment::DeclaredPayment;
use crate::error::SaleError;
use crate::sale_id::SaleId;

/// Audit entry when declared payment changes (RN-PAG3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentDeclarationAuditEntry {
    pub sale_id: SaleId,
    pub actor_id: UserId,
    pub previous: DeclaredPayment,
    pub current: DeclaredPayment,
}

/// Port invoked on every declared-payment change — no silent updates (RN-PAG3).
pub trait PaymentDeclarationAuditPort {
    fn record_change(&mut self, entry: PaymentDeclarationAuditEntry) -> Result<(), SaleError>;
}

/// In-memory audit log for domain and application tests.
#[derive(Debug, Default)]
pub struct InMemoryPaymentDeclarationAuditPort {
    pub entries: Vec<PaymentDeclarationAuditEntry>,
}

impl PaymentDeclarationAuditPort for InMemoryPaymentDeclarationAuditPort {
    fn record_change(&mut self, entry: PaymentDeclarationAuditEntry) -> Result<(), SaleError> {
        self.entries.push(entry);
        Ok(())
    }
}

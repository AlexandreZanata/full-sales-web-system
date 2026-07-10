use chrono::{DateTime, Utc};
use domain_shared::{Currency, Money, TenantId};
use uuid::Uuid;

use crate::error::BillingError;
use crate::invoice_status::{InvoiceStatus, can_transition_invoice};

#[derive(Debug, Clone)]
pub struct Invoice {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub subscription_id: Uuid,
    pub amount: Money,
    pub due_date: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
    pub status: InvoiceStatus,
    pub asaas_payment_id: Option<String>,
    pub pdf_url: Option<String>,
}

impl Invoice {
    pub fn new_open(
        id: Uuid,
        tenant_id: TenantId,
        subscription_id: Uuid,
        amount_minor: i64,
        due_date: DateTime<Utc>,
        asaas_payment_id: String,
    ) -> Result<Self, BillingError> {
        let amount = Money::new(amount_minor, Currency::brl())
            .map_err(|_| BillingError::InvalidRequest("invalid_invoice_amount".into()))?;
        Ok(Self {
            id,
            tenant_id,
            subscription_id,
            amount,
            due_date,
            paid_at: None,
            status: InvoiceStatus::Open,
            asaas_payment_id: Some(asaas_payment_id),
            pdf_url: None,
        })
    }

    pub fn mark_paid(&mut self, paid_at: DateTime<Utc>) -> Result<(), BillingError> {
        self.transition_to(InvoiceStatus::Paid)?;
        self.paid_at = Some(paid_at);
        Ok(())
    }

    pub fn mark_overdue(&mut self) -> Result<(), BillingError> {
        self.transition_to(InvoiceStatus::Overdue)
    }

    pub fn cancel(&mut self) -> Result<(), BillingError> {
        self.transition_to(InvoiceStatus::Cancelled)
    }

    pub fn refund(&mut self) -> Result<(), BillingError> {
        self.transition_to(InvoiceStatus::Refunded)
    }

    fn transition_to(&mut self, target: InvoiceStatus) -> Result<(), BillingError> {
        if !can_transition_invoice(self.status, target) {
            return Err(BillingError::InvalidInvoiceTransition {
                from: self.status.as_str().to_owned(),
                to: target.as_str().to_owned(),
            });
        }
        self.status = target;
        Ok(())
    }
}

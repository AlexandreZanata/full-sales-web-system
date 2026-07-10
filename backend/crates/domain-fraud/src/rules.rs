use domain_shared::TenantId;
use uuid::Uuid;

use crate::fraud_event::{FraudEventType, FraudSeverity};

/// Input for pluggable fraud rule evaluation (Phase 6A).
#[derive(Debug, Clone, Default)]
pub struct FraudCheckContext {
    pub tenant_id: Option<TenantId>,
    pub user_id: Option<Uuid>,
    pub email: Option<String>,
    pub cnpj: Option<String>,
    pub ip: Option<String>,
    pub card_fingerprint: Option<String>,
    pub payment_amount_minor: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FraudCheckOutcome {
    Allow,
    Flag {
        event_type: FraudEventType,
        severity: FraudSeverity,
        metadata: serde_json::Value,
    },
    Block {
        event_type: FraudEventType,
        severity: FraudSeverity,
        metadata: serde_json::Value,
    },
}

/// Pluggable fraud rule contract — application wires concrete checks.
pub trait FraudRule: Send + Sync {
    fn name(&self) -> &'static str;
    fn evaluate(&self, ctx: &FraudCheckContext) -> FraudCheckOutcome;
}

use domain_shared::TenantId;

use crate::error::BillingError;
use crate::payment_method_toggles::PaymentMethodToggles;
use crate::plan_features::plan_allows_online_payments;

/// Tenant Admin payment collection settings (ADR-018).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TenantPaymentSettings {
    pub tenant_id: TenantId,
    pub enabled: bool,
    pub methods: PaymentMethodToggles,
    pub auto_capture: bool,
}

impl TenantPaymentSettings {
    pub fn defaults(tenant_id: TenantId) -> Self {
        Self {
            tenant_id,
            enabled: false,
            methods: PaymentMethodToggles::default(),
            auto_capture: true,
        }
    }

    pub fn apply_update(
        &mut self,
        enabled: bool,
        methods: PaymentMethodToggles,
        auto_capture: bool,
        plan_feature_limits: &serde_json::Value,
    ) -> Result<(), BillingError> {
        if enabled {
            ensure_online_payments_allowed(plan_feature_limits)?;
            if !methods.any_enabled() {
                return Err(BillingError::InvalidRequest(
                    "at_least_one_payment_method_required".into(),
                ));
            }
        }
        self.enabled = enabled;
        self.methods = methods;
        self.auto_capture = auto_capture;
        Ok(())
    }
}

pub fn ensure_online_payments_allowed(
    plan_feature_limits: &serde_json::Value,
) -> Result<(), BillingError> {
    if plan_allows_online_payments(plan_feature_limits) {
        Ok(())
    } else {
        Err(BillingError::PlanDoesNotAllowOnlinePayments)
    }
}

use domain_billing::{
    BillingError, BillingInterval, PaymentMethodToggles, SubscriptionPlan, TenantPaymentSettings,
    ensure_online_payments_allowed,
};
use domain_shared::TenantId;
use uuid::Uuid;

fn starter_plan() -> SubscriptionPlan {
    SubscriptionPlan::new(
        Uuid::parse_str("01900002-0001-7000-8000-000000000001").expect("starter id"),
        "Starter",
        "Starter",
        9_900,
        BillingInterval::Monthly,
        serde_json::json!({ "maxUsers": 5, "customDomain": false, "onlinePayments": false }),
    )
    .expect("starter plan")
}

fn pro_plan() -> SubscriptionPlan {
    SubscriptionPlan::new(
        Uuid::parse_str("01900002-0001-7000-8000-000000000002").expect("pro id"),
        "Pro",
        "Pro",
        19_900,
        BillingInterval::Monthly,
        serde_json::json!({ "maxUsers": 25, "customDomain": true, "onlinePayments": true }),
    )
    .expect("pro plan")
}

// BR-TP-001: Starter plan cannot enable online payments.
#[test]
fn given_starter_plan_when_enable_online_payments_then_rejected() {
    let plan = starter_plan();
    let err = ensure_online_payments_allowed(&plan.feature_limits).unwrap_err();
    assert_eq!(err, BillingError::PlanDoesNotAllowOnlinePayments);

    let mut settings = TenantPaymentSettings::defaults(TenantId::generate());
    let err = settings
        .apply_update(
            true,
            PaymentMethodToggles::all_enabled(),
            true,
            &plan.feature_limits,
        )
        .unwrap_err();
    assert_eq!(err, BillingError::PlanDoesNotAllowOnlinePayments);
}

#[test]
fn given_pro_plan_when_enable_online_payments_then_allowed() {
    let plan = pro_plan();
    ensure_online_payments_allowed(&plan.feature_limits).expect("pro allows online payments");

    let mut settings = TenantPaymentSettings::defaults(TenantId::generate());
    settings
        .apply_update(
            true,
            PaymentMethodToggles::all_enabled(),
            true,
            &plan.feature_limits,
        )
        .expect("enable");
    assert!(settings.enabled);
}

#[test]
fn given_enabled_settings_when_all_methods_disabled_then_rejected() {
    let plan = pro_plan();
    let mut settings = TenantPaymentSettings::defaults(TenantId::generate());
    let err = settings
        .apply_update(
            true,
            PaymentMethodToggles {
                pix: false,
                credit: false,
                boleto: false,
            },
            true,
            &plan.feature_limits,
        )
        .unwrap_err();
    assert!(matches!(err, BillingError::InvalidRequest(_)));
}

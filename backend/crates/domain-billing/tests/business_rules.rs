use chrono::{Duration, Utc};
use domain_billing::{
    BillingError, BillingInterval, Invoice, InvoiceStatus, Subscription, SubscriptionPlan,
    SubscriptionStatus, can_transition_invoice, can_transition_subscription,
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
        serde_json::json!({ "maxUsers": 5, "customDomain": false }),
    )
    .expect("starter plan")
}

// BR-BI-001 contract: duplicate webhook must not change subscription state.
#[test]
fn given_active_subscription_when_duplicate_payment_confirmed_then_state_unchanged() {
    let mut sub = Subscription::new_pending(Uuid::now_v7(), TenantId::generate(), starter_plan().id);
    let period_end = Utc::now() + Duration::days(30);
    sub.activate(period_end).expect("activate");
    let before = sub.status;
    sub.restore_from_payment(period_end).expect("idempotent active");
    assert_eq!(sub.status, before);
    assert_eq!(sub.status, SubscriptionStatus::Active);
}

// BR-BI-002 contract: trial converts to active on first payment.
#[test]
fn given_pending_subscription_when_activate_then_active() {
    let mut sub = Subscription::new_pending(Uuid::now_v7(), TenantId::generate(), starter_plan().id);
    let period_end = Utc::now() + Duration::days(30);
    sub.activate(period_end).expect("activate");
    assert_eq!(sub.status, SubscriptionStatus::Active);
    assert_eq!(sub.current_period_end, Some(period_end));
}

#[test]
fn given_active_subscription_when_overdue_then_past_due() {
    let mut sub = Subscription::new_pending(Uuid::now_v7(), TenantId::generate(), starter_plan().id);
    sub.activate(Utc::now() + Duration::days(30)).expect("activate");
    sub.mark_past_due().expect("past due");
    assert_eq!(sub.status, SubscriptionStatus::PastDue);
}

#[test]
fn given_past_due_subscription_when_payment_confirmed_then_active() {
    let mut sub = Subscription::new_pending(Uuid::now_v7(), TenantId::generate(), starter_plan().id);
    sub.activate(Utc::now()).expect("activate");
    sub.mark_past_due().expect("past due");
    let new_end = Utc::now() + Duration::days(30);
    sub.restore_from_payment(new_end).expect("restore");
    assert_eq!(sub.status, SubscriptionStatus::Active);
}

#[test]
fn given_expired_subscription_when_activate_then_invalid_transition() {
    let mut sub = Subscription::new_pending(Uuid::now_v7(), TenantId::generate(), starter_plan().id);
    sub.activate(Utc::now()).expect("activate");
    sub.cancel().expect("cancel");
    sub.expire().expect("expire");
    let err = sub.activate(Utc::now()).unwrap_err();
    assert!(matches!(
        err,
        BillingError::InvalidSubscriptionTransition { .. }
    ));
}

#[test]
fn given_open_invoice_when_paid_then_paid_at_set() {
    let tenant_id = TenantId::generate();
    let sub_id = Uuid::now_v7();
    let mut invoice = Invoice::new_open(
        Uuid::now_v7(),
        tenant_id,
        sub_id,
        9_900,
        Utc::now() + Duration::days(7),
        "pay_123".into(),
    )
    .expect("invoice");
    let paid_at = Utc::now();
    invoice.mark_paid(paid_at).expect("paid");
    assert_eq!(invoice.status, InvoiceStatus::Paid);
    assert_eq!(invoice.paid_at, Some(paid_at));
}

#[test]
fn given_paid_invoice_when_refund_then_refunded() {
    let mut invoice = Invoice::new_open(
        Uuid::now_v7(),
        TenantId::generate(),
        Uuid::now_v7(),
        9_900,
        Utc::now(),
        "pay_456".into(),
    )
    .expect("invoice");
    invoice.mark_paid(Utc::now()).expect("paid");
    invoice.refund().expect("refund");
    assert_eq!(invoice.status, InvoiceStatus::Refunded);
}

#[test]
fn given_invalid_subscription_transition_when_checked_then_false() {
    assert!(!can_transition_subscription(
        SubscriptionStatus::Expired,
        SubscriptionStatus::Active
    ));
}

#[test]
fn given_invalid_invoice_transition_when_checked_then_false() {
    assert!(!can_transition_invoice(InvoiceStatus::Refunded, InvoiceStatus::Paid));
}

#[test]
fn given_starter_plan_when_price_then_minor_units() {
    let plan = starter_plan();
    assert_eq!(plan.price.amount_minor(), 9_900);
    assert_eq!(plan.billing_interval, BillingInterval::Monthly);
}

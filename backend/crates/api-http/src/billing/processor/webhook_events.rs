use chrono::Utc;
use domain_billing::{InvoiceStatus, SubscriptionStatus};
use domain_shared::TenantId;
use uuid::Uuid;

use application::AppError;
use application::billing::{
    apply_payment_confirmed, apply_payment_overdue, apply_subscription_deleted,
};

use super::support::row_to_tenant;

pub async fn process_asaas_event(
    admin_pool: &infra_postgres::PgPool,
    event_type: &str,
    tenant_id: Option<TenantId>,
    payload: &serde_json::Value,
) -> Result<(), AppError> {
    let Some(tenant_id) = tenant_id else {
        return Ok(());
    };

    match event_type {
        "PAYMENT_CONFIRMED" | "PAYMENT_RECEIVED" => {
            handle_payment_confirmed(admin_pool, tenant_id, payload).await?;
        }
        "PAYMENT_OVERDUE" => handle_payment_overdue(admin_pool, tenant_id).await?,
        "SUBSCRIPTION_DELETED" => handle_subscription_deleted(admin_pool, tenant_id).await?,
        _ => {}
    }
    Ok(())
}

async fn handle_payment_confirmed(
    pool: &infra_postgres::PgPool,
    tenant_id: TenantId,
    payload: &serde_json::Value,
) -> Result<(), AppError> {
    let payment = payload.get("payment").ok_or(AppError::Forbidden)?;
    let payment_id = payment
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_owned();
    let value_minor = payment_minor(payment);
    let due_date = Utc::now() + chrono::Duration::days(7);
    let paid_at = Utc::now();
    let period_end = Utc::now() + chrono::Duration::days(30);

    let sub = infra_postgres::billing::find_subscription_by_tenant(pool, tenant_id)
        .await
        .map_err(|_| AppError::Forbidden)?;
    let Some(sub) = sub else {
        return Ok(());
    };

    infra_postgres::billing::upsert_invoice(
        pool,
        infra_postgres::billing::InvoiceUpsert {
            id: Uuid::now_v7(),
            tenant_id,
            subscription_id: sub.id,
            amount_minor: value_minor,
            due_date,
            status: InvoiceStatus::Paid,
            asaas_payment_id: payment_id,
            paid_at: Some(paid_at),
            pdf_url: payment
                .get("invoiceUrl")
                .and_then(|v| v.as_str())
                .map(str::to_owned),
        },
    )
    .await
    .map_err(|_| AppError::Forbidden)?;

    let new_status = if matches!(
        sub.status,
        SubscriptionStatus::Pending | SubscriptionStatus::PastDue
    ) {
        SubscriptionStatus::Active
    } else {
        sub.status
    };
    infra_postgres::billing::update_subscription_status(pool, sub.id, new_status, Some(period_end))
        .await
        .map_err(|_| AppError::Forbidden)?;

    let row = infra_postgres::shared::find_tenant_lifecycle(pool, tenant_id)
        .await
        .map_err(|_| AppError::Forbidden)?
        .ok_or(AppError::Forbidden)?;
    let mut tenant = row_to_tenant(&row);
    apply_payment_confirmed(&mut tenant)?;
    infra_postgres::shared::mark_tenant_payment_cleared(pool, tenant_id, tenant.status)
        .await
        .map_err(|_| AppError::Forbidden)?;
    Ok(())
}

async fn handle_payment_overdue(
    pool: &infra_postgres::PgPool,
    tenant_id: TenantId,
) -> Result<(), AppError> {
    if let Some(sub) = infra_postgres::billing::find_subscription_by_tenant(pool, tenant_id)
        .await
        .map_err(|_| AppError::Forbidden)?
    {
        infra_postgres::billing::update_subscription_status(
            pool,
            sub.id,
            SubscriptionStatus::PastDue,
            sub.current_period_end,
        )
        .await
        .map_err(|_| AppError::Forbidden)?;
    }

    let row = infra_postgres::shared::find_tenant_lifecycle(pool, tenant_id)
        .await
        .map_err(|_| AppError::Forbidden)?
        .ok_or(AppError::Forbidden)?;
    let mut tenant = row_to_tenant(&row);
    apply_payment_overdue(&mut tenant)?;
    infra_postgres::shared::mark_tenant_past_due(pool, tenant_id, tenant.status)
        .await
        .map_err(|_| AppError::Forbidden)?;
    Ok(())
}

async fn handle_subscription_deleted(
    pool: &infra_postgres::PgPool,
    tenant_id: TenantId,
) -> Result<(), AppError> {
    if let Some(sub) = infra_postgres::billing::find_subscription_by_tenant(pool, tenant_id)
        .await
        .map_err(|_| AppError::Forbidden)?
    {
        infra_postgres::billing::update_subscription_status(
            pool,
            sub.id,
            SubscriptionStatus::Cancelled,
            sub.current_period_end,
        )
        .await
        .map_err(|_| AppError::Forbidden)?;
    }

    let row = infra_postgres::shared::find_tenant_lifecycle(pool, tenant_id)
        .await
        .map_err(|_| AppError::Forbidden)?
        .ok_or(AppError::Forbidden)?;
    let mut tenant = row_to_tenant(&row);
    apply_subscription_deleted(&mut tenant)?;
    infra_postgres::shared::update_tenant_lifecycle(
        pool,
        tenant_id,
        tenant.status,
        tenant.plan_id,
        tenant.trial_ends_at,
        tenant.suspended_at,
        tenant.suspended_reason.as_deref(),
        tenant.offboarding_scheduled_at,
        None,
        None,
    )
    .await
    .map_err(|_| AppError::Forbidden)?;
    Ok(())
}

fn payment_minor(payment: &serde_json::Value) -> i64 {
    payment
        .get("value")
        .and_then(|v| v.as_f64())
        .map(|v| (v * 100.0).round() as i64)
        .unwrap_or(0)
}

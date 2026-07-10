use application::billing::{
    order_payment_external_reference, parse_order_payment_reference, primary_billing_type,
};
use uuid::Uuid;

use crate::error::ApiError;
use crate::state::AppState;

use super::support::{load_settings, map_billing_api, tenant_asaas_client};
use super::types::PaymentMethodsResponse;

pub async fn public_payment_methods(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
) -> Result<Option<PaymentMethodsResponse>, ApiError> {
    let settings = load_settings(state, tenant_id).await?;
    if !settings.enabled {
        return Ok(None);
    }
    Ok(Some(PaymentMethodsResponse {
        pix: settings.methods.pix,
        credit: settings.methods.credit,
        boleto: settings.methods.boleto,
    }))
}

pub async fn create_order_payment(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
    order_id: Uuid,
    customer_name: &str,
    total_minor: i64,
) -> Result<String, ApiError> {
    let settings = load_settings(state, tenant_id).await?;
    if !settings.enabled {
        return Err(ApiError::bad_request(
            "ONLINE_PAYMENTS_DISABLED",
            "Online payments are not enabled",
        ));
    }
    let client = tenant_asaas_client(state, tenant_id).await?;
    let value = total_minor as f64 / 100.0;
    let billing_type = primary_billing_type(settings.methods);
    let reference = order_payment_external_reference(order_id);
    let payment = client
        .create_payment(customer_name, billing_type, value, &reference)
        .await
        .map_err(map_billing_api)?;
    Ok(payment.id)
}

pub async fn process_order_payment_webhook(
    admin_pool: &infra_postgres::PgPool,
    event_type: &str,
    payload: &serde_json::Value,
) -> Result<(), application::AppError> {
    if !matches!(event_type, "PAYMENT_CONFIRMED" | "PAYMENT_RECEIVED") {
        return Ok(());
    }
    let payment = payload.get("payment").ok_or(application::AppError::Forbidden)?;
    let payment_id = payment
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let reference = payment
        .get("externalReference")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let Some(order_id) = parse_order_payment_reference(reference) else {
        return Ok(());
    };
    infra_postgres::orders::confirm_order_payment_admin(admin_pool, order_id, payment_id)
        .await
        .map_err(|_| application::AppError::Forbidden)?;
    Ok(())
}

use axum::{Json, extract::State};

use domain_billing::{BillingError, PaymentMethodToggles};

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;

use super::support::{
    audit_payment_action, ensure_admin, load_plan_limits, load_settings, map_billing_api,
};
use super::types::{
    AsaasConnectionResponse, PaymentMethodsResponse, PaymentSettingsResponse,
    UpdatePaymentSettingsRequest,
};

pub async fn get_payment_settings(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<PaymentSettingsResponse>, ApiError> {
    ensure_admin(&auth)?;
    payment_settings_response(&state, auth.tenant_id).await
}

pub async fn payment_settings_response(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
) -> Result<Json<PaymentSettingsResponse>, ApiError> {
    let settings = load_settings(state, tenant_id).await?;
    let creds = infra_postgres::billing::find_credentials(&state.admin_pool, tenant_id)
        .await
        .map_err(|_| ApiError::internal())?;
    Ok(Json(PaymentSettingsResponse {
        enabled: settings.enabled,
        methods: PaymentMethodsResponse {
            pix: settings.methods.pix,
            credit: settings.methods.credit,
            boleto: settings.methods.boleto,
        },
        auto_capture: settings.auto_capture,
        asaas: AsaasConnectionResponse {
            connected: creds.is_some(),
            api_key_last4: creds.as_ref().map(|c| c.api_key_last4.clone()),
            connected_at: creds.map(|c| c.connected_at),
        },
    }))
}

pub async fn update_payment_settings(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<UpdatePaymentSettingsRequest>,
) -> Result<Json<PaymentSettingsResponse>, ApiError> {
    ensure_admin(&auth)?;
    let limits = load_plan_limits(&state, auth.tenant_id).await?;
    let mut settings = load_settings(&state, auth.tenant_id).await?;
    let methods = PaymentMethodToggles {
        pix: body.methods.pix,
        credit: body.methods.credit,
        boleto: body.methods.boleto,
    };
    settings
        .apply_update(body.enabled, methods, body.auto_capture, &limits)
        .map_err(map_billing_api)?;
    if body.enabled {
        let connected = infra_postgres::billing::find_credentials(&state.admin_pool, auth.tenant_id)
            .await
            .map_err(|_| ApiError::internal())?
            .is_some();
        if !connected {
            return Err(map_billing_api(BillingError::TenantAsaasNotConnected));
        }
    }
    infra_postgres::billing::upsert_payment_settings(
        &state.admin_pool,
        auth.tenant_id,
        settings.enabled,
        settings.methods,
        settings.auto_capture,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    audit_payment_action(
        &state,
        &auth,
        "tenant.payment_settings.updated",
        serde_json::json!({ "enabled": settings.enabled }),
    )
    .await?;
    payment_settings_response(&state, auth.tenant_id).await
}

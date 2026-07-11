use axum::{Json, extract::State};

use application::billing::api_key_last4;
use domain_billing::ensure_online_payments_allowed;

use crate::audit_context::AuditRequestContext;
use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;

use super::support::{audit_payment_action, ensure_admin, load_plan_limits, map_billing_api};
use super::types::{ConnectAsaasRequest, ConnectAsaasResponse};

pub async fn connect_asaas(
    State(state): State<AppState>,
    auth: AuthUser,
    ctx: AuditRequestContext,
    Json(body): Json<ConnectAsaasRequest>,
) -> Result<Json<ConnectAsaasResponse>, ApiError> {
    ensure_admin(&auth)?;
    let limits = load_plan_limits(&state, auth.tenant_id).await?;
    ensure_online_payments_allowed(&limits).map_err(map_billing_api)?;
    let api_key = body.api_key.trim();
    if api_key.len() < 8 {
        return Err(ApiError::bad_request(
            "VALIDATION_ERROR",
            "apiKey must be at least 8 characters",
        ));
    }
    let last4 = api_key_last4(api_key)
        .ok_or_else(|| ApiError::bad_request("VALIDATION_ERROR", "apiKey is too short"))?;
    let client = infra_asaas::TenantAsaasClient::new(
        api_key.to_owned(),
        state.tenant_asaas_base_url.clone(),
    )
    .map_err(|_| ApiError::internal())?;
    let account = client.my_account().await.map_err(map_billing_api)?;
    let encryptor = state
        .credential_encryptor
        .as_ref()
        .ok_or_else(ApiError::internal)?;
    let blob = encryptor
        .encrypt(api_key)
        .map_err(|_| ApiError::internal())?;
    infra_postgres::billing::upsert_credentials(
        &state.admin_pool,
        auth.tenant_id,
        &blob.ciphertext,
        &blob.nonce,
        blob.key_version,
        &last4,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    audit_payment_action(
        &state,
        &ctx,
        &auth,
        "tenant.asaas.connected",
        serde_json::json!({ "apiKeyLast4": last4 }),
    )
    .await?;
    Ok(Json(ConnectAsaasResponse {
        connected: true,
        account_name: account.name,
    }))
}

pub async fn disconnect_asaas(
    State(state): State<AppState>,
    auth: AuthUser,
    ctx: AuditRequestContext,
) -> Result<axum::http::StatusCode, ApiError> {
    ensure_admin(&auth)?;
    infra_postgres::billing::delete_credentials(&state.admin_pool, auth.tenant_id)
        .await
        .map_err(|_| ApiError::internal())?;
    infra_postgres::billing::disable_online_payments(&state.admin_pool, auth.tenant_id)
        .await
        .map_err(|_| ApiError::internal())?;
    audit_payment_action(
        &state,
        &ctx,
        &auth,
        "tenant.asaas.disconnected",
        serde_json::json!({}),
    )
    .await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

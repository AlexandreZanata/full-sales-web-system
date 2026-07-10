use domain_billing::{BillingError, TenantPaymentSettings};
use domain_identity::Role;
use infra_crypto::EncryptedBlob;
use infra_postgres::audit::NewAuditEvent;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;

pub fn ensure_admin(auth: &AuthUser) -> Result<(), ApiError> {
    (auth.role == Role::Admin)
        .then_some(())
        .ok_or_else(ApiError::forbidden)
}

pub fn map_billing_api(err: BillingError) -> ApiError {
    match err {
        BillingError::PlanDoesNotAllowOnlinePayments => ApiError::forbidden_with_code(
            "PLAN_FEATURE_UNAVAILABLE",
            "Plan does not include online payments",
        ),
        BillingError::InvalidCredentials => ApiError::bad_request(
            "INVALID_ASAAS_CREDENTIALS",
            "Invalid Asaas API key",
        ),
        BillingError::TenantAsaasNotConnected => ApiError::bad_request(
            "ASAAS_NOT_CONNECTED",
            "Connect tenant Asaas credentials first",
        ),
        BillingError::InvalidRequest(_) => ApiError::bad_request(
            "VALIDATION_ERROR",
            "Invalid tenant payment settings",
        ),
        _ => ApiError::internal(),
    }
}

pub async fn load_plan_limits(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
) -> Result<serde_json::Value, ApiError> {
    let row = infra_postgres::shared::find_tenant_lifecycle(&state.admin_pool, tenant_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;
    let plan_limits = match row.plan_id {
        Some(plan_id) => {
            let plan = infra_postgres::billing::find_plan(&state.admin_pool, plan_id)
                .await
                .map_err(|_| ApiError::internal())?
                .ok_or_else(ApiError::not_found)?;
            plan.feature_limits
        }
        None => serde_json::json!({}),
    };
    let resolved = application::feature_flags::resolve_feature_flags(&plan_limits, &row.settings);
    Ok(serde_json::json!({
        "onlinePayments": resolved.online_payments,
        "customDomain": resolved.custom_domain,
        "apiRateTier": resolved.api_rate_tier,
    }))
}

pub async fn load_settings(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
) -> Result<TenantPaymentSettings, ApiError> {
    let row = infra_postgres::billing::find_payment_settings(&state.admin_pool, tenant_id)
        .await
        .map_err(|_| ApiError::internal())?;
    Ok(match row {
        Some(row) => TenantPaymentSettings {
            tenant_id,
            enabled: row.enabled,
            methods: row.methods,
            auto_capture: row.auto_capture,
        },
        None => TenantPaymentSettings::defaults(tenant_id),
    })
}

pub async fn tenant_asaas_client(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
) -> Result<infra_asaas::TenantAsaasClient, ApiError> {
    let creds = infra_postgres::billing::find_credentials(&state.admin_pool, tenant_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(|| map_billing_api(BillingError::TenantAsaasNotConnected))?;
    let encryptor = state
        .credential_encryptor
        .as_ref()
        .ok_or_else(ApiError::internal)?;
    let blob = EncryptedBlob {
        ciphertext: creds.ciphertext,
        nonce: creds.nonce,
        key_version: creds.key_version,
    };
    let api_key = encryptor
        .decrypt(&blob)
        .map_err(|_| ApiError::internal())?;
    infra_asaas::TenantAsaasClient::new(api_key, state.tenant_asaas_base_url.clone())
        .map_err(|_| ApiError::internal())
}

pub async fn audit_payment_action(
    state: &AppState,
    ctx: &crate::audit_context::AuditRequestContext,
    auth: &AuthUser,
    action: &str,
    metadata: serde_json::Value,
) -> Result<(), ApiError> {
    infra_postgres::audit::insert_audit_event(
        &state.app_pool,
        auth.tenant_id,
        NewAuditEvent {
            id: Uuid::now_v7(),
            actor_id: auth.user_id,
            actor_type: domain_audit::ActorType::User,
            action: action.to_owned(),
            resource_type: "TenantPaymentSettings".into(),
            resource_id: auth.tenant_id.as_uuid(),
            metadata: Some(metadata),
            correlation_id: ctx.correlation_id,
            ip: Some(ctx.ip.clone()),
        },
    )
    .await
    .map_err(|_| ApiError::internal())
}

pub async fn enforce_settlement_rate_limit(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
) -> Result<(), ApiError> {
    let key = format!("settlement:{}", tenant_id.as_uuid());
    if !state
        .rate_limiter
        .try_consume(&key, state.settlement_rate_limit.clone())
    {
        return Err(ApiError::rate_limited());
    }
    Ok(())
}

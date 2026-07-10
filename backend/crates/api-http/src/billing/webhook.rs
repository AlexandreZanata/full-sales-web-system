use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use domain_shared::TenantId;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::billing::webhook_auth::{WEBHOOK_TOKEN_HEADER, validate_webhook_token, webhook_token_from_env};
use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct AsaasWebhookPayload {
    pub id: String,
    pub event: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub received: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub duplicate: bool,
}

pub async fn asaas_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<serde_json::Value>,
) -> Result<(StatusCode, Json<WebhookResponse>), ApiError> {
    let expected = state
        .asaas_webhook_token
        .as_deref()
        .map(str::to_owned)
        .or_else(webhook_token_from_env)
        .ok_or_else(ApiError::webhook_unauthorized)?;
    let provided = headers
        .get(WEBHOOK_TOKEN_HEADER)
        .and_then(|v| v.to_str().ok());
    if !validate_webhook_token(provided, &expected) {
        return Err(ApiError::webhook_unauthorized());
    }

    let payload: AsaasWebhookPayload = serde_json::from_value(body.clone())
        .map_err(|_| ApiError::bad_request("INVALID_PAYLOAD", "Invalid Asaas webhook payload"))?;

    let tenant_id = extract_tenant_id(&body);
    let inserted = infra_postgres::billing::insert_payment_event(
        &state.admin_pool,
        infra_postgres::billing::PaymentEventInsert {
            asaas_event_id: payload.id.clone(),
            event_type: payload.event.clone(),
            tenant_id,
            payload: body.clone(),
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;

    if inserted {
        crate::billing::process_asaas_event(
            &state.admin_pool,
            &payload.event,
            tenant_id,
            &body,
        )
        .await
        .map_err(|_| ApiError::internal())?;
        infra_postgres::billing::mark_payment_event_processed(&state.admin_pool, &payload.id)
            .await
            .map_err(|_| ApiError::internal())?;
    }

    Ok((
        StatusCode::OK,
        Json(WebhookResponse {
            received: true,
            duplicate: !inserted,
        }),
    ))
}

fn extract_tenant_id(body: &serde_json::Value) -> Option<TenantId> {
    for key in ["payment", "subscription", "invoice"] {
        if let Some(reference) = body
            .get(key)
            .and_then(|v| v.get("externalReference"))
            .and_then(|v| v.as_str())
            && let Ok(uuid) = Uuid::parse_str(reference)
        {
            return Some(TenantId::from_uuid(uuid));
        }
    }
    None
}

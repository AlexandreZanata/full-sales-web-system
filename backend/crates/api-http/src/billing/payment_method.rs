use axum::{extract::State, http::StatusCode, Json};
use domain_identity::Role;
use serde::Deserialize;

use application::billing::AttachPaymentMethodRequest;
use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct PaymentMethodRequest {
    #[serde(rename = "type")]
    pub payment_type: String,
    #[serde(rename = "creditCardToken")]
    pub credit_card_token: String,
}

pub async fn attach_payment_method(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<PaymentMethodRequest>,
) -> Result<StatusCode, ApiError> {
    if auth.role != Role::Admin {
        return Err(ApiError::forbidden());
    }
    if body.payment_type != "credit_card" || body.credit_card_token.trim().is_empty() {
        return Err(ApiError::bad_request(
            "INVALID_PAYMENT_METHOD",
            "credit_card token required",
        ));
    }
    let customer_id = infra_postgres::billing::find_asaas_customer_id(&state.admin_pool, auth.tenant_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(|| ApiError::bad_request("CUSTOMER_NOT_FOUND", "Asaas customer not provisioned"))?;
    state
        .payment_gateway
        .attach_payment_method(AttachPaymentMethodRequest {
            customer_id,
            credit_card_token: body.credit_card_token,
        })
        .await
        .map_err(|_| ApiError::bad_request("PAYMENT_METHOD_FAILED", "Could not attach payment method"))?;
    Ok(StatusCode::CREATED)
}

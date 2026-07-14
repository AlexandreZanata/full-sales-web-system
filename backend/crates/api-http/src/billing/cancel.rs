//! POST /v1/billing/subscription/cancel (T-17-150 / OD-17-3).

use axum::{Json, extract::State, http::StatusCode};
use domain_billing::SubscriptionStatus;
use serde::Serialize;

use application::billing::CancelSubscriptionRequest;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;

fn ensure_tenant_admin(auth: &AuthUser) -> Result<(), ApiError> {
    (auth.role == domain_identity::Role::Admin)
        .then_some(())
        .ok_or_else(ApiError::forbidden)
}

#[derive(Serialize)]
pub struct CancelSubscriptionResponse {
    pub status: String,
    #[serde(rename = "cancelAtPeriodEnd")]
    pub cancel_at_period_end: bool,
}

pub async fn cancel_subscription(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<(StatusCode, Json<CancelSubscriptionResponse>), ApiError> {
    ensure_tenant_admin(&auth)?;
    let sub =
        infra_postgres::billing::find_subscription_by_tenant(&state.admin_pool, auth.tenant_id)
            .await
            .map_err(|_| ApiError::internal())?
            .ok_or_else(ApiError::not_found)?;

    if sub.status == SubscriptionStatus::Cancelled {
        return Ok((
            StatusCode::ACCEPTED,
            Json(CancelSubscriptionResponse {
                status: "Cancelled".into(),
                cancel_at_period_end: true,
            }),
        ));
    }

    if !matches!(
        sub.status,
        SubscriptionStatus::Active | SubscriptionStatus::PastDue
    ) {
        return Err(ApiError::bad_request(
            "INVALID_SUBSCRIPTION_STATUS",
            "Subscription cannot be cancelled in current status",
        ));
    }

    if let Some(asaas_id) = sub.asaas_subscription_id.as_deref() {
        state
            .payment_gateway
            .cancel_subscription(CancelSubscriptionRequest {
                subscription_id: asaas_id.to_owned(),
            })
            .await
            .map_err(|_| {
                ApiError::bad_request("CANCEL_FAILED", "Payment gateway rejected cancel")
            })?;
    }

    infra_postgres::billing::update_subscription_status(
        &state.admin_pool,
        sub.id,
        SubscriptionStatus::Cancelled,
        sub.current_period_end,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    Ok((
        StatusCode::ACCEPTED,
        Json(CancelSubscriptionResponse {
            status: "Cancelled".into(),
            cancel_at_period_end: true,
        }),
    ))
}

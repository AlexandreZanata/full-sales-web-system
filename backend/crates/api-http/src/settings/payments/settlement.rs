use axum::{
    Json,
    extract::{Query, State},
};

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;

use super::support::{
    enforce_settlement_rate_limit, ensure_admin, map_billing_api, tenant_asaas_client,
};
use super::types::{
    PaymentBalanceResponse, TransactionItem, TransactionsQuery, TransactionsResponse,
};

pub async fn get_payment_balance(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<PaymentBalanceResponse>, ApiError> {
    ensure_admin(&auth)?;
    enforce_settlement_rate_limit(&state, auth.tenant_id).await?;
    let cache_key = format!("balance:{}", auth.tenant_id.as_uuid());
    if let Some(cached) = state.settlement_cache.get(&cache_key).await {
        return Ok(Json(
            serde_json::from_value(cached).map_err(|_| ApiError::internal())?,
        ));
    }
    let client = tenant_asaas_client(&state, auth.tenant_id).await?;
    let balance = client.get_balance().await.map_err(map_billing_api)?;
    let response = PaymentBalanceResponse {
        balance_minor: (balance.balance * 100.0).round() as i64,
        currency: "BRL".to_owned(),
    };
    state
        .settlement_cache
        .set(
            cache_key,
            serde_json::to_value(&response).map_err(|_| ApiError::internal())?,
        )
        .await;
    Ok(Json(response))
}

pub async fn list_payment_transactions(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<TransactionsQuery>,
) -> Result<Json<TransactionsResponse>, ApiError> {
    ensure_admin(&auth)?;
    enforce_settlement_rate_limit(&state, auth.tenant_id).await?;
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(20).clamp(1, 50);
    let cache_key = format!("tx:{}:{offset}:{limit}", auth.tenant_id.as_uuid());
    if let Some(cached) = state.settlement_cache.get(&cache_key).await {
        return Ok(Json(
            serde_json::from_value(cached).map_err(|_| ApiError::internal())?,
        ));
    }
    let client = tenant_asaas_client(&state, auth.tenant_id).await?;
    let page = client
        .list_financial_transactions(offset, limit)
        .await
        .map_err(map_billing_api)?;
    let response = TransactionsResponse {
        data: page
            .data
            .into_iter()
            .map(|tx| TransactionItem {
                id: tx.id,
                tx_type: tx.tx_type,
                amount_minor: (tx.value * 100.0).round() as i64,
                date: tx.date,
                description: tx.description,
            })
            .collect(),
        has_more: page.has_more,
    };
    state
        .settlement_cache
        .set(
            cache_key,
            serde_json::to_value(&response).map_err(|_| ApiError::internal())?,
        )
        .await;
    Ok(Json(response))
}

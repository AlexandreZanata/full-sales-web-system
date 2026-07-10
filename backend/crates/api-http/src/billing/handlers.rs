use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;

fn ensure_tenant_admin(auth: &AuthUser) -> Result<(), ApiError> {
    (auth.role == domain_identity::Role::Admin)
        .then_some(())
        .ok_or_else(ApiError::forbidden)
}

#[derive(Deserialize)]
pub struct InvoiceListQuery {
    pub limit: Option<i64>,
    pub cursor: Option<Uuid>,
}

#[derive(Serialize)]
pub struct SubscriptionResponse {
    pub plan: PlanSummary,
    pub status: String,
    #[serde(rename = "tenantStatus")]
    pub tenant_status: String,
    #[serde(rename = "currentPeriodEnd")]
    pub current_period_end: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "trialEndsAt")]
    pub trial_ends_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize)]
pub struct PlanSummary {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    #[serde(rename = "priceMinor")]
    pub price_minor: i64,
    #[serde(rename = "billingInterval")]
    pub billing_interval: String,
}

#[derive(Serialize)]
pub struct InvoiceListResponse {
    pub data: Vec<InvoiceSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Uuid>,
}

#[derive(Serialize)]
pub struct InvoiceSummary {
    pub id: Uuid,
    #[serde(rename = "amountMinor")]
    pub amount_minor: i64,
    pub currency: String,
    #[serde(rename = "dueDate")]
    pub due_date: chrono::DateTime<chrono::Utc>,
    pub status: String,
    #[serde(rename = "paidAt")]
    pub paid_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize)]
pub struct InvoiceDetailResponse {
    pub id: Uuid,
    #[serde(rename = "amountMinor")]
    pub amount_minor: i64,
    pub currency: String,
    #[serde(rename = "dueDate")]
    pub due_date: chrono::DateTime<chrono::Utc>,
    pub status: String,
    #[serde(rename = "paidAt")]
    pub paid_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "pdfUrl")]
    pub pdf_url: Option<String>,
}

pub async fn get_subscription(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<SubscriptionResponse>, ApiError> {
    ensure_tenant_admin(&auth)?;
    let tenant_id = auth.tenant_id;
    let row = infra_postgres::shared::find_tenant_lifecycle(&state.admin_pool, tenant_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;
    let plan_id = row.plan_id.ok_or_else(ApiError::not_found)?;
    let plan = infra_postgres::billing::find_plan(&state.admin_pool, plan_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;
    let sub = infra_postgres::billing::find_subscription_by_tenant(&state.admin_pool, tenant_id)
        .await
        .map_err(|_| ApiError::internal())?;
    Ok(Json(SubscriptionResponse {
        plan: PlanSummary {
            id: plan.id,
            code: plan.code,
            name: plan.name,
            price_minor: plan.price_minor,
            billing_interval: plan.billing_interval,
        },
        status: sub
            .as_ref()
            .map(|s| s.status.as_str().to_owned())
            .unwrap_or_else(|| "Pending".into()),
        tenant_status: row.status.as_str().to_owned(),
        current_period_end: sub.as_ref().and_then(|s| s.current_period_end),
        trial_ends_at: row.trial_ends_at,
    }))
}

pub async fn list_invoices(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<InvoiceListQuery>,
) -> Result<Json<InvoiceListResponse>, ApiError> {
    ensure_tenant_admin(&auth)?;
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    let rows = infra_postgres::billing::list_invoices(
        &state.app_pool,
        auth.tenant_id,
        query.cursor,
        limit,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    let cursor = rows.last().map(|r| r.id);
    Ok(Json(InvoiceListResponse {
        data: rows
            .into_iter()
            .map(|r| InvoiceSummary {
                id: r.id,
                amount_minor: r.amount_minor,
                currency: r.amount_currency,
                due_date: r.due_date,
                status: r.status.as_str().to_owned(),
                paid_at: r.paid_at,
            })
            .collect(),
        cursor,
    }))
}

pub async fn get_invoice(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<InvoiceDetailResponse>, ApiError> {
    ensure_tenant_admin(&auth)?;
    let row = infra_postgres::billing::find_invoice(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;
    Ok(Json(InvoiceDetailResponse {
        id: row.id,
        amount_minor: row.amount_minor,
        currency: row.amount_currency,
        due_date: row.due_date,
        status: row.status.as_str().to_owned(),
        paid_at: row.paid_at,
        pdf_url: row.pdf_url,
    }))
}

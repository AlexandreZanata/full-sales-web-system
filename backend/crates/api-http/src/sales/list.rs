use axum::{
    Json,
    extract::{Query, State},
};
use chrono::{DateTime, Utc};
use domain_identity::Role;
use domain_sales::PaymentMethod;
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::pagination::paginate_offset;
use crate::sales::types::{SaleSummaryResponse, parse_sale_status};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ListSalesQuery {
    #[serde(default = "crate::pagination::default_page")]
    pub page: u32,
    #[serde(rename = "pageSize", default = "crate::pagination::default_page_size")]
    pub page_size: u32,
    #[serde(rename = "commerceId")]
    pub commerce_id: Option<Uuid>,
    #[serde(rename = "driverId")]
    pub driver_id: Option<Uuid>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
    pub status: Option<String>,
}

#[derive(serde::Serialize)]
pub struct PaginatedSalesResponse {
    pub items: Vec<SaleSummaryResponse>,
    pub page: u32,
    #[serde(rename = "pageSize")]
    pub page_size: u32,
    pub total: u64,
}

pub async fn list_sales(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<ListSalesQuery>,
) -> Result<Json<PaginatedSalesResponse>, ApiError> {
    match auth.role {
        Role::Admin | Role::Driver => {}
        _ => return Err(ApiError::forbidden()),
    }

    let (page, page_size, offset) = paginate_offset(query.page, query.page_size);
    let driver_id = match auth.role {
        Role::Driver => Some(auth.user_id),
        Role::Admin => query.driver_id,
        _ => None,
    };

    let filters = infra_postgres::sales::SaleFilters {
        commerce_id: query.commerce_id,
        driver_id,
        status: query.status.clone(),
        from: query.from,
        to: query.to,
    };

    let rows = infra_postgres::sales::list_sales(
        &state.app_pool,
        auth.tenant_id,
        &filters,
        page_size as i64,
        offset,
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let total = infra_postgres::sales::count_sales(&state.app_pool, auth.tenant_id, &filters)
        .await
        .map_err(|_| ApiError::internal())? as u64;

    let items: Vec<SaleSummaryResponse> =
        rows.into_iter().filter_map(sale_summary_from_row).collect();

    Ok(Json(PaginatedSalesResponse {
        items,
        page,
        page_size,
        total,
    }))
}

fn sale_summary_from_row(row: infra_postgres::sales::SaleListRow) -> Option<SaleSummaryResponse> {
    let status = parse_sale_status(&row.status).ok()?;
    let payment_method = PaymentMethod::parse(&row.payment_method).ok()?;
    Some(SaleSummaryResponse {
        id: row.id,
        commerce_id: row.commerce_id,
        driver_id: row.driver_id,
        status,
        payment_method,
        total_amount: row.total_amount,
        total_currency: row.total_currency,
        declared_payment_method: row.declared_payment_method,
        declared_payment_received: row.declared_payment_received,
        created_at: row.created_at,
    })
}

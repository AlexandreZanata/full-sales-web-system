use axum::{
    Json,
    extract::{RawQuery, State},
    response::{IntoResponse, Response},
};
use domain_identity::Role;
use domain_sales::PaymentMethod;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::list_query::{
    SALES_LIST_CONFIG, CursorListResponse, build_cursor_page, decode_query_pairs,
    filter_eq_string, filter_eq_uuid, filter_gte_datetime, filter_lte_datetime, parse_list_query,
};
use crate::sales::types::{SaleSummaryResponse, parse_sale_status};
use crate::state::AppState;

pub async fn list_sales(
    State(state): State<AppState>,
    auth: AuthUser,
    RawQuery(query): RawQuery,
) -> Result<Json<CursorListResponse<SaleSummaryResponse>>, Response> {
    match auth.role {
        Role::Admin | Role::Driver | Role::Seller => {}
        _ => return Err(IntoResponse::into_response(ApiError::forbidden())),
    }

    let parsed = parse_list_query(&decode_query_pairs(query.as_deref()), &SALES_LIST_CONFIG)
        .map_err(IntoResponse::into_response)?;

    let driver_id = match auth.role {
        Role::Driver | Role::Seller => Some(auth.user_id),
        Role::Admin => filter_eq_uuid(&parsed.filters, "driver_id"),
        _ => None,
    };

    let filters = infra_postgres::sales::SaleFilters {
        commerce_id: filter_eq_uuid(&parsed.filters, "commerce_id"),
        driver_id,
        status: filter_eq_string(&parsed.filters, "status"),
        from: filter_gte_datetime(&parsed.filters, "created_at"),
        to: filter_lte_datetime(&parsed.filters, "created_at"),
    };

    let rows = infra_postgres::sales::list_sales_cursor(
        &state.app_pool,
        auth.tenant_id,
        &filters,
        parsed.pagination.cursor,
        parsed.pagination.fetch_size() as i64,
    )
    .await
    .map_err(|_| IntoResponse::into_response(ApiError::internal()))?;

    let items: Vec<SaleSummaryResponse> = rows
        .into_iter()
        .filter_map(sale_summary_from_row)
        .collect();

    Ok(Json(build_cursor_page(
        items,
        parsed.pagination.limit,
        |sale| sale.id,
    )))
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

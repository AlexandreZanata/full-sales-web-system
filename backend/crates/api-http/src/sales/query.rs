use axum::{
    Json,
    extract::{Path, State},
};
use domain_identity::Role;
use domain_sales::PaymentMethod;
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::sales::types::{SaleItemResponse, SaleResponse, parse_sale_status};
use crate::state::AppState;

pub async fn get_sale(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<SaleResponse>, ApiError> {
    let row = infra_postgres::sales::find_sale_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::sale_not_found)?;

    if auth.role != Role::Admin && row.driver_id != auth.user_id {
        return Err(ApiError::sale_not_found());
    }

    let items = infra_postgres::sales::list_sale_items(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?;

    Ok(Json(build_sale_response(&row, items)))
}

pub(crate) fn build_sale_response(
    row: &infra_postgres::sales::SaleRow,
    items: Vec<infra_postgres::sales::SaleItemRow>,
) -> SaleResponse {
    let status = parse_sale_status(&row.status).unwrap_or(domain_sales::SaleStatus::Pending);
    let payment_method = PaymentMethod::parse(&row.payment_method).unwrap_or(PaymentMethod::Cash);

    SaleResponse {
        id: row.id,
        commerce_id: row.commerce_id,
        driver_id: row.driver_id,
        order_id: row.order_id,
        status,
        payment_method,
        declared_payment_method: row.declared_payment_method.clone(),
        declared_payment_received: row.declared_payment_received,
        total_amount: row.total_amount,
        total_currency: row.total_currency.clone(),
        items: items
            .into_iter()
            .map(|item| SaleItemResponse {
                product_id: item.product_id,
                quantity: item.quantity,
                unit_price_amount: item.unit_price_amount,
                unit_price_currency: item.unit_price_currency,
                line_total_amount: item.line_total_amount,
            })
            .collect(),
    }
}

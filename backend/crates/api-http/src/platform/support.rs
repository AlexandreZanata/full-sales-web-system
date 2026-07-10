use axum::{
    Json,
    extract::{Path, RawQuery, State},
    response::{IntoResponse, Response},
};
use domain_shared::TenantId;
use infra_postgres::rls::SessionContext;
use uuid::Uuid;

use crate::admin_orders::list::OrderSummaryResponse;
use crate::audit_context::AuditRequestContext;
use crate::error::ApiError;
use crate::list_query::{
    ORDERS_LIST_CONFIG, PRODUCTS_LIST_CONFIG, SALES_LIST_CONFIG, build_cursor_page,
    decode_query_pairs, filter_eq_bool, filter_eq_string, filter_eq_uuid, filter_gte_datetime,
    filter_lte_datetime, parse_list_query,
};
use crate::platform::auth::PlatformAuthUser;
use crate::platform_audit::record_platform_audit;
use crate::sales::types::SaleSummaryResponse;
use crate::products::ProductResponse;
use crate::state::AppState;

pub async fn list_tenant_orders_support(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    ctx: AuditRequestContext,
    Path(id): Path<Uuid>,
    RawQuery(query): RawQuery,
) -> Result<Json<crate::list_query::CursorListResponse<OrderSummaryResponse>>, Response> {
    let tenant_id = TenantId::from_uuid(id);
    ensure_tenant(&state, id).await.map_err(api_err_response)?;
    audit_support(&state, &ctx, &auth, id, "support.orders.list")
        .await
        .map_err(api_err_response)?;

    let session = admin_session(tenant_id, auth.user_id);
    let parsed = parse_list_query(&decode_query_pairs(query.as_deref()), &ORDERS_LIST_CONFIG)
        .map_err(|e| e.into_response("unknown".to_owned()))?;
    let status = filter_eq_string(&parsed.filters, "status");
    let filters = infra_postgres::orders::OrderListFilters {
        status: status.as_deref(),
        commerce_id: filter_eq_uuid(&parsed.filters, "commerce_id"),
        from: filter_gte_datetime(&parsed.filters, "created_at"),
        to: filter_lte_datetime(&parsed.filters, "created_at"),
    };
    let rows = infra_postgres::orders::list_orders_cursor(
        &state.app_pool,
        &session,
        &filters,
        parsed.pagination.cursor,
        parsed.pagination.fetch_size() as i64,
    )
    .await
    .map_err(|_| api_err_response(ApiError::internal()))?;

    let items: Vec<OrderSummaryResponse> = rows
        .into_iter()
        .map(|row| OrderSummaryResponse {
            id: row.id,
            status: row.status,
            commerce_id: row.commerce_id,
            total_amount: row.total_amount,
            total_currency: row.total_currency,
            created_at: row.created_at,
        })
        .collect();
    Ok(Json(build_cursor_page(
        items,
        parsed.pagination.limit,
        |item| item.id,
    )))
}

pub async fn list_tenant_sales_support(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    ctx: AuditRequestContext,
    Path(id): Path<Uuid>,
    RawQuery(query): RawQuery,
) -> Result<Json<crate::list_query::CursorListResponse<SaleSummaryResponse>>, Response> {
    let tenant_id = TenantId::from_uuid(id);
    ensure_tenant(&state, id).await.map_err(api_err_response)?;
    audit_support(&state, &ctx, &auth, id, "support.sales.list")
        .await
        .map_err(api_err_response)?;

    let parsed = parse_list_query(&decode_query_pairs(query.as_deref()), &SALES_LIST_CONFIG)
        .map_err(|e| e.into_response("unknown".to_owned()))?;
    let filters = infra_postgres::sales::SaleFilters {
        commerce_id: filter_eq_uuid(&parsed.filters, "commerce_id"),
        driver_id: filter_eq_uuid(&parsed.filters, "driver_id"),
        status: filter_eq_string(&parsed.filters, "status"),
        from: filter_gte_datetime(&parsed.filters, "created_at"),
        to: filter_lte_datetime(&parsed.filters, "created_at"),
    };
    let rows = infra_postgres::sales::list_sales_cursor(
        &state.app_pool,
        tenant_id,
        &filters,
        parsed.pagination.cursor,
        parsed.pagination.fetch_size() as i64,
    )
    .await
    .map_err(|_| api_err_response(ApiError::internal()))?;

    let items: Vec<SaleSummaryResponse> = rows
        .into_iter()
        .filter_map(crate::sales::sale_summary_from_row)
        .collect();
    Ok(Json(build_cursor_page(
        items,
        parsed.pagination.limit,
        |item| item.id,
    )))
}

pub async fn list_tenant_products_support(
    State(state): State<AppState>,
    auth: PlatformAuthUser,
    ctx: AuditRequestContext,
    Path(id): Path<Uuid>,
    RawQuery(query): RawQuery,
) -> Result<Json<crate::list_query::CursorListResponse<ProductResponse>>, Response> {
    let tenant_id = TenantId::from_uuid(id);
    ensure_tenant(&state, id).await.map_err(api_err_response)?;
    audit_support(&state, &ctx, &auth, id, "support.products.list")
        .await
        .map_err(api_err_response)?;

    let parsed = parse_list_query(&decode_query_pairs(query.as_deref()), &PRODUCTS_LIST_CONFIG)
        .map_err(|e| e.into_response("unknown".to_owned()))?;
    let active = filter_eq_bool(&parsed.filters, "active");
    let rows = infra_postgres::inventory::list_products_cursor(
        &state.app_pool,
        tenant_id,
        active,
        parsed.pagination.cursor,
        parsed.pagination.fetch_size() as i64,
    )
    .await
    .map_err(|_| api_err_response(ApiError::internal()))?;

    let items: Vec<ProductResponse> = rows
        .into_iter()
        .map(|row| crate::products::product_response_from_row(&row, None))
        .collect();
    Ok(Json(build_cursor_page(
        items,
        parsed.pagination.limit,
        |item| item.id,
    )))
}

fn admin_session(tenant_id: TenantId, platform_user_id: Uuid) -> SessionContext {
    SessionContext {
        tenant_id,
        role: "Admin".into(),
        user_id: platform_user_id,
        commerce_id: None,
    }
}

async fn ensure_tenant(state: &AppState, id: Uuid) -> Result<(), ApiError> {
    let exists = infra_postgres::identity::tenant_exists(&state.admin_pool, id)
        .await
        .map_err(|_| ApiError::internal())?;
    if !exists {
        return Err(ApiError::not_found());
    }
    Ok(())
}

async fn audit_support(
    state: &AppState,
    ctx: &AuditRequestContext,
    auth: &PlatformAuthUser,
    tenant_id: Uuid,
    action: &str,
) -> Result<(), ApiError> {
    record_platform_audit(
        state,
        ctx,
        auth.user_id,
        action,
        Some(TenantId::from_uuid(tenant_id)),
        "Tenant",
        tenant_id,
        None,
    )
    .await
}

fn api_err_response(err: ApiError) -> Response {
    IntoResponse::into_response(err)
}

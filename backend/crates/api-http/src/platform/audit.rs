use axum::{
    Json,
    extract::{RawQuery, State},
};

use crate::audit::AuditEventResponse;
use crate::error::ApiError;
use crate::list_query::{
    CursorListResponse, build_cursor_page, decode_query_pairs, filter_eq_string, filter_eq_uuid,
    filter_gte_datetime, filter_lte_datetime, parse_list_query,
};
use crate::platform::auth::PlatformAuthUser;
use crate::state::AppState;

static PLATFORM_AUDIT_FILTERS: [application::list_query::FilterFieldSpec; 4] = [
    application::list_query::FilterFieldSpec::new("tenant_id", &[application::list_query::ListFilterOp::Eq]),
    application::list_query::FilterFieldSpec::new("actor_id", &[application::list_query::ListFilterOp::Eq]),
    application::list_query::FilterFieldSpec::new("action", &[application::list_query::ListFilterOp::Eq]),
    application::list_query::FilterFieldSpec::new(
        "created_at",
        &[
            application::list_query::ListFilterOp::Gte,
            application::list_query::ListFilterOp::Lte,
        ],
    ),
];

static PLATFORM_AUDIT_LIST_CONFIG: crate::list_query::RouteListConfig<'static> =
    crate::list_query::RouteListConfig {
        filter_whitelist: &PLATFORM_AUDIT_FILTERS,
        sort_whitelist: &[],
    };

pub async fn list_platform_audit_events(
    State(state): State<AppState>,
    _auth: PlatformAuthUser,
    RawQuery(query): RawQuery,
) -> Result<Json<CursorListResponse<AuditEventResponse>>, ApiError> {
    let parsed = parse_list_query(
        &decode_query_pairs(query.as_deref()),
        &PLATFORM_AUDIT_LIST_CONFIG,
    )
    .map_err(|_| ApiError::bad_request("INVALID_QUERY", "Invalid audit list query"))?;
    let from = filter_gte_datetime(&parsed.filters, "created_at");
    let to = filter_lte_datetime(&parsed.filters, "created_at");
    let (from, to) = application::validate_audit_date_range(from, to).map_err(map_range_error)?;
    let filters = infra_postgres::audit::AuditEventFilters {
        tenant_id: filter_eq_uuid(&parsed.filters, "tenant_id"),
        actor_id: filter_eq_uuid(&parsed.filters, "actor_id"),
        action: filter_eq_string(&parsed.filters, "action"),
        from: Some(from),
        to: Some(to),
    };
    let rows = infra_postgres::audit::list_audit_events_platform(
        &state.admin_pool,
        &filters,
        parsed.pagination.cursor,
        parsed.pagination.fetch_size() as i64,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    let items = rows.into_iter().map(map_row).collect();
    Ok(Json(build_cursor_page(
        items,
        parsed.pagination.limit,
        |event| event.id,
    )))
}

pub(crate) fn map_range_error(err: application::AuditRangeError) -> ApiError {
    match err {
        application::AuditRangeError::InvalidRange => {
            ApiError::bad_request("INVALID_DATE_RANGE", "Invalid audit date range")
        }
        application::AuditRangeError::TooWide => ApiError::bad_request(
            "AUDIT_RANGE_TOO_WIDE",
            "Audit query range exceeds 90 days",
        ),
    }
}

pub(crate) fn map_row(row: infra_postgres::audit::AuditEventRow) -> AuditEventResponse {
    AuditEventResponse {
        id: row.id,
        tenant_id: row.tenant_id,
        actor_id: row.actor_id,
        actor_type: row.actor_type,
        action: row.action,
        resource_type: row.resource_type,
        resource_id: row.resource_id,
        metadata: row.metadata,
        correlation_id: row.correlation_id,
        ip: row.ip,
        created_at: row.created_at,
    }
}

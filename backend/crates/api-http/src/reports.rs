mod export;
mod support;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::Response,
};
use uuid::Uuid;

use crate::auth::{AuthUser, require_admin};
use crate::error::ApiError;
use crate::pagination::paginate_offset;
use crate::state::AppState;
use crate::validation::ValidatedJson;

pub use support::{
    GenerateReportRequest, PaginatedReportsResponse, ReportResponse, ReportsQuery,
    VerifyReportResponse,
};

pub async fn generate_report(
    State(state): State<AppState>,
    auth: AuthUser,
    ValidatedJson(body): ValidatedJson<GenerateReportRequest>,
) -> Result<Response, ApiError> {
    require_admin(&auth)?;
    let signing_key = state
        .report_signing_key
        .as_ref()
        .ok_or_else(ApiError::signing_key_unavailable)?;
    let (report_id, row) =
        support::build_and_persist(&state, auth.tenant_id, &body, signing_key).await?;
    let location = format!("/v1/reports/{report_id}");
    let payload =
        serde_json::to_vec(&support::report_to_response(&row)).map_err(|_| ApiError::internal())?;
    Response::builder()
        .status(StatusCode::CREATED)
        .header(http::header::CONTENT_TYPE, "application/json")
        .header(
            http::header::LOCATION,
            http::HeaderValue::from_str(&location).map_err(|_| ApiError::internal())?,
        )
        .body(axum::body::Body::from(payload))
        .map_err(|_| ApiError::internal())
}

pub async fn list_reports(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(query): Query<ReportsQuery>,
) -> Result<Json<PaginatedReportsResponse>, ApiError> {
    require_admin(&auth)?;
    // offset-based: admin jumps to arbitrary report page
    let (page, page_size, offset) = paginate_offset(query.page, query.page_size);
    let rows = infra_postgres::reports::list_reports(
        &state.app_pool,
        auth.tenant_id,
        page_size as i64,
        offset,
    )
    .await
    .map_err(|_| ApiError::internal())?;
    let total = infra_postgres::reports::list_report_ids(&state.app_pool, auth.tenant_id)
        .await
        .map_err(|_| ApiError::internal())?
        .len() as u64;
    Ok(Json(PaginatedReportsResponse {
        items: rows.iter().map(support::report_to_response).collect(),
        page,
        page_size,
        total,
    }))
}

pub async fn get_report(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ReportResponse>, ApiError> {
    let row = infra_postgres::reports::find_report_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::report_not_found)?;
    support::ensure_can_read_report(&auth, &row.canonical_payload)?;
    Ok(Json(support::report_to_response(&row)))
}

pub use export::export_report;

pub async fn verify_report(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<VerifyReportResponse>, ApiError> {
    let rate_key = format!("ratelimit:verify:{}", crate::client_ip::client_ip(&headers));
    if !state
        .rate_limiter
        .try_consume(&rate_key, state.verify_rate_limit)
    {
        return Err(ApiError::rate_limited());
    }
    Ok(Json(support::verify(&state, id).await?))
}

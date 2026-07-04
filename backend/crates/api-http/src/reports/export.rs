use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{HeaderValue, StatusCode},
    response::Response,
};
use domain_reports::{
    ExportBranding, ExportFormat, ExportMeta, ReportExportError, parse_export_view, render_export,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::{AuthUser, require_admin};
use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ExportQuery {
    pub format: String,
}

pub async fn export_report(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Query(query): Query<ExportQuery>,
) -> Result<Response, ApiError> {
    require_admin(&auth)?;
    let format = ExportFormat::parse(&query.format)
        .map_err(|_| ApiError::bad_request("UNSUPPORTED_FORMAT", "Unsupported export format"))?;

    let row = infra_postgres::reports::find_report_by_id(&state.app_pool, auth.tenant_id, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::report_not_found)?;

    let view =
        parse_export_view(&row.report_type, &row.canonical_payload).map_err(map_export_error)?;

    let branding = if format == ExportFormat::Pdf {
        Some(load_export_branding(&state, auth.tenant_id).await?)
    } else {
        None
    };

    let meta = ExportMeta {
        report_id: row.id,
        report_type: row.report_type.clone(),
        period_start: row.period_start,
        verify_url: Some(format!("/v1/reports/{id}/verify")),
    };

    let rendered =
        render_export(&view, &meta, format, branding.as_ref()).map_err(map_export_error)?;

    let disposition = format!("attachment; filename=\"{}\"", rendered.filename);
    Response::builder()
        .status(StatusCode::OK)
        .header(
            http::header::CONTENT_TYPE,
            HeaderValue::from_str(rendered.content_type).map_err(|_| ApiError::internal())?,
        )
        .header(
            http::header::CONTENT_DISPOSITION,
            HeaderValue::from_str(&disposition).map_err(|_| ApiError::internal())?,
        )
        .body(Body::from(rendered.bytes))
        .map_err(|_| ApiError::internal())
}

async fn load_export_branding(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
) -> Result<ExportBranding, ApiError> {
    let tenant = infra_postgres::shared::find_tenant_by_id(&state.app_pool, tenant_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;

    Ok(ExportBranding {
        display_name: tenant.display_name,
        logo_png: None,
    })
}

fn map_export_error(err: ReportExportError) -> ApiError {
    match err {
        ReportExportError::UnsupportedFormat => {
            ApiError::bad_request("UNSUPPORTED_FORMAT", "Unsupported export format")
        }
        ReportExportError::UnsupportedReportType(_) => ApiError::bad_request(
            "VALIDATION_ERROR",
            "Export not supported for this report type",
        ),
        ReportExportError::InvalidJson
        | ReportExportError::MissingField(_)
        | ReportExportError::UnsupportedVersion(_) => {
            ApiError::bad_request("VALIDATION_ERROR", "Report payload cannot be exported")
        }
        ReportExportError::RenderFailed => ApiError::internal(),
    }
}

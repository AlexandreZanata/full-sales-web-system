use domain_commerces::CommerceId;
use domain_identity::{Role, UserId};
use domain_orders::{OrderId, OrderStatus};
use domain_reports::{
    sign_canonical_payload, verify_canonical_payload, ReportAssemblyInput, ReportPeriod,
    ReportSaleFact,
};
use domain_sales::{DeclaredPaymentMethod, SaleId, SaleStatus};
use ed25519_dalek::{SigningKey, VerifyingKey};
use infra_postgres::reports::{NewReport, ReportRow, SaleReportQuery};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GenerateReportRequest {
    #[serde(rename = "reportType")]
    pub report_type: String,
    #[serde(rename = "periodStart")]
    pub period_start: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "periodEnd")]
    pub period_end: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "driverId")]
    pub driver_id: Option<Uuid>,
    #[serde(rename = "commerceId")]
    pub commerce_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct ReportsQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(rename = "pageSize", default = "default_page_size")]
    pub page_size: u32,
}

#[derive(Serialize)]
pub struct ReportResponse {
    pub id: Uuid,
    #[serde(rename = "reportType")]
    pub report_type: String,
    #[serde(rename = "periodStart")]
    pub period_start: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "periodEnd")]
    pub period_end: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "canonicalPayload")]
    pub canonical_payload: String,
    pub signature: String,
    #[serde(rename = "publicKeyId")]
    pub public_key_id: String,
    #[serde(rename = "generatedAt")]
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub struct PaginatedReportsResponse {
    pub items: Vec<ReportResponse>,
    pub page: u32,
    #[serde(rename = "pageSize")]
    pub page_size: u32,
    pub total: u64,
}

#[derive(Serialize)]
pub struct VerifyReportResponse {
    pub valid: bool,
    #[serde(rename = "reportId")]
    pub report_id: Uuid,
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    20
}

pub async fn build_and_persist(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
    body: &GenerateReportRequest,
    signing_key: &SigningKey,
) -> Result<(Uuid, ReportRow), ApiError> {
    validate_report_type(&body.report_type)?;
    let driver_id = body
        .driver_id
        .ok_or_else(|| ApiError::bad_request("VALIDATION_ERROR", "driverId is required for this report"))?;
    let key_row = infra_postgres::reports::find_active_signing_key(&state.app_pool, tenant_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::signing_key_unavailable)?;

    let sales = load_report_sales(state, tenant_id, body).await?;
    let assembled = ReportAssemblyInput {
        period: ReportPeriod {
            start: body.period_start,
            end: body.period_end,
        },
        driver_id: UserId::from_uuid(driver_id),
        sales,
    }
    .assemble()
    .map_err(|_| ApiError::bad_request("VALIDATION_ERROR", "Invalid report period"))?;

    let signature = sign_canonical_payload(&assembled.canonical_json, signing_key);
    let report_id = Uuid::now_v7();
    infra_postgres::reports::insert_report(
        &state.app_pool,
        tenant_id,
        NewReport {
            id: report_id,
            report_type: &body.report_type,
            period_start: body.period_start,
            period_end: body.period_end,
            canonical_payload: &assembled.canonical_json,
            signature: &signature,
            public_key_id: &key_row.public_key_id,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;

    let row = infra_postgres::reports::find_report_by_id(&state.app_pool, tenant_id, report_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::internal)?;
    Ok((report_id, row))
}

pub async fn verify(state: &AppState, id: Uuid) -> Result<VerifyReportResponse, ApiError> {
    let row = infra_postgres::reports::find_report_by_id_admin(&state.admin_pool, id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::report_not_found)?;
    let key = infra_postgres::reports::find_signing_key_by_public_key_id(
        &state.admin_pool,
        &row.public_key_id,
    )
    .await
    .map_err(|_| ApiError::internal())?
    .ok_or_else(ApiError::report_not_found)?;
    let verifying_key = VerifyingKey::from_bytes(
        key.public_key
            .as_slice()
            .try_into()
            .map_err(|_| ApiError::internal())?,
    )
    .map_err(|_| ApiError::internal())?;
    Ok(VerifyReportResponse {
        valid: verify_canonical_payload(&row.canonical_payload, &row.signature, &verifying_key),
        report_id: row.id,
    })
}

pub fn report_to_response(row: &ReportRow) -> ReportResponse {
    ReportResponse {
        id: row.id,
        report_type: row.report_type.clone(),
        period_start: row.period_start,
        period_end: row.period_end,
        canonical_payload: row.canonical_payload.clone(),
        signature: hex::encode(&row.signature),
        public_key_id: row.public_key_id.clone(),
        generated_at: row.generated_at,
    }
}

pub fn ensure_can_read_report(auth: &AuthUser, canonical_payload: &str) -> Result<(), ApiError> {
    if auth.role == Role::Admin {
        return Ok(());
    }
    if auth.role != Role::Driver {
        return Err(ApiError::forbidden());
    }
    let value: serde_json::Value =
        serde_json::from_str(canonical_payload).map_err(|_| ApiError::internal())?;
    let driver_id = value
        .get("driverId")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(ApiError::forbidden)?;
    if driver_id == auth.user_id {
        Ok(())
    } else {
        Err(ApiError::forbidden())
    }
}

async fn load_report_sales(
    state: &AppState,
    tenant_id: domain_shared::TenantId,
    body: &GenerateReportRequest,
) -> Result<Vec<ReportSaleFact>, ApiError> {
    let rows = infra_postgres::reports::query_sales_for_report(
        &state.app_pool,
        tenant_id,
        &SaleReportQuery {
            period_start: body.period_start,
            period_end: body.period_end,
            driver_id: body.driver_id,
            commerce_id: body.commerce_id,
        },
    )
    .await
    .map_err(|_| ApiError::internal())?;
    rows.into_iter().map(map_sale_fact).collect()
}

fn map_sale_fact(row: infra_postgres::reports::SaleReportFactRow) -> Result<ReportSaleFact, ApiError> {
    Ok(ReportSaleFact {
        sale_id: SaleId::from_uuid(row.sale_id),
        order_id: row.order_id.map(OrderId::from_uuid),
        commerce_id: CommerceId::from_uuid(row.commerce_id),
        amount_cents: row.total_amount,
        currency: row.total_currency,
        sale_status: parse_sale_status(&row.status)?,
        order_status: row
            .order_status
            .as_deref()
            .map(parse_order_status)
            .transpose()?,
        declared_method: DeclaredPaymentMethod::parse(&row.declared_payment_method)
            .map_err(|_| ApiError::internal())?,
        declared_received: row.declared_payment_received,
    })
}

fn validate_report_type(value: &str) -> Result<(), ApiError> {
    match value {
        "DailyDriver" | "CommercePeriod" | "Consolidated" => Ok(()),
        _ => Err(ApiError::bad_request("VALIDATION_ERROR", "Invalid reportType")),
    }
}

fn parse_sale_status(value: &str) -> Result<SaleStatus, ApiError> {
    match value {
        "Pending" => Ok(SaleStatus::Pending),
        "Confirmed" => Ok(SaleStatus::Confirmed),
        "Cancelled" => Ok(SaleStatus::Cancelled),
        _ => Err(ApiError::internal()),
    }
}

fn parse_order_status(value: &str) -> Result<OrderStatus, ApiError> {
    value.parse().map_err(|_| ApiError::internal())
}

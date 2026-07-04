use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportExportView {
    pub version: u32,
    pub period: ExportPeriod,
    pub driver_id: Uuid,
    pub sales: Vec<ExportSaleRow>,
    pub settlement: ExportSettlement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportPeriod {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportSaleRow {
    pub sale_id: Uuid,
    pub order_id: Option<Uuid>,
    pub commerce_id: Uuid,
    pub amount_cents: i64,
    pub currency: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportSettlement {
    pub total_declared_cents: i64,
    pub currency: String,
    pub by_payment_method: Vec<(String, i64)>,
    pub disclaimer: String,
}

#[derive(Debug, Clone)]
pub struct ExportBranding {
    pub display_name: String,
    pub logo_png: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct ExportMeta {
    pub report_id: Uuid,
    pub report_type: String,
    pub period_start: DateTime<Utc>,
    pub verify_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedExport {
    pub bytes: Vec<u8>,
    pub content_type: &'static str,
    pub filename: String,
}

pub const PDF_ROWS_PER_PAGE: usize = 40;

pub mod csv;
pub mod error;
pub mod format;
pub mod parse;
pub mod pdf;
mod pdf_body;
mod pdf_draw;
mod pdf_header;
pub mod view;
pub mod xlsx;

pub use csv::render_csv;
pub use error::ReportExportError;
pub use parse::parse_export_view;
pub use pdf::render_pdf;
pub use view::{
    ExportBranding, ExportMeta, ExportPeriod, ExportSaleRow, ExportSettlement, PDF_ROWS_PER_PAGE,
    RenderedExport, ReportExportView,
};
pub use xlsx::render_xlsx;

use view::ExportBranding as Branding;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Pdf,
    Csv,
    Xlsx,
}

impl ExportFormat {
    pub fn parse(value: &str) -> Result<Self, ReportExportError> {
        match value {
            "pdf" => Ok(Self::Pdf),
            "csv" => Ok(Self::Csv),
            "xlsx" => Ok(Self::Xlsx),
            _ => Err(ReportExportError::UnsupportedFormat),
        }
    }
}

pub fn render_export(
    view: &ReportExportView,
    meta: &ExportMeta,
    format: ExportFormat,
    branding: Option<&Branding>,
) -> Result<RenderedExport, ReportExportError> {
    match format {
        ExportFormat::Csv => render_csv(view, meta),
        ExportFormat::Xlsx => render_xlsx(view, meta),
        ExportFormat::Pdf => {
            let branding = branding.ok_or(ReportExportError::RenderFailed)?;
            render_pdf(view, meta, branding)
        }
    }
}

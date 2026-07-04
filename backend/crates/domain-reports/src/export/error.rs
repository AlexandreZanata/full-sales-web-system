use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ReportExportError {
    #[error("canonical payload is invalid JSON")]
    InvalidJson,
    #[error("missing required field: {0}")]
    MissingField(&'static str),
    #[error("unsupported payload version: {0}")]
    UnsupportedVersion(u32),
    #[error("unsupported report type for export: {0}")]
    UnsupportedReportType(String),
    #[error("unsupported export format")]
    UnsupportedFormat,
    #[error("export render failed")]
    RenderFailed,
}

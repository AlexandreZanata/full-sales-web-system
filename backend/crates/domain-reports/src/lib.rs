//! Reports domain — canonical payload assembly and signature verification.

pub mod canonical;
pub mod error;
pub mod export;
pub mod payload;
pub mod signature;

pub use canonical::to_canonical_json;
pub use error::ReportError;
pub use export::{
    ExportBranding, ExportFormat, ExportMeta, ReportExportError, ReportExportView, RenderedExport,
    parse_export_view, render_export,
};
pub use payload::{
    AssembledReportPayload, PAYLOAD_VERSION, ReportAssemblyInput, ReportPeriod, ReportSaleFact,
    SETTLEMENT_DISCLAIMER, is_eligible_for_report,
};
pub use signature::{sign_canonical_payload, verify_canonical_payload};

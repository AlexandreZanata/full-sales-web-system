//! Reports domain — canonical payload assembly and signature verification.

pub mod canonical;
pub mod error;
pub mod payload;
pub mod signature;

pub use canonical::to_canonical_json;
pub use error::ReportError;
pub use payload::{
    is_eligible_for_report, AssembledReportPayload, ReportAssemblyInput, ReportPeriod,
    ReportSaleFact, PAYLOAD_VERSION, SETTLEMENT_DISCLAIMER,
};
pub use signature::{sign_canonical_payload, verify_canonical_payload};

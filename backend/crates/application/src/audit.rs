use std::time::Duration;

use chrono::{DateTime, Utc};
use domain_audit::ActorType;
use domain_shared::TenantId;
use uuid::Uuid;

pub const MAX_AUDIT_RANGE_DAYS: i64 = 90;

#[derive(Debug, Clone)]
pub struct RecordAuditEvent {
    pub id: Uuid,
    pub actor_id: Uuid,
    pub actor_type: ActorType,
    pub tenant_id: Option<TenantId>,
    pub action: String,
    pub resource: String,
    pub resource_id: Uuid,
    pub metadata: Option<serde_json::Value>,
    pub ip: Option<String>,
    pub correlation_id: Option<Uuid>,
}

pub fn validate_audit_date_range(
    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>,
) -> Result<(DateTime<Utc>, DateTime<Utc>), AuditRangeError> {
    let end = to.unwrap_or_else(Utc::now);
    let start = from.unwrap_or_else(|| end - Duration::from_secs(MAX_AUDIT_RANGE_DAYS as u64 * 86400));
    if end < start {
        return Err(AuditRangeError::InvalidRange);
    }
    if end - start > chrono::Duration::days(MAX_AUDIT_RANGE_DAYS) {
        return Err(AuditRangeError::TooWide);
    }
    Ok((start, end))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditRangeError {
    InvalidRange,
    TooWide,
}

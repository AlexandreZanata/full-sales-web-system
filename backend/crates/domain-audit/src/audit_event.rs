use chrono::{DateTime, Utc};
use domain_shared::TenantId;
use uuid::Uuid;

use crate::actor_type::ActorType;

#[derive(Debug, Clone)]
pub struct AuditEvent {
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
    pub created_at: DateTime<Utc>,
}

impl AuditEvent {
    pub fn new(
        id: Uuid,
        actor_id: Uuid,
        actor_type: ActorType,
        tenant_id: Option<TenantId>,
        action: impl Into<String>,
        resource: impl Into<String>,
        resource_id: Uuid,
        metadata: Option<serde_json::Value>,
        ip: Option<String>,
        correlation_id: Option<Uuid>,
    ) -> Self {
        Self {
            id,
            actor_id,
            actor_type,
            tenant_id,
            action: action.into(),
            resource: resource.into(),
            resource_id,
            metadata,
            ip,
            correlation_id,
            created_at: Utc::now(),
        }
    }
}

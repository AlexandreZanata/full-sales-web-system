use domain_audit::ActorType;
use domain_shared::TenantId;
use infra_postgres::audit::NewAuditEvent;
use uuid::Uuid;

use crate::audit_context::AuditRequestContext;
use crate::error::ApiError;
use crate::state::AppState;

#[allow(clippy::too_many_arguments)]
pub async fn record_platform_audit(
    state: &AppState,
    ctx: &AuditRequestContext,
    actor_id: Uuid,
    action: &str,
    tenant_id: Option<TenantId>,
    resource: &str,
    resource_id: Uuid,
    metadata: Option<serde_json::Value>,
) -> Result<(), ApiError> {
    infra_postgres::audit::insert_platform_audit_event(
        &state.admin_pool,
        tenant_id,
        NewAuditEvent {
            id: Uuid::now_v7(),
            actor_id,
            actor_type: ActorType::PlatformAdmin,
            action: action.to_owned(),
            resource_type: resource.to_owned(),
            resource_id,
            metadata,
            correlation_id: ctx.correlation_id,
            ip: Some(ctx.ip.clone()),
        },
    )
    .await
    .map_err(|_| ApiError::internal())
}

#[allow(dead_code, clippy::too_many_arguments)] // tenant-scoped audit helper for upcoming handlers
pub async fn record_tenant_audit(
    state: &AppState,
    ctx: &AuditRequestContext,
    tenant_id: TenantId,
    actor_id: Uuid,
    action: &str,
    resource: &str,
    resource_id: Uuid,
    metadata: Option<serde_json::Value>,
) -> Result<(), ApiError> {
    infra_postgres::audit::insert_audit_event(
        &state.app_pool,
        tenant_id,
        NewAuditEvent {
            id: Uuid::now_v7(),
            actor_id,
            actor_type: ActorType::User,
            action: action.to_owned(),
            resource_type: resource.to_owned(),
            resource_id,
            metadata,
            correlation_id: ctx.correlation_id,
            ip: Some(ctx.ip.clone()),
        },
    )
    .await
    .map_err(|_| ApiError::internal())
}

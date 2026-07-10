use uuid::Uuid;

use crate::state::AppState;

/// ponytail: full audit.events wiring lands in Phase 10 — stub logs only for now.
pub async fn record_platform_audit_stub(
    _state: &AppState,
    actor_id: Uuid,
    action: &str,
    tenant_id: Option<Uuid>,
) {
    tracing::info!(
        actor_id = %actor_id,
        action = action,
        tenant_id = ?tenant_id,
        "platform audit stub"
    );
}

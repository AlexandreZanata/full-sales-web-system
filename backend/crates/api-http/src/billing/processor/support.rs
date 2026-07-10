use domain_platform::Tenant;

pub(crate) fn row_to_tenant(row: &infra_postgres::shared::TenantLifecycleRow) -> Tenant {
    Tenant {
        id: row.id,
        legal_name: row.legal_name.clone(),
        display_name: row.display_name.clone(),
        status: row.status,
        plan_id: row.plan_id,
        trial_ends_at: row.trial_ends_at,
        suspended_at: row.suspended_at,
        suspended_reason: row.suspended_reason.clone(),
        offboarding_scheduled_at: row.offboarding_scheduled_at,
        settings: row.settings.clone(),
    }
}

use chrono::{Duration, Utc};
use domain_domains::{DomainStatus, TenantDomain, txt_record_name};
use domain_shared::TenantId;
use infra_postgres::domains::DomainRow;
use uuid::Uuid;

use crate::error::ApiError;
use crate::state::AppState;

#[derive(Clone, Copy)]
pub struct HostTenant(pub TenantId);

pub fn row_to_domain(row: &DomainRow) -> TenantDomain {
    TenantDomain {
        id: row.id,
        tenant_id: row.tenant_id,
        hostname: row.hostname.clone(),
        status: DomainStatus::parse(&row.status).unwrap_or(DomainStatus::Failed),
        verification_token: row.verification_token.clone(),
        verified_at: row.verified_at,
        is_primary: row.is_primary,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }
}

pub async fn ensure_custom_domain_plan(
    state: &AppState,
    tenant_id: TenantId,
) -> Result<(), ApiError> {
    let flags = crate::platform::feature_support::load_resolved_flags(state, tenant_id).await?;
    if !flags.custom_domain {
        return Err(ApiError::forbidden_with_code(
            "PLAN_FEATURE_UNAVAILABLE",
            "Custom domain requires Pro or Enterprise plan",
        ));
    }
    Ok(())
}

pub fn reserved_platform_hosts() -> Vec<String> {
    let mut hosts = vec!["localhost".into(), "127.0.0.1".into()];
    if let Ok(raw) = std::env::var("PLATFORM_APEX_HOST") {
        for part in raw.split(',') {
            let host = part.trim().to_lowercase();
            if !host.is_empty() {
                hosts.push(host);
            }
        }
    }
    hosts
}

pub fn verification_token() -> String {
    Uuid::now_v7().simple().to_string()
}

pub fn challenge_expires_at() -> chrono::DateTime<Utc> {
    Utc::now() + Duration::hours(72)
}

pub async fn persist_domain(
    pool: &infra_postgres::PgPool,
    domain: &TenantDomain,
    bypass_rls: bool,
) -> Result<(), ApiError> {
    if bypass_rls {
        infra_postgres::domains::update_tenant_domain_admin(
            pool,
            domain.id,
            domain.status.as_str(),
            domain.verified_at,
            domain.is_primary,
        )
        .await
        .map_err(|_| ApiError::internal())
    } else {
        infra_postgres::domains::update_tenant_domain(
            pool,
            domain.tenant_id,
            domain.id,
            domain.status.as_str(),
            domain.verified_at,
            domain.is_primary,
        )
        .await
        .map_err(|_| ApiError::internal())
    }
}

pub fn txt_challenge(domain: &TenantDomain) -> (String, String) {
    (
        txt_record_name(&domain.hostname),
        domain.verification_token.clone(),
    )
}

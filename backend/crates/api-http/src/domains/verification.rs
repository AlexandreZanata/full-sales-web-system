use chrono::Utc;
use domain_domains::txt_record_name;

use crate::error::ApiError;
use crate::state::AppState;

use super::support::{persist_domain, row_to_domain};

pub async fn run_domain_verification_job(state: &AppState) -> Result<serde_json::Value, ApiError> {
    let rows = infra_postgres::domains::list_verifying_domains(&state.admin_pool)
        .await
        .map_err(|_| ApiError::internal())?;
    let mut verified = Vec::new();
    let mut failed = Vec::new();

    for row in rows {
        let mut domain = row_to_domain(&row);
        let challenge = infra_postgres::domains::find_active_challenge(&state.admin_pool, domain.id)
            .await
            .map_err(|_| ApiError::internal())?;
        let Some(challenge) = challenge else {
            domain.mark_failed(Utc::now()).map_err(|_| ApiError::internal())?;
            persist_domain(&state.admin_pool, &domain, true).await?;
            failed.push(domain.id);
            continue;
        };

        let record = txt_record_name(&domain.hostname);
        let values = state
            .dns_resolver
            .lookup_txt(&record)
            .await
            .unwrap_or_default();
        if values.iter().any(|v| v.trim() == challenge.token) {
            domain.mark_verified(Utc::now()).map_err(|_| ApiError::internal())?;
            domain.activate(Utc::now()).map_err(|_| ApiError::internal())?;
            persist_domain(&state.admin_pool, &domain, true).await?;
            verified.push(domain.id);
            continue;
        }

        if challenge.expires_at <= Utc::now() {
            domain.mark_failed(Utc::now()).map_err(|_| ApiError::internal())?;
            persist_domain(&state.admin_pool, &domain, true).await?;
            infra_postgres::domains::expire_challenges(&state.admin_pool, domain.id)
                .await
                .map_err(|_| ApiError::internal())?;
            failed.push(domain.id);
        }
    }

    Ok(serde_json::json!({ "verified": verified, "failed": failed }))
}

pub async fn force_verify_domain(state: &AppState, domain_id: uuid::Uuid) -> Result<(), ApiError> {
    let row = infra_postgres::domains::find_domain_by_id_admin(&state.admin_pool, domain_id)
        .await
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;
    let mut domain = row_to_domain(&row);
    let now = Utc::now();
    match domain.status {
        domain_domains::DomainStatus::Pending => {
            domain.start_verifying(now).map_err(map_domain_err)?;
        }
        domain_domains::DomainStatus::Failed => {
            domain.retry_verification(now).map_err(map_domain_err)?;
        }
        domain_domains::DomainStatus::Detached => {
            return Err(ApiError::bad_request("INVALID_TRANSITION", "Domain is detached"));
        }
        _ => {}
    }
    domain.mark_verified(now).map_err(map_domain_err)?;
    domain.activate(now).map_err(map_domain_err)?;
    persist_domain(&state.admin_pool, &domain, true).await
}

fn map_domain_err(err: domain_domains::DomainError) -> ApiError {
    match err {
        domain_domains::DomainError::InvalidTransition { .. } => {
            ApiError::bad_request("INVALID_TRANSITION", "Invalid domain status transition")
        }
        _ => ApiError::bad_request("INVALID_TRANSITION", "Invalid domain operation"),
    }
}

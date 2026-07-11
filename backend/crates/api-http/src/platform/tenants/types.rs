use chrono::{DateTime, Utc};
use domain_platform::{Tenant, TenantStatus};
use domain_shared::TenantId;
use infra_postgres::shared::TenantLifecycleRow;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct TenantListItem {
    pub id: Uuid,
    #[serde(rename = "legalName")]
    pub legal_name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub status: String,
    #[serde(rename = "planId", skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<Uuid>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct TenantDetailResponse {
    pub id: Uuid,
    #[serde(rename = "legalName")]
    pub legal_name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub status: String,
    #[serde(rename = "planId", skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<Uuid>,
    #[serde(rename = "trialEndsAt", skip_serializing_if = "Option::is_none")]
    pub trial_ends_at: Option<DateTime<Utc>>,
    #[serde(rename = "suspendedAt", skip_serializing_if = "Option::is_none")]
    pub suspended_at: Option<DateTime<Utc>>,
    #[serde(rename = "suspendedReason", skip_serializing_if = "Option::is_none")]
    pub suspended_reason: Option<String>,
    pub counts: TenantCountsResponse,
    pub settings: serde_json::Value,
}

#[derive(Serialize)]
pub struct TenantCountsResponse {
    pub users: i64,
    pub commerces: i64,
    pub orders: i64,
}

#[derive(Serialize)]
pub struct ProvisionTenantResponse {
    #[serde(rename = "tenantId")]
    pub tenant_id: Uuid,
    #[serde(rename = "adminUserId")]
    pub admin_user_id: Uuid,
    pub status: String,
    #[serde(rename = "trialEndsAt", skip_serializing_if = "Option::is_none")]
    pub trial_ends_at: Option<DateTime<Utc>>,
    #[serde(rename = "adminTemporaryPassword")]
    pub admin_temporary_password: String,
}

pub fn row_to_tenant(row: &TenantLifecycleRow) -> Tenant {
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

pub fn tenant_detail(
    row: TenantLifecycleRow,
    counts: infra_postgres::shared::TenantCounts,
) -> TenantDetailResponse {
    TenantDetailResponse {
        id: row.id.as_uuid(),
        legal_name: row.legal_name,
        display_name: row.display_name,
        status: row.status.as_str().to_owned(),
        plan_id: row.plan_id,
        trial_ends_at: row.trial_ends_at,
        suspended_at: row.suspended_at,
        suspended_reason: row.suspended_reason,
        counts: TenantCountsResponse {
            users: counts.users,
            commerces: counts.commerces,
            orders: counts.orders,
        },
        settings: row.settings,
    }
}

pub fn map_platform_patch_error(err: application::AppError) -> crate::error::ApiError {
    match err {
        application::AppError::Platform(
            domain_platform::PlatformError::InvalidTenantTransition { .. },
        ) => crate::error::ApiError::bad_request(
            "INVALID_TENANT_TRANSITION",
            "Invalid tenant status transition",
        ),
        application::AppError::Platform(domain_platform::PlatformError::SuspendReasonRequired)
        | application::AppError::Platform(domain_platform::PlatformError::SuspendReasonTooShort) => {
            crate::error::ApiError::bad_request(
                "INVALID_INPUT",
                "Suspend reason required (min 3 chars)",
            )
        }
        application::AppError::Platform(_) => {
            crate::error::ApiError::bad_request("INVALID_INPUT", "Invalid tenant input")
        }
        other => match other {
            application::AppError::TenantSuspended => crate::error::ApiError::tenant_suspended(),
            _ => crate::error::ApiError::internal(),
        },
    }
}

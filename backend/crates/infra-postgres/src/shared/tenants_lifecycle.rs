use chrono::{DateTime, Utc};
use domain_platform::TenantStatus;
use domain_shared::TenantId;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_bypass_rls;

#[derive(Debug, Clone)]
pub struct TenantLifecycleRow {
    pub id: TenantId,
    pub legal_name: String,
    pub display_name: String,
    pub status: TenantStatus,
    pub plan_id: Option<Uuid>,
    pub trial_ends_at: Option<DateTime<Utc>>,
    pub suspended_at: Option<DateTime<Utc>>,
    pub suspended_reason: Option<String>,
    pub offboarding_scheduled_at: Option<DateTime<Utc>>,
    pub settings: serde_json::Value,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct TenantListRow {
    pub id: TenantId,
    pub legal_name: String,
    pub display_name: String,
    pub status: String,
    pub plan_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default)]
pub struct TenantCounts {
    pub users: i64,
    pub commerces: i64,
    pub orders: i64,
}

pub struct ProvisionTenantParams<'a> {
    pub id: TenantId,
    pub legal_name: &'a str,
    pub display_name: &'a str,
    pub status: TenantStatus,
    pub plan_id: Option<Uuid>,
    pub trial_ends_at: Option<DateTime<Utc>>,
    pub settings: serde_json::Value,
    pub admin_user_id: Uuid,
    pub admin_email: &'a str,
    pub admin_name: &'a str,
    pub admin_password_hash: &'a str,
}

pub async fn find_tenant_lifecycle(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Option<TenantLifecycleRow>, PostgresError> {
    let row = sqlx::query_as::<_, LifecycleRecord>(
        "SELECT id, legal_name, display_name, status, plan_id, trial_ends_at,
                suspended_at, suspended_reason, offboarding_scheduled_at, settings, active
         FROM shared.tenants WHERE id = $1",
    )
    .bind(tenant_id.as_uuid())
    .fetch_optional(pool)
    .await?;
    Ok(row.map(TenantLifecycleRow::from))
}

pub async fn find_tenant_status(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<Option<TenantStatus>, PostgresError> {
    let status = sqlx::query_scalar::<_, String>("SELECT status FROM shared.tenants WHERE id = $1")
        .bind(tenant_id.as_uuid())
        .fetch_optional(pool)
        .await?;
    match status {
        None => Ok(None),
        Some(raw) => TenantStatus::parse(&raw)
            .map(Some)
            .map_err(|_| PostgresError::from(sqlx::Error::Decode("invalid tenant status".into()))),
    }
}

pub async fn list_tenants_platform(
    pool: &PgPool,
    status: Option<&str>,
    plan_id: Option<Uuid>,
    after_id: Option<Uuid>,
    limit: i64,
) -> Result<Vec<TenantListRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    let rows = sqlx::query_as::<_, ListRecord>(
        "SELECT id, legal_name, display_name, status, plan_id, created_at
         FROM shared.tenants
         WHERE ($1::text IS NULL OR status = $1)
           AND ($2::uuid IS NULL OR plan_id = $2)
           AND ($3::uuid IS NULL OR id > $3)
         ORDER BY id ASC
         LIMIT $4",
    )
    .bind(status)
    .bind(plan_id)
    .bind(after_id)
    .bind(limit)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows.into_iter().map(TenantListRow::from).collect())
}

pub async fn provision_tenant(
    admin_pool: &PgPool,
    app_pool: &PgPool,
    params: ProvisionTenantParams<'_>,
) -> Result<(), PostgresError> {
    let mut admin_tx = admin_pool.begin().await?;
    insert_tenant_row(&mut admin_tx, &params).await?;
    admin_tx.commit().await?;

    let mut app_tx = app_pool.begin().await?;
    crate::rls::apply_tenant_context(&mut app_tx, params.id).await?;
    sqlx::query(
        "INSERT INTO identity.users
         (id, tenant_id, email, name, role, password_hash)
         VALUES ($1, $2, $3, $4, 'Admin', $5)",
    )
    .bind(params.admin_user_id)
    .bind(params.id.as_uuid())
    .bind(params.admin_email)
    .bind(params.admin_name)
    .bind(params.admin_password_hash)
    .execute(&mut *app_tx)
    .await?;
    app_tx.commit().await?;
    Ok(())
}

async fn insert_tenant_row(
    tx: &mut Transaction<'_, Postgres>,
    params: &ProvisionTenantParams<'_>,
) -> Result<(), PostgresError> {
    let active = matches!(
        params.status,
        TenantStatus::Trial | TenantStatus::Active | TenantStatus::PastDue
    );
    sqlx::query(
        "INSERT INTO shared.tenants
         (id, name, legal_name, display_name, status, plan_id, trial_ends_at, settings, active)
         VALUES ($1, $2, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(params.id.as_uuid())
    .bind(params.legal_name)
    .bind(params.display_name)
    .bind(params.status.as_str())
    .bind(params.plan_id)
    .bind(params.trial_ends_at)
    .bind(&params.settings)
    .bind(active)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn update_tenant_lifecycle(
    pool: &PgPool,
    tenant_id: TenantId,
    status: TenantStatus,
    plan_id: Option<Uuid>,
    trial_ends_at: Option<DateTime<Utc>>,
    suspended_at: Option<DateTime<Utc>>,
    suspended_reason: Option<&str>,
    offboarding_scheduled_at: Option<DateTime<Utc>>,
    display_name: Option<&str>,
    settings: Option<serde_json::Value>,
) -> Result<bool, PostgresError> {
    let active = matches!(
        status,
        TenantStatus::Trial | TenantStatus::Active | TenantStatus::PastDue
    );
    let result = sqlx::query(
        "UPDATE shared.tenants SET
            status = $2,
            plan_id = COALESCE($3, plan_id),
            trial_ends_at = $4,
            suspended_at = $5,
            suspended_reason = $6,
            offboarding_scheduled_at = $7,
            display_name = COALESCE($8, display_name),
            settings = COALESCE($9, settings),
            active = $10,
            updated_at = now()
         WHERE id = $1",
    )
    .bind(tenant_id.as_uuid())
    .bind(status.as_str())
    .bind(plan_id)
    .bind(trial_ends_at)
    .bind(suspended_at)
    .bind(suspended_reason)
    .bind(offboarding_scheduled_at)
    .bind(display_name)
    .bind(settings)
    .bind(active)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn tenant_counts(
    pool: &PgPool,
    tenant_id: TenantId,
) -> Result<TenantCounts, PostgresError> {
    let mut tx = pool.begin().await?;
    crate::rls::apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query("SELECT set_config('app.role', 'Admin', true)")
        .execute(&mut *tx)
        .await?;
    let users =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM identity.users WHERE tenant_id = $1")
            .bind(tenant_id.as_uuid())
            .fetch_one(&mut *tx)
            .await?;
    let commerces = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM commerces.commerces WHERE tenant_id = $1",
    )
    .bind(tenant_id.as_uuid())
    .fetch_one(&mut *tx)
    .await?;
    let orders =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM orders.orders WHERE tenant_id = $1")
            .bind(tenant_id.as_uuid())
            .fetch_one(&mut *tx)
            .await?;
    tx.commit().await?;
    Ok(TenantCounts {
        users,
        commerces,
        orders,
    })
}

pub async fn plan_exists(pool: &PgPool, plan_id: Uuid) -> Result<bool, PostgresError> {
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM billing.plans WHERE id = $1 AND active = true)",
    )
    .bind(plan_id)
    .fetch_one(pool)
    .await?;
    Ok(exists)
}

pub async fn mark_tenant_past_due(
    pool: &PgPool,
    tenant_id: TenantId,
    status: TenantStatus,
) -> Result<bool, PostgresError> {
    let result = sqlx::query(
        "UPDATE shared.tenants SET
            status = $2,
            past_due_at = COALESCE(past_due_at, now()),
            active = true,
            updated_at = now()
         WHERE id = $1",
    )
    .bind(tenant_id.as_uuid())
    .bind(status.as_str())
    .execute(pool)
    .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn mark_tenant_payment_cleared(
    pool: &PgPool,
    tenant_id: TenantId,
    status: TenantStatus,
) -> Result<bool, PostgresError> {
    let result = sqlx::query(
        "UPDATE shared.tenants SET
            status = $2,
            past_due_at = NULL,
            trial_ends_at = NULL,
            active = true,
            updated_at = now()
         WHERE id = $1",
    )
    .bind(tenant_id.as_uuid())
    .bind(status.as_str())
    .execute(pool)
    .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn set_grace_extended_until(
    pool: &PgPool,
    tenant_id: TenantId,
    until: DateTime<Utc>,
) -> Result<bool, PostgresError> {
    let result = sqlx::query(
        "UPDATE shared.tenants SET grace_extended_until = $2, updated_at = now() WHERE id = $1",
    )
    .bind(tenant_id.as_uuid())
    .bind(until)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn find_dunning_candidates(pool: &PgPool) -> Result<Vec<TenantId>, PostgresError> {
    let ids = sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM shared.tenants
         WHERE status = 'PastDue'
           AND past_due_at IS NOT NULL
           AND now() >= COALESCE(grace_extended_until, past_due_at + interval '7 days')",
    )
    .fetch_all(pool)
    .await?;
    Ok(ids.into_iter().map(TenantId::from_uuid).collect())
}

/// Test/admin helper — backdate past-due timestamp for dunning contract tests.
pub async fn backdate_past_due_at(
    pool: &PgPool,
    tenant_id: TenantId,
    at: DateTime<Utc>,
) -> Result<(), PostgresError> {
    sqlx::query("UPDATE shared.tenants SET past_due_at = $2 WHERE id = $1")
        .bind(tenant_id.as_uuid())
        .bind(at)
        .execute(pool)
        .await?;
    Ok(())
}

#[derive(sqlx::FromRow)]
struct LifecycleRecord {
    id: Uuid,
    legal_name: String,
    display_name: String,
    status: String,
    plan_id: Option<Uuid>,
    trial_ends_at: Option<DateTime<Utc>>,
    suspended_at: Option<DateTime<Utc>>,
    suspended_reason: Option<String>,
    offboarding_scheduled_at: Option<DateTime<Utc>>,
    settings: serde_json::Value,
    active: bool,
}

impl From<LifecycleRecord> for TenantLifecycleRow {
    fn from(r: LifecycleRecord) -> Self {
        Self {
            id: TenantId::from_uuid(r.id),
            legal_name: r.legal_name,
            display_name: r.display_name,
            status: TenantStatus::parse(&r.status).unwrap_or(TenantStatus::Active),
            plan_id: r.plan_id,
            trial_ends_at: r.trial_ends_at,
            suspended_at: r.suspended_at,
            suspended_reason: r.suspended_reason,
            offboarding_scheduled_at: r.offboarding_scheduled_at,
            settings: r.settings,
            active: r.active,
        }
    }
}

#[derive(sqlx::FromRow)]
struct ListRecord {
    id: Uuid,
    legal_name: String,
    display_name: String,
    status: String,
    plan_id: Option<Uuid>,
    created_at: DateTime<Utc>,
}

impl From<ListRecord> for TenantListRow {
    fn from(r: ListRecord) -> Self {
        Self {
            id: TenantId::from_uuid(r.id),
            legal_name: r.legal_name,
            display_name: r.display_name,
            status: r.status,
            plan_id: r.plan_id,
            created_at: r.created_at,
        }
    }
}

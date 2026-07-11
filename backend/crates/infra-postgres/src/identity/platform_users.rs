use chrono::{DateTime, Utc};
use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_bypass_rls;

#[derive(Debug, Clone)]
pub struct CrossTenantUserFilters<'a> {
    pub tenant_id: Option<Uuid>,
    pub role: Option<&'a str>,
    pub active: Option<bool>,
    pub email_prefix: Option<&'a str>,
    pub sort: UserSort,
    pub after_id: Option<Uuid>,
    pub limit: i64,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum UserSort {
    #[default]
    CreatedAt,
    Email,
    Name,
}

#[derive(Debug, Clone)]
pub struct CrossTenantUserDetailRow {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub name: String,
    pub role: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub tenant_display_name: String,
}

pub async fn list_users_cross_tenant(
    pool: &PgPool,
    filters: CrossTenantUserFilters<'_>,
) -> Result<Vec<CrossTenantUserDetailRow>, PostgresError> {
    let order = sort_clause(filters.sort);
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    let query = format!(
        "SELECT u.id, u.tenant_id, u.email, u.name, u.role, u.active, u.created_at, u.last_login_at,
                t.display_name AS tenant_display_name
         FROM identity.users u
         JOIN shared.tenants t ON t.id = u.tenant_id
         WHERE ($1::uuid IS NULL OR u.tenant_id = $1)
           AND ($2::text IS NULL OR u.role = $2)
           AND ($3::bool IS NULL OR u.active = $3)
           AND ($4::text IS NULL OR lower(u.email) LIKE lower($4) || '%')
           AND ($5::uuid IS NULL OR u.id > $5)
         ORDER BY {order}, u.id ASC
         LIMIT $6"
    );
    let rows = sqlx::query_as::<_, CrossTenantUserDetailRecord>(&query)
        .bind(filters.tenant_id)
        .bind(filters.role)
        .bind(filters.active)
        .bind(filters.email_prefix)
        .bind(filters.after_id)
        .bind(filters.limit)
        .fetch_all(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(CrossTenantUserDetailRow::from)
        .collect())
}

pub async fn find_user_cross_tenant(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<CrossTenantUserDetailRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    let row = sqlx::query_as::<_, CrossTenantUserDetailRecord>(
        "SELECT u.id, u.tenant_id, u.email, u.name, u.role, u.active, u.created_at, u.last_login_at,
                t.display_name AS tenant_display_name
         FROM identity.users u
         JOIN shared.tenants t ON t.id = u.tenant_id
         WHERE u.id = $1",
    )
    .bind(user_id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(CrossTenantUserDetailRow::from))
}

pub async fn set_user_active(
    pool: &PgPool,
    user_id: Uuid,
    active: bool,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    let result =
        sqlx::query("UPDATE identity.users SET active = $2, updated_at = now() WHERE id = $1")
            .bind(user_id)
            .bind(active)
            .execute(&mut *tx)
            .await?;
    tx.commit().await?;
    Ok(result.rows_affected() == 1)
}

pub async fn update_user_role(
    pool: &PgPool,
    user_id: Uuid,
    role: &str,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    let result =
        sqlx::query("UPDATE identity.users SET role = $2, updated_at = now() WHERE id = $1")
            .bind(user_id)
            .bind(role)
            .execute(&mut *tx)
            .await?;
    tx.commit().await?;
    Ok(result.rows_affected() == 1)
}

pub async fn update_user_password(
    pool: &PgPool,
    user_id: Uuid,
    password_hash: &str,
) -> Result<bool, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    let result = sqlx::query(
        "UPDATE identity.users SET password_hash = $2, updated_at = now() WHERE id = $1",
    )
    .bind(user_id)
    .bind(password_hash)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(result.rows_affected() == 1)
}

pub async fn count_active_admins(
    pool: &PgPool,
    tenant_id: TenantId,
    except_user_id: Option<Uuid>,
) -> Result<i64, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_bypass_rls(&mut tx).await?;
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM identity.users
         WHERE tenant_id = $1 AND role = 'Admin' AND active = true
           AND ($2::uuid IS NULL OR id <> $2)",
    )
    .bind(tenant_id.as_uuid())
    .bind(except_user_id)
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(count)
}

pub async fn touch_last_login(pool: &PgPool, user_id: Uuid) -> Result<(), PostgresError> {
    sqlx::query("UPDATE identity.users SET last_login_at = now() WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn list_tenant_users(
    pool: &PgPool,
    tenant_id: TenantId,
    after_id: Option<Uuid>,
    limit: i64,
) -> Result<Vec<CrossTenantUserDetailRow>, PostgresError> {
    list_users_cross_tenant(
        pool,
        CrossTenantUserFilters {
            tenant_id: Some(tenant_id.as_uuid()),
            role: None,
            active: None,
            email_prefix: None,
            sort: UserSort::CreatedAt,
            after_id,
            limit,
        },
    )
    .await
}

fn sort_clause(sort: UserSort) -> &'static str {
    match sort {
        UserSort::CreatedAt => "u.created_at DESC",
        UserSort::Email => "u.email ASC",
        UserSort::Name => "u.name ASC",
    }
}

#[derive(sqlx::FromRow)]
pub struct CrossTenantUserDetailRecord {
    id: Uuid,
    tenant_id: Uuid,
    email: String,
    name: String,
    role: String,
    active: bool,
    created_at: DateTime<Utc>,
    last_login_at: Option<DateTime<Utc>>,
    tenant_display_name: String,
}

impl From<CrossTenantUserDetailRecord> for CrossTenantUserDetailRow {
    fn from(row: CrossTenantUserDetailRecord) -> Self {
        Self {
            id: row.id,
            tenant_id: row.tenant_id,
            email: row.email,
            name: row.name,
            role: row.role,
            active: row.active,
            created_at: row.created_at,
            last_login_at: row.last_login_at,
            tenant_display_name: row.tenant_display_name,
        }
    }
}

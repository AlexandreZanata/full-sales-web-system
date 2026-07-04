use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;
use crate::rls::apply_tenant_context;

pub mod driver_profiles;
pub mod seller_profiles;

pub use driver_profiles::{find_driver_profile_by_user_id, insert_driver_profile, DriverProfileInsert, DriverProfileRow};
pub use seller_profiles::{insert_seller_profile, SellerProfileInsert};

/// Row persisted in `identity.users`.
#[derive(Debug, Clone)]
pub struct UserRow {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub email: String,
}

pub struct InsertUserParams<'a> {
    pub id: Uuid,
    pub email: &'a str,
    pub name: &'a str,
    pub role: &'a str,
    pub password_hash: &'a str,
    pub commerce_id: Option<Uuid>,
    pub profile_file_id: Option<Uuid>,
}

/// Inserts a user under the current RLS tenant context.
pub async fn insert_user(
    pool: &PgPool,
    tenant_id: TenantId,
    params: InsertUserParams<'_>,
) -> Result<(), PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    sqlx::query(
        "INSERT INTO identity.users
         (id, tenant_id, email, name, role, password_hash, commerce_id, profile_file_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(params.id)
    .bind(tenant_id.as_uuid())
    .bind(params.email)
    .bind(params.name)
    .bind(params.role)
    .bind(params.password_hash)
    .bind(params.commerce_id)
    .bind(params.profile_file_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

/// Deactivates a user (admin connection — test/support helper).
pub async fn deactivate_user(pool: &PgPool, user_id: Uuid) -> Result<(), PostgresError> {
    sqlx::query("UPDATE identity.users SET active = false WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Lists user ids visible under the current tenant RLS context.
pub async fn list_user_ids(pool: &PgPool, tenant_id: TenantId) -> Result<Vec<Uuid>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_scalar::<_, Uuid>("SELECT id FROM identity.users ORDER BY email")
        .fetch_all(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(rows)
}

/// Finds login credentials by email (admin connection — pre-tenant lookup).
pub async fn find_user_for_login(
    pool: &PgPool,
    email: &str,
) -> Result<Option<LoginUserRecord>, PostgresError> {
    let row = sqlx::query_as::<_, LoginRecord>(
        "SELECT id, tenant_id, role, password_hash, active, commerce_id
         FROM identity.users WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(LoginUserRecord::from))
}

/// Finds login credentials by user id (admin connection).
pub async fn find_login_record_by_id(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<LoginUserRecord>, PostgresError> {
    let row = sqlx::query_as::<_, LoginRecord>(
        "SELECT id, tenant_id, role, password_hash, active, commerce_id
         FROM identity.users WHERE id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(LoginUserRecord::from))
}

/// Login credential row for authentication use cases.
#[derive(Debug, Clone)]
pub struct LoginUserRecord {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub role: String,
    pub password_hash: String,
    pub active: bool,
    pub commerce_id: Option<Uuid>,
}

#[derive(sqlx::FromRow)]
struct LoginRecord {
    id: Uuid,
    tenant_id: Uuid,
    role: String,
    password_hash: String,
    active: bool,
    commerce_id: Option<Uuid>,
}

impl From<LoginRecord> for LoginUserRecord {
    fn from(r: LoginRecord) -> Self {
        Self {
            id: r.id,
            tenant_id: r.tenant_id,
            role: r.role,
            password_hash: r.password_hash,
            active: r.active,
            commerce_id: r.commerce_id,
        }
    }
}

/// Finds a user by id within tenant RLS scope.
pub async fn find_user_by_id(
    pool: &PgPool,
    tenant_id: TenantId,
    user_id: Uuid,
) -> Result<Option<UserRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let row = sqlx::query_as::<_, UserRecord>(
        "SELECT id, tenant_id, email FROM identity.users WHERE id = $1",
    )
    .bind(user_id)
    .fetch_optional(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(row.map(|r| UserRow {
        id: r.id,
        tenant_id: TenantId::from_uuid(r.tenant_id),
        email: r.email,
    }))
}

pub struct UserListRow {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: String,
    pub active: bool,
}

pub async fn list_users(
    pool: &PgPool,
    tenant_id: TenantId,
    limit: i64,
    offset: i64,
) -> Result<Vec<UserListRow>, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let rows = sqlx::query_as::<_, (Uuid, String, String, String, bool)>(
        "SELECT id, email, name, role, active
         FROM identity.users ORDER BY email LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(|(id, email, name, role, active)| UserListRow {
            id,
            email,
            name,
            role,
            active,
        })
        .collect())
}

pub async fn count_users(pool: &PgPool, tenant_id: TenantId) -> Result<i64, PostgresError> {
    let mut tx = pool.begin().await?;
    apply_tenant_context(&mut tx, tenant_id).await?;
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM identity.users")
        .fetch_one(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(count)
}

#[derive(sqlx::FromRow)]
struct UserRecord {
    id: Uuid,
    tenant_id: Uuid,
    email: String,
}

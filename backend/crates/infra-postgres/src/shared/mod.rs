use domain_shared::TenantId;
use sqlx::PgPool;
use uuid::Uuid;

use crate::PostgresError;

/// Row persisted in `shared.tenants`.
#[derive(Debug, Clone)]
pub struct TenantRow {
    pub id: TenantId,
    pub name: String,
    pub display_name: String,
    pub logo_file_id: Option<Uuid>,
    pub sales_contact_phone: Option<String>,
    pub active: bool,
}

/// Inserts a tenant (admin/migration path — no RLS on `shared.tenants`).
pub async fn insert_tenant(pool: &PgPool, id: TenantId, name: &str) -> Result<(), PostgresError> {
    sqlx::query(
        "INSERT INTO shared.tenants (id, name, display_name) VALUES ($1, $2, $2)",
    )
    .bind(id.as_uuid())
    .bind(name)
    .execute(pool)
    .await?;
    Ok(())
}

/// Finds a tenant by id.
pub async fn find_tenant_by_id(
    pool: &PgPool,
    id: TenantId,
) -> Result<Option<TenantRow>, PostgresError> {
    let row = sqlx::query_as::<_, TenantRecord>(
        "SELECT id, name, display_name, logo_file_id, sales_contact_phone, active FROM shared.tenants WHERE id = $1",
    )
    .bind(id.as_uuid())
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| TenantRow {
        id: TenantId::from_uuid(r.id),
        name: r.name,
        display_name: r.display_name,
        logo_file_id: r.logo_file_id,
        sales_contact_phone: r.sales_contact_phone,
        active: r.active,
    }))
}

pub async fn update_tenant_display_name(
    pool: &PgPool,
    tenant_id: TenantId,
    display_name: &str,
) -> Result<bool, PostgresError> {
    let result = sqlx::query(
        "UPDATE shared.tenants SET display_name = $1 WHERE id = $2",
    )
    .bind(display_name)
    .bind(tenant_id.as_uuid())
    .execute(pool)
    .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn update_tenant_logo(
    pool: &PgPool,
    tenant_id: TenantId,
    logo_file_id: Option<Uuid>,
) -> Result<bool, PostgresError> {
    let result = sqlx::query("UPDATE shared.tenants SET logo_file_id = $1 WHERE id = $2")
        .bind(logo_file_id)
        .bind(tenant_id.as_uuid())
        .execute(pool)
        .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn update_tenant_sales_contact_phone(
    pool: &PgPool,
    tenant_id: TenantId,
    sales_contact_phone: Option<&str>,
) -> Result<bool, PostgresError> {
    let result = sqlx::query(
        "UPDATE shared.tenants SET sales_contact_phone = $1 WHERE id = $2",
    )
    .bind(sales_contact_phone)
    .bind(tenant_id.as_uuid())
    .execute(pool)
    .await?;
    Ok(result.rows_affected() == 1)
}

#[derive(sqlx::FromRow)]
struct TenantRecord {
    id: Uuid,
    name: String,
    display_name: String,
    logo_file_id: Option<Uuid>,
    sales_contact_phone: Option<String>,
    active: bool,
}
